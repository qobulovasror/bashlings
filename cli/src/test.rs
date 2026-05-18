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
