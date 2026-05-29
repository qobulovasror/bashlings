//! Test runner: parses `# @test:...` directives, executes the script via bash,
//! and evaluates each assertion.

use anyhow::{anyhow, Context, Result};
use std::path::Path;
use std::process::Command;

/// A single test assertion declared at the bottom of an exercise file.
#[derive(Debug, Clone)]
pub enum Assertion {
    /// Script's stdout must equal this literal string (after trailing-newline trim).
    Stdout(String),
    /// Script's stdout must equal the stdout of running this command via `bash -c`.
    StdoutCmd(String),
    /// Script's exit code must equal this number.
    Exit(i32),
}

impl Assertion {
    pub fn kind(&self) -> &'static str {
        match self {
            Assertion::Stdout(_) => "stdout",
            Assertion::StdoutCmd(_) => "stdout-cmd",
            Assertion::Exit(_) => "exit",
        }
    }

    pub fn preview(&self) -> String {
        match self {
            Assertion::Stdout(v) => format!("{v:?}"),
            Assertion::StdoutCmd(c) => format!("`{c}`"),
            Assertion::Exit(c) => c.to_string(),
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

/// Aggregate report for one exercise run.
#[derive(Debug)]
pub struct TestReport {
    pub results: Vec<AssertionResult>,
    #[allow(dead_code)]
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
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
///   - `stdout`     — exact string match
///   - `stdout-cmd` — run `bash -c <value>`, compare its stdout
///   - `exit`       — exact exit-code match (integer)
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
            "exit" => {
                let code: i32 = value.parse().with_context(|| {
                    format!("{lineno}-qator: '@test:exit' qiymati son emas: '{value}'")
                })?;
                out.push(Assertion::Exit(code));
            }
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
pub fn run_script(path: &Path) -> Result<(String, String, i32)> {
    let output = Command::new("bash")
        .arg(path)
        .output()
        .with_context(|| format!("'bash {}' ni ishga tushira olmadik", path.display()))?;
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    let exit_code = output.status.code().unwrap_or(-1);
    Ok((stdout, stderr, exit_code))
}

/// Run a one-liner via `bash -c` and return its stdout.
fn run_expected_cmd(cmd: &str) -> Result<String> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .with_context(|| format!("expected komandasini bajara olmadik: 'bash -c {cmd}'"))?;
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Evaluate each assertion against the recorded run.
pub fn evaluate(
    assertions: &[Assertion],
    actual_stdout: &str,
    actual_exit: i32,
) -> Result<Vec<AssertionResult>> {
    let normalized_stdout = normalize(actual_stdout);
    let mut results = Vec::with_capacity(assertions.len());

    for a in assertions {
        let (passed, expected, actual) = match a {
            Assertion::Stdout(expected) => {
                let exp = expected.clone();
                let act = normalized_stdout.clone();
                (exp == act, exp, act)
            }
            Assertion::StdoutCmd(cmd) => {
                let expected_raw = run_expected_cmd(cmd)?;
                let exp = normalize(&expected_raw);
                let act = normalized_stdout.clone();
                (exp == act, exp, act)
            }
            Assertion::Exit(code) => {
                let passed = *code == actual_exit;
                (passed, code.to_string(), actual_exit.to_string())
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

/// Convenience: parse, run, evaluate — and return everything.
pub fn run_full(path: &Path) -> Result<TestReport> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("'{}' faylini o'qib bo'lmadi", path.display()))?;
    let assertions = parse_assertions(&content)?;
    let (stdout, stderr, exit_code) = run_script(path)?;
    let results = evaluate(&assertions, &stdout, exit_code)?;
    Ok(TestReport {
        results,
        stdout,
        stderr,
        exit_code,
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
        let results = evaluate(&assertions, "hi\n", 0).unwrap();
        assert!(results[0].passed);
    }

    #[test]
    fn evaluate_stdout_mismatch_fails() {
        let assertions = vec![Assertion::Stdout("hi".into())];
        let results = evaluate(&assertions, "ho\n", 0).unwrap();
        assert!(!results[0].passed);
        assert_eq!(results[0].actual, "ho");
    }

    #[test]
    fn evaluate_exit_match() {
        let assertions = vec![Assertion::Exit(0)];
        let results = evaluate(&assertions, "", 0).unwrap();
        assert!(results[0].passed);

        let results = evaluate(&assertions, "", 1).unwrap();
        assert!(!results[0].passed);
    }

    // ─── TestReport aggregate ──────────────────────────────────────────

    #[test]
    fn all_passed_requires_nonempty() {
        let report = TestReport {
            results: vec![],
            stdout: String::new(),
            stderr: String::new(),
            exit_code: 0,
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
        };
        assert_eq!(report.passed_count(), 1);
        assert_eq!(report.total(), 2);
        assert!(!report.all_passed());
    }
}
