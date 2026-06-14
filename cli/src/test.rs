//! Test runner: parses `# @test:...` directives, executes the script via bash,
//! and evaluates each assertion.

use anyhow::{anyhow, Context, Result};
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Hard wall-clock limit (seconds) for a single exercise script. Protects
/// against infinite loops (`while true; do :; done`) hanging the CLI forever.
pub const SCRIPT_TIMEOUT_SECS: u64 = 10;
const SCRIPT_TIMEOUT: Duration = Duration::from_secs(SCRIPT_TIMEOUT_SECS);

/// A single test assertion declared at the bottom of an exercise file.
#[derive(Debug, Clone)]
pub enum Assertion {
    /// Script's stdout must equal this literal string (after trailing-newline trim).
    Stdout(String),
    /// Script's stdout must equal the stdout of running this command via `bash -c`.
    StdoutCmd(String),
    /// Script's stdout must contain this substring.
    StdoutContains(String),
    /// Script's stdout must match this regular expression.
    StdoutRegex(String),
    /// Script's stderr must equal this literal string (after trailing-newline trim).
    Stderr(String),
    /// Script's exit code must equal this number.
    Exit(i32),
    /// The given path must exist on disk after the script runs.
    FileExists(String),
}

impl Assertion {
    pub fn kind(&self) -> &'static str {
        match self {
            Assertion::Stdout(_) => "stdout",
            Assertion::StdoutCmd(_) => "stdout-cmd",
            Assertion::StdoutContains(_) => "stdout-contains",
            Assertion::StdoutRegex(_) => "stdout-regex",
            Assertion::Stderr(_) => "stderr",
            Assertion::Exit(_) => "exit",
            Assertion::FileExists(_) => "file-exists",
        }
    }

    pub fn preview(&self) -> String {
        match self {
            Assertion::Stdout(v) => format!("{v:?}"),
            Assertion::StdoutCmd(c) => format!("`{c}`"),
            Assertion::StdoutContains(v) => format!("⊇ {v:?}"),
            Assertion::StdoutRegex(p) => format!("/{p}/"),
            Assertion::Stderr(v) => format!("{v:?}"),
            Assertion::Exit(c) => c.to_string(),
            Assertion::FileExists(p) => p.clone(),
        }
    }
}

/// Outcome for a single assertion after running the script.
#[derive(Debug)]
pub struct AssertionResult {
    pub assertion: Assertion,
    pub passed: bool,
    pub expected: String,
    pub actual: String,
}

/// Raw result of executing one script via bash.
#[derive(Debug)]
pub struct ScriptOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub timed_out: bool,
}

/// Aggregate report for one exercise run.
#[derive(Debug)]
pub struct TestReport {
    pub results: Vec<AssertionResult>,
    #[allow(dead_code)]
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub timed_out: bool,
}

impl TestReport {
    pub fn all_passed(&self) -> bool {
        !self.results.is_empty() && self.results.iter().all(|r| r.passed)
    }

    pub fn passed_count(&self) -> usize {
        self.results.iter().filter(|r| r.passed).count()
    }

    pub fn total(&self) -> usize {
        self.results.len()
    }
}

/// Parse `# @test:KIND: VALUE` directives from a script source.
///
/// Recognised kinds:
///   - `stdout`          — exact string match
///   - `stdout-cmd`      — run `bash -c <value>`, compare its stdout
///   - `stdout-contains` — stdout must contain the substring
///   - `stdout-regex`    — stdout must match the regular expression
///   - `stderr`          — exact stderr match
///   - `exit`            — exact exit-code match (integer)
///   - `file-exists`     — the given path must exist after the run
///
/// Unknown directives produce a warning but don't fail parsing.
pub fn parse_assertions(content: &str) -> Result<Vec<Assertion>> {
    let mut out = Vec::new();
    for (idx, raw_line) in content.lines().enumerate() {
        let lineno = idx + 1;
        let line = raw_line.trim();
        if !line.starts_with('#') {
            continue;
        }
        let rest = line.trim_start_matches('#').trim_start();
        let Some(rest) = rest.strip_prefix("@test:") else {
            continue;
        };
        let Some((kind, value)) = rest.split_once(':') else {
            return Err(anyhow!(
                "{lineno}-qator: noto'g'ri '@test:' direktiva — '# @test:KIND: VALUE' kutilgan"
            ));
        };
        let kind = kind.trim();
        let value = value.trim();

        match kind {
            "stdout" => out.push(Assertion::Stdout(value.to_string())),
            "stdout-cmd" => out.push(Assertion::StdoutCmd(value.to_string())),
            "stdout-contains" => out.push(Assertion::StdoutContains(value.to_string())),
            "stdout-regex" => {
                // Fail fast on an invalid pattern (author error, not learner error).
                regex::Regex::new(value).with_context(|| {
                    format!("{lineno}-qator: '@test:stdout-regex' noto'g'ri regex: '{value}'")
                })?;
                out.push(Assertion::StdoutRegex(value.to_string()));
            }
            "stderr" => out.push(Assertion::Stderr(value.to_string())),
            "exit" => {
                let code: i32 = value.parse().with_context(|| {
                    format!("{lineno}-qator: '@test:exit' qiymati son emas: '{value}'")
                })?;
                out.push(Assertion::Exit(code));
            }
            "file-exists" => out.push(Assertion::FileExists(value.to_string())),
            other => {
                eprintln!("⚠  {lineno}-qator: noma'lum direktiva @test:{other} (e'tibordan chetda)");
            }
        }
    }
    Ok(out)
}

/// Trim trailing newlines (\n, \r) only — preserve all leading/internal whitespace.
fn normalize(s: &str) -> String {
    s.trim_end_matches(['\n', '\r']).to_string()
}

/// Execute the user's exercise script with `bash <path>`.
///
/// Hardened runner:
///   - runs in `cwd` (deterministic — independent of where `bashlings` was called)
///   - stdin is `/dev/null` (a `read` in the script can't block on the terminal)
///   - killed after [`SCRIPT_TIMEOUT`] (infinite loops can't hang the CLI)
pub fn run_script(path: &Path, cwd: &Path) -> Result<ScriptOutput> {
    let mut child = Command::new("bash")
        .arg(path)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("'bash {}' ni ishga tushira olmadik", path.display()))?;

    // Drain stdout/stderr on separate threads so a chatty script can't fill the
    // pipe buffer and deadlock while we poll for the timeout.
    let mut out_pipe = child.stdout.take().expect("stdout piped");
    let mut err_pipe = child.stderr.take().expect("stderr piped");
    let out_thread = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = out_pipe.read_to_end(&mut buf);
        buf
    });
    let err_thread = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = err_pipe.read_to_end(&mut buf);
        buf
    });

    let start = Instant::now();
    let mut timed_out = false;
    let status = loop {
        match child.try_wait().context("skript holatini tekshirib bo'lmadi")? {
            Some(status) => break status,
            None => {
                if start.elapsed() >= SCRIPT_TIMEOUT {
                    let _ = child.kill();
                    timed_out = true;
                    break child.wait().context("to'xtatilgan skriptni kutib bo'lmadi")?;
                }
                std::thread::sleep(Duration::from_millis(20));
            }
        }
    };

    let stdout = String::from_utf8_lossy(&out_thread.join().unwrap_or_default()).into_owned();
    let stderr = String::from_utf8_lossy(&err_thread.join().unwrap_or_default()).into_owned();
    let exit_code = if timed_out {
        124 // GNU `timeout` konventsiyasi
    } else {
        status.code().unwrap_or(-1)
    };

    Ok(ScriptOutput {
        stdout,
        stderr,
        exit_code,
        timed_out,
    })
}

/// Run a one-liner via `bash -c` (in `cwd`) and return its stdout.
fn run_expected_cmd(cmd: &str, cwd: &Path) -> Result<String> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .output()
        .with_context(|| format!("expected komandasini bajara olmadik: 'bash -c {cmd}'"))?;
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Detect bash major version (e.g. 5 from `5.3.9`). `None` if bash is missing
/// or the version can't be parsed.
pub fn bash_major_version() -> Option<u32> {
    let output = Command::new("bash")
        .arg("-c")
        .arg("echo \"${BASH_VERSINFO[0]}\"")
        .output()
        .ok()?;
    String::from_utf8_lossy(&output.stdout).trim().parse().ok()
}

/// Print a one-line warning to stderr when the active bash is older than 4
/// (macOS ships 3.2, which lacks `declare -A`, `mapfile`, etc.).
pub fn warn_if_old_bash() {
    if let Some(major) = bash_major_version() {
        if major < 4 {
            eprintln!(
                "{}",
                crate::tr!(
                    "⚠  bash {}.x aniqlandi — ba'zi mashqlar bash 4+ talab qiladi \
                     (declare -A, mapfile). Yangilash: `brew install bash`.",
                    "⚠  bash {}.x detected — some exercises need bash 4+ \
                     (declare -A, mapfile). Upgrade: `brew install bash`.",
                    major
                )
            );
        }
    }
}

/// Evaluate each assertion against the recorded run. `base` is the directory
/// used for `stdout-cmd` execution and for resolving relative `file-exists` paths.
pub fn evaluate(
    assertions: &[Assertion],
    actual_stdout: &str,
    actual_stderr: &str,
    actual_exit: i32,
    base: &Path,
) -> Result<Vec<AssertionResult>> {
    let normalized_stdout = normalize(actual_stdout);
    let normalized_stderr = normalize(actual_stderr);
    let mut results = Vec::with_capacity(assertions.len());

    for a in assertions {
        let (passed, expected, actual) = match a {
            Assertion::Stdout(expected) => {
                let exp = expected.clone();
                let act = normalized_stdout.clone();
                (exp == act, exp, act)
            }
            Assertion::StdoutCmd(cmd) => {
                let expected_raw = run_expected_cmd(cmd, base)?;
                let exp = normalize(&expected_raw);
                let act = normalized_stdout.clone();
                (exp == act, exp, act)
            }
            Assertion::StdoutContains(sub) => {
                let passed = normalized_stdout.contains(sub.as_str());
                (
                    passed,
                    format!("contains {sub:?}"),
                    normalized_stdout.clone(),
                )
            }
            Assertion::StdoutRegex(pat) => {
                // Pattern is validated at parse time, so this won't normally fail.
                let re = regex::Regex::new(pat)
                    .with_context(|| format!("noto'g'ri regex: '{pat}'"))?;
                let passed = re.is_match(&normalized_stdout);
                (passed, format!("/{pat}/"), normalized_stdout.clone())
            }
            Assertion::Stderr(expected) => {
                let exp = expected.clone();
                let act = normalized_stderr.clone();
                (exp == act, exp, act)
            }
            Assertion::Exit(code) => {
                let passed = *code == actual_exit;
                (passed, code.to_string(), actual_exit.to_string())
            }
            Assertion::FileExists(p) => {
                // Relative paths resolve against the run's base directory.
                let candidate = if Path::new(p).is_absolute() {
                    Path::new(p).to_path_buf()
                } else {
                    base.join(p)
                };
                let exists = candidate.exists();
                (
                    exists,
                    format!("mavjud: {p}"),
                    if exists { "mavjud" } else { "yo'q" }.to_string(),
                )
            }
        };
        results.push(AssertionResult {
            assertion: a.clone(),
            passed,
            expected,
            actual,
        });
    }
    Ok(results)
}

/// Convenience: parse, run (in `cwd`), evaluate — and return everything.
pub fn run_full(path: &Path, cwd: &Path) -> Result<TestReport> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("'{}' faylini o'qib bo'lmadi", path.display()))?;
    let assertions = parse_assertions(&content)?;
    let run = run_script(path, cwd)?;
    let results = evaluate(&assertions, &run.stdout, &run.stderr, run.exit_code, cwd)?;
    Ok(TestReport {
        results,
        stdout: run.stdout,
        stderr: run.stderr,
        exit_code: run.exit_code,
        timed_out: run.timed_out,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─── parse_assertions ──────────────────────────────────────────────

    #[test]
    fn parses_stdout_directive() {
        let src = "echo hi\n# @test:stdout: hello\n";
        let out = parse_assertions(src).unwrap();
        assert_eq!(out.len(), 1);
        match &out[0] {
            Assertion::Stdout(s) => assert_eq!(s, "hello"),
            _ => panic!("expected Stdout"),
        }
    }

    #[test]
    fn parses_stdout_cmd_directive() {
        let src = "# @test:stdout-cmd: printf 'a\\nb\\n'\n";
        let out = parse_assertions(src).unwrap();
        match &out[0] {
            Assertion::StdoutCmd(c) => assert_eq!(c, "printf 'a\\nb\\n'"),
            _ => panic!("expected StdoutCmd"),
        }
    }

    #[test]
    fn parses_exit_directive() {
        let src = "# @test:exit: 1\n";
        let out = parse_assertions(src).unwrap();
        match &out[0] {
            Assertion::Exit(code) => assert_eq!(*code, 1),
            _ => panic!("expected Exit"),
        }
    }

    #[test]
    fn parses_multiple_directives() {
        let src = "echo x\n# @test:stdout: x\n# @test:exit: 0\n";
        let out = parse_assertions(src).unwrap();
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn ignores_non_directive_comments() {
        let src = "# Just a comment\n# Another comment\n";
        let out = parse_assertions(src).unwrap();
        assert!(out.is_empty());
    }

    #[test]
    fn directive_value_with_colon_is_preserved() {
        // The bug-prone case: directive value contains ':' (e.g., "name: foo")
        let src = "# @test:stdout: name: Ali\n";
        let out = parse_assertions(src).unwrap();
        match &out[0] {
            Assertion::Stdout(s) => assert_eq!(s, "name: Ali"),
            _ => panic!("expected Stdout"),
        }
    }

    #[test]
    fn unknown_directive_does_not_fail() {
        // Unknown directives produce a warning to stderr but don't error.
        let src = "# @test:foo: bar\n";
        let out = parse_assertions(src).unwrap();
        assert!(out.is_empty());
    }

    #[test]
    fn malformed_directive_returns_error() {
        // Missing the second colon.
        let src = "# @test:stdout-no-colon\n";
        let result = parse_assertions(src);
        assert!(result.is_err());
    }

    #[test]
    fn invalid_exit_code_returns_error() {
        let src = "# @test:exit: notanumber\n";
        let result = parse_assertions(src);
        assert!(result.is_err());
    }

    // ─── normalize ─────────────────────────────────────────────────────

    #[test]
    fn normalize_strips_trailing_newlines() {
        assert_eq!(normalize("hello\n"), "hello");
        assert_eq!(normalize("hello\n\n\n"), "hello");
        assert_eq!(normalize("hello\r\n"), "hello");
    }

    #[test]
    fn normalize_preserves_internal_whitespace() {
        assert_eq!(normalize("a  b\n"), "a  b");
        assert_eq!(normalize("a\nb\n"), "a\nb");
    }

    // ─── evaluate ──────────────────────────────────────────────────────

    #[test]
    fn evaluate_stdout_match_passes() {
        let assertions = vec![Assertion::Stdout("hi".into())];
        let results = evaluate(&assertions, "hi\n", "", 0, Path::new(".")).unwrap();
        assert!(results[0].passed);
    }

    #[test]
    fn evaluate_stdout_mismatch_fails() {
        let assertions = vec![Assertion::Stdout("hi".into())];
        let results = evaluate(&assertions, "ho\n", "", 0, Path::new(".")).unwrap();
        assert!(!results[0].passed);
        assert_eq!(results[0].actual, "ho");
    }

    #[test]
    fn evaluate_exit_match() {
        let assertions = vec![Assertion::Exit(0)];
        let results = evaluate(&assertions, "", "", 0, Path::new(".")).unwrap();
        assert!(results[0].passed);

        let results = evaluate(&assertions, "", "", 1, Path::new(".")).unwrap();
        assert!(!results[0].passed);
    }

    #[test]
    fn evaluate_stdout_contains() {
        let assertions = vec![Assertion::StdoutContains("ll".into())];
        assert!(evaluate(&assertions, "hello\n", "", 0, Path::new(".")).unwrap()[0].passed);
        assert!(!evaluate(&assertions, "hi\n", "", 0, Path::new(".")).unwrap()[0].passed);
    }

    #[test]
    fn evaluate_stdout_regex() {
        let assertions = vec![Assertion::StdoutRegex(r"^\d{4}-\d{2}-\d{2}$".into())];
        assert!(evaluate(&assertions, "2026-06-13\n", "", 0, Path::new(".")).unwrap()[0].passed);
        assert!(!evaluate(&assertions, "13/06/2026\n", "", 0, Path::new(".")).unwrap()[0].passed);
    }

    #[test]
    fn evaluate_stderr_match() {
        let assertions = vec![Assertion::Stderr("boom".into())];
        assert!(evaluate(&assertions, "", "boom\n", 1, Path::new(".")).unwrap()[0].passed);
        assert!(!evaluate(&assertions, "", "other\n", 1, Path::new(".")).unwrap()[0].passed);
    }

    #[test]
    fn evaluate_file_exists() {
        let present = vec![Assertion::FileExists("Cargo.toml".into())];
        assert!(evaluate(&present, "", "", 0, Path::new(".")).unwrap()[0].passed);
        let absent = vec![Assertion::FileExists("no-such-file-xyz".into())];
        assert!(!evaluate(&absent, "", "", 0, Path::new(".")).unwrap()[0].passed);
    }

    // ─── parse new directives ──────────────────────────────────────────

    #[test]
    fn parses_new_directives() {
        let src = "# @test:stdout-contains: foo\n# @test:stdout-regex: ^a.+\n\
                   # @test:stderr: err\n# @test:file-exists: /tmp/x\n";
        let out = parse_assertions(src).unwrap();
        assert_eq!(out.len(), 4);
        assert!(matches!(out[0], Assertion::StdoutContains(_)));
        assert!(matches!(out[1], Assertion::StdoutRegex(_)));
        assert!(matches!(out[2], Assertion::Stderr(_)));
        assert!(matches!(out[3], Assertion::FileExists(_)));
    }

    #[test]
    fn invalid_regex_returns_error() {
        let src = "# @test:stdout-regex: (unclosed\n";
        assert!(parse_assertions(src).is_err());
    }

    // ─── TestReport aggregate ──────────────────────────────────────────

    #[test]
    fn all_passed_requires_nonempty() {
        let report = TestReport {
            results: vec![],
            stdout: String::new(),
            stderr: String::new(),
            exit_code: 0,
            timed_out: false,
        };
        assert!(!report.all_passed());
    }

    #[test]
    fn passed_count_is_accurate() {
        let report = TestReport {
            results: vec![
                AssertionResult {
                    assertion: Assertion::Exit(0),
                    passed: true,
                    expected: "0".into(),
                    actual: "0".into(),
                },
                AssertionResult {
                    assertion: Assertion::Exit(0),
                    passed: false,
                    expected: "0".into(),
                    actual: "1".into(),
                },
            ],
            stdout: String::new(),
            stderr: String::new(),
            exit_code: 0,
            timed_out: false,
        };
        assert_eq!(report.passed_count(), 1);
        assert_eq!(report.total(), 2);
        assert!(!report.all_passed());
    }
}
