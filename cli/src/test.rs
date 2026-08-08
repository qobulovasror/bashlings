//! Test runner: parses `# @test:...` directives, executes the script via bash
//! inside a per-run [`crate::sandbox::Sandbox`], and evaluates each assertion.

use crate::sandbox::Sandbox;
use crate::tr;
use anyhow::{anyhow, Context, Result};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant};

/// Hard wall-clock limit (seconds) for a single exercise script. Protects
/// against infinite loops (`while true; do :; done`) hanging the CLI forever.
pub const SCRIPT_TIMEOUT_SECS: u64 = 10;
const SCRIPT_TIMEOUT: Duration = Duration::from_secs(SCRIPT_TIMEOUT_SECS);

/// Wrapper that runs before the learner's script, in the same process (`exec`).
///
/// Timeout + process-group kill handle a runaway loop, but a fork bomb would
/// still saturate the machine for the full timeout — so cap the process count
/// at "whatever this user already runs, plus a little headroom", and cap file
/// size at 64 MB. Both are fail-open: on a system where the hard limit is
/// lower, or `ps` is unavailable, the script simply runs unrestricted.
const HARDEN_PREAMBLE: &str = r#"
__bl_procs=$(ps -U "$(id -u)" -o pid= 2>/dev/null | wc -l 2>/dev/null)
case "$__bl_procs" in
  ''|*[!0-9' ']*) : ;;
  *) ulimit -u "$((__bl_procs + 64))" 2>/dev/null || : ;;
esac
ulimit -f 65536 2>/dev/null || :
unset __bl_procs
exec bash "$0" "$@"
"#;

/// Everything a script run needs beyond the script itself: where it runs, what
/// it is given, and how long it may take.
///
/// The same context drives `@test:stdout-cmd` expected commands — if the two
/// sides disagreed about cwd, env or stdin, exercises like
/// `# @test:stdout-cmd: pwd` would never pass.
pub struct RunCtx<'a> {
    pub workdir: &'a Path,
    pub env: &'a [(String, String)],
    /// Positional parameters handed to the script (`$1`, `$@`).
    pub args: &'a [String],
    /// File served as the script's stdin; `None` means `/dev/null`.
    pub stdin: Option<&'a Path>,
    pub timeout: Duration,
}

#[cfg(test)]
impl<'a> RunCtx<'a> {
    /// Defaults matching a bare script run: no arguments, no stdin, 10s.
    fn new(workdir: &'a Path, env: &'a [(String, String)]) -> Self {
        Self {
            workdir,
            env,
            args: &[],
            stdin: None,
            timeout: SCRIPT_TIMEOUT,
        }
    }
}

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
    /// The file must exist and its contents must equal this literal.
    FileContent { path: String, expected: String },
    /// The file must exist and contain this substring.
    FileContains { path: String, needle: String },
    /// Nothing may exist at this path — not even a broken symlink.
    FileMissing(String),
    /// The path's permission bits must equal this mode.
    Perm { path: String, mode: u32 },
    /// The files left in the working directory, exactly.
    Tree(Vec<String>),
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
            Assertion::FileContent { .. } => "file-content",
            Assertion::FileContains { .. } => "file-contains",
            Assertion::FileMissing(_) => "file-missing",
            Assertion::Perm { .. } => "perm",
            Assertion::Tree(_) => "tree",
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
            Assertion::FileContent { path, expected } => format!("{path} = {expected:?}"),
            Assertion::FileContains { path, needle } => format!("{path} ⊇ {needle:?}"),
            Assertion::FileMissing(p) => format!("¬{p}"),
            Assertion::Perm { path, mode } => format!("{path} {mode:04o}"),
            Assertion::Tree(paths) => paths.join(", "),
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
    /// Limit actually applied — `@setup:timeout` may raise or lower the default.
    pub timeout_secs: u64,
    /// Sandbox directory left on disk for inspection (`--keep-sandbox` only).
    pub sandbox: Option<PathBuf>,
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
///   - `file-missing`    — nothing may exist at the given path
///   - `file-content`    — `<path> :: <literal>` file contents
///   - `file-contains`   — `<path> :: <substring>`
///   - `perm`            — `<path> <0755>` permission bits
///   - `tree`            — the exact set of files left behind, comma-separated
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
            return Err(anyhow!(tr!(
                "{}-qator: noto'g'ri '@test:' direktiva — '# @test:KIND: VALUE' kutilgan",
                "line {}: invalid '@test:' directive — expected '# @test:KIND: VALUE'",
                lineno
            )));
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
                    tr!(
                        "{}-qator: '@test:stdout-regex' noto'g'ri regex: '{}'",
                        "line {}: '@test:stdout-regex' invalid regex: '{}'",
                        lineno,
                        value
                    )
                })?;
                out.push(Assertion::StdoutRegex(value.to_string()));
            }
            "stderr" => out.push(Assertion::Stderr(value.to_string())),
            "exit" => {
                let code: i32 = value.parse().with_context(|| {
                    tr!(
                        "{}-qator: '@test:exit' qiymati son emas: '{}'",
                        "line {}: '@test:exit' value is not a number: '{}'",
                        lineno,
                        value
                    )
                })?;
                out.push(Assertion::Exit(code));
            }
            "file-exists" => out.push(Assertion::FileExists(value.to_string())),
            "file-missing" => out.push(Assertion::FileMissing(value.to_string())),
            "file-content" => {
                let (path, expected) = split_pair(value, kind, lineno)?;
                out.push(Assertion::FileContent { path, expected });
            }
            "file-contains" => {
                let (path, needle) = split_pair(value, kind, lineno)?;
                out.push(Assertion::FileContains { path, needle });
            }
            "perm" => {
                // The mode is the last field, so a path with spaces still works.
                let (path, mode) = value.rsplit_once(' ').ok_or_else(|| {
                    anyhow!(tr!(
                        "{}-qator: '@test:perm' formati — '<yo'l> <0755>'",
                        "line {}: '@test:perm' format is '<path> <0755>'",
                        lineno
                    ))
                })?;
                let mode = u32::from_str_radix(mode.trim(), 8).with_context(|| {
                    tr!(
                        "{}-qator: rejim sakkizlik son bo'lishi kerak (masalan 0644): '{}'",
                        "line {}: mode must be octal (e.g. 0644): '{}'",
                        lineno,
                        mode.trim()
                    )
                })?;
                out.push(Assertion::Perm {
                    path: path.trim().to_string(),
                    mode,
                });
            }
            "tree" => {
                let paths: Vec<String> = value
                    .split(',')
                    .map(str::trim)
                    .filter(|p| !p.is_empty())
                    .map(String::from)
                    .collect();
                out.push(Assertion::Tree(paths));
            }
            other => {
                eprintln!(
                    "{}",
                    tr!(
                        "⚠  {}-qator: noma'lum direktiva @test:{} (e'tibordan chetda)",
                        "⚠  line {}: unknown directive @test:{} (ignored)",
                        lineno,
                        other
                    )
                );
            }
        }
    }
    Ok(out)
}

/// Trim trailing newlines (\n, \r) only — preserve all leading/internal whitespace.
fn normalize(s: &str) -> String {
    s.trim_end_matches(['\n', '\r']).to_string()
}

/// Split a `<path> :: <value>` directive. `::` rather than whitespace, because
/// file names with spaces are course material.
fn split_pair(value: &str, kind: &str, lineno: usize) -> Result<(String, String)> {
    let (path, rest) = value.split_once("::").ok_or_else(|| {
        anyhow!(tr!(
            "{}-qator: '@test:{}' formati — '<yo'l> :: <qiymat>'",
            "line {}: '@test:{}' format is '<path> :: <value>'",
            lineno,
            kind
        ))
    })?;
    Ok((path.trim().to_string(), rest.trim().to_string()))
}

/// Resolve an assertion path: relative ones hang off the run's working directory.
fn resolve_against(base: &Path, path: &str) -> PathBuf {
    if Path::new(path).is_absolute() {
        PathBuf::from(path)
    } else {
        base.join(path)
    }
}

/// Permission bits of `path`, or `None` when it doesn't exist.
#[cfg(unix)]
fn file_mode(path: &Path) -> Option<u32> {
    use std::os::unix::fs::PermissionsExt;
    Some(std::fs::metadata(path).ok()?.permissions().mode() & 0o7777)
}

#[cfg(not(unix))]
fn file_mode(_path: &Path) -> Option<u32> {
    None
}

/// Every file (and symlink) under `dir`, as sorted `/`-joined relative paths.
/// Directories are not listed — an empty one is invisible to `@test:tree`.
fn collect_tree(dir: &Path) -> Vec<String> {
    fn walk(dir: &Path, prefix: &str, out: &mut Vec<String>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            let rel = if prefix.is_empty() {
                name
            } else {
                format!("{prefix}/{name}")
            };
            match std::fs::symlink_metadata(entry.path()) {
                Ok(meta) if meta.is_dir() => walk(&entry.path(), &rel, out),
                Ok(_) => out.push(rel),
                Err(_) => {}
            }
        }
    }
    let mut out = Vec::new();
    walk(dir, "", &mut out);
    out.sort();
    out
}

/// Execute the user's exercise script with `bash <path>`.
///
/// Hardened runner:
///   - runs in `ctx.workdir` — a throwaway sandbox, never the workspace
///   - stdin is `/dev/null` (a `read` in the script can't block on the terminal)
///   - own process group, so the timeout kills grandchildren too
///   - `ulimit` caps on process count and file size (see [`HARDEN_PREAMBLE`])
///   - killed after [`SCRIPT_TIMEOUT`] (infinite loops can't hang the CLI)
pub fn run_script(path: &Path, ctx: &RunCtx) -> Result<ScriptOutput> {
    let mut cmd = Command::new("bash");
    cmd.arg("-c")
        .arg(HARDEN_PREAMBLE)
        // `$0` for the preamble — the script it `exec`s into — then `$@`.
        .arg(path)
        .args(ctx.args)
        .current_dir(ctx.workdir)
        .envs(ctx.env.iter().map(|(k, v)| (k.as_str(), v.as_str())))
        .stdin(stdin_from(ctx.stdin)?)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Own process group: `kill(child)` would leave a fork bomb's descendants
    // running — and holding the stdout pipe open, which would hang the drain
    // threads below forever. `killpg` takes the whole tree down.
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }

    let mut child = cmd.spawn().with_context(|| {
        tr!("'bash {}' ni ishga tushira olmadik", "could not run 'bash {}'", path.display())
    })?;
    let pgid = child.id();

    // Drain stdout/stderr on separate threads so a chatty script can't fill the
    // pipe buffer and deadlock while we poll for the timeout.
    let out_rx = drain(child.stdout.take().expect("stdout piped"));
    let err_rx = drain(child.stderr.take().expect("stderr piped"));

    let start = Instant::now();
    let mut timed_out = false;
    let status = loop {
        match child
            .try_wait()
            .context(tr!("skript holatini tekshirib bo'lmadi", "could not check script status"))?
        {
            Some(status) => break status,
            None => {
                if start.elapsed() >= ctx.timeout {
                    kill_group(pgid);
                    let _ = child.kill();
                    timed_out = true;
                    break child.wait().context(tr!(
                        "to'xtatilgan skriptni kutib bo'lmadi",
                        "could not wait on the killed script"
                    ))?;
                }
                std::thread::sleep(Duration::from_millis(20));
            }
        }
    };

    // A background job (`sleep 30 &`) inherits the pipes, so reading to EOF can
    // outlive the script itself. Grant a short grace period, then take the whole
    // process group down — the CLI must never hang on a leftover child.
    let mut group_killed = timed_out;
    let stdout = String::from_utf8_lossy(&collect(&out_rx, pgid, &mut group_killed)).into_owned();
    let stderr = String::from_utf8_lossy(&collect(&err_rx, pgid, &mut group_killed)).into_owned();
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

/// How long a leftover background job may keep the pipes open after the script
/// itself has exited, before the whole process group is killed.
const DRAIN_GRACE: Duration = Duration::from_millis(200);

/// Open the stdin file for a child process.
///
/// `@setup:stdin` is served from a *file*, never a pipe. A pipe would mean the
/// CLI has to keep writing while the script reads — and a script that stops
/// reading (`read -r name` and nothing more) turns that write into `EPIPE`,
/// which `main`'s `SIGPIPE`/`SIG_DFL` disposition escalates into the CLI being
/// killed outright. A file also matches how a learner runs the script by hand:
/// `./script.sh < input.txt`.
fn stdin_from(path: Option<&Path>) -> Result<Stdio> {
    match path {
        None => Ok(Stdio::null()),
        Some(p) => {
            let file = std::fs::File::open(p).with_context(|| {
                tr!(
                    "stdin faylini ocholmadik: {}",
                    "could not open the stdin file: {}",
                    p.display()
                )
            })?;
            Ok(Stdio::from(file))
        }
    }
}

/// Read a pipe to EOF on its own thread; the buffer arrives on the channel.
fn drain<R: Read + Send + 'static>(mut pipe: R) -> mpsc::Receiver<Vec<u8>> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = pipe.read_to_end(&mut buf);
        let _ = tx.send(buf);
    });
    rx
}

/// Wait for one drained pipe, killing the process group if it takes too long.
/// `group_killed` makes the kill happen at most once per run.
fn collect(rx: &mpsc::Receiver<Vec<u8>>, pgid: u32, group_killed: &mut bool) -> Vec<u8> {
    match rx.recv_timeout(DRAIN_GRACE) {
        Ok(buf) => buf,
        Err(_) => {
            if !*group_killed {
                kill_group(pgid);
                *group_killed = true;
            }
            // With the writers gone the pipe hits EOF, so this returns promptly.
            #[cfg(unix)]
            {
                rx.recv().unwrap_or_default()
            }
            #[cfg(not(unix))]
            {
                Vec::new()
            }
        }
    }
}

/// Kill the script and every process it spawned. No-op where process groups
/// aren't available.
fn kill_group(pgid: u32) {
    #[cfg(unix)]
    {
        // The child is its own group leader, so pgid == its pid.
        unsafe {
            libc::killpg(pgid as libc::pid_t, libc::SIGKILL);
        }
    }
    #[cfg(not(unix))]
    {
        let _ = pgid;
    }
}

/// Run a one-liner via `bash -c` in the same sandbox and environment as the
/// exercise script, and return its stdout.
fn run_expected_cmd(cmd: &str, ctx: &RunCtx) -> Result<String> {
    let mut child = Command::new("bash");
    child
        .arg("-c")
        .arg(cmd)
        .current_dir(ctx.workdir)
        .envs(ctx.env.iter().map(|(k, v)| (k.as_str(), v.as_str())));

    // The expected command reads the same stdin as the script — a fresh handle
    // on the same file, so it starts at offset 0 — letting `@setup:stdin` plus
    // `# @test:stdout-cmd: sort` compare like for like.
    let output = child
        .stdin(stdin_from(ctx.stdin)?)
        .output()
        .with_context(|| {
            tr!(
                "expected komandasini bajara olmadik: 'bash -c {}'",
                "could not run the expected command: 'bash -c {}'",
                cmd
            )
        })?;
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

/// Evaluate each assertion against the recorded run. `ctx.workdir` is the
/// directory used for `stdout-cmd` execution and for resolving relative
/// `file-exists` paths — i.e. the sandbox the script just ran in.
pub fn evaluate(
    assertions: &[Assertion],
    actual_stdout: &str,
    actual_stderr: &str,
    actual_exit: i32,
    ctx: &RunCtx,
) -> Result<Vec<AssertionResult>> {
    let base = ctx.workdir;
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
                let expected_raw = run_expected_cmd(cmd, ctx)?;
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
                    .with_context(|| tr!("noto'g'ri regex: '{}'", "invalid regex: '{}'", pat))?;
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
                let exists = resolve_against(base, p).exists();
                (
                    exists,
                    format!("mavjud: {p}"),
                    if exists {
                        tr!("mavjud", "exists")
                    } else {
                        tr!("yo'q", "missing")
                    }
                    .to_string(),
                )
            }
            Assertion::FileMissing(p) => {
                // `symlink_metadata`, not `exists()`: a broken symlink is still
                // something left behind.
                let present = std::fs::symlink_metadata(resolve_against(base, p)).is_ok();
                (
                    !present,
                    tr!("yo'q: {}", "missing: {}", p),
                    if present {
                        tr!("mavjud", "exists")
                    } else {
                        tr!("yo'q", "missing")
                    }
                    .to_string(),
                )
            }
            Assertion::FileContent { path, expected } => {
                let exp = normalize(expected);
                match std::fs::read_to_string(resolve_against(base, path)) {
                    // A missing file is a learner mistake, not a runner error.
                    Err(_) => (false, exp, tr!("<fayl yo'q>", "<no such file>").to_string()),
                    Ok(raw) => {
                        let act = normalize(&raw);
                        (exp == act, exp, act)
                    }
                }
            }
            Assertion::FileContains { path, needle } => {
                let exp = format!("{path} ⊇ {needle:?}");
                match std::fs::read_to_string(resolve_against(base, path)) {
                    Err(_) => (false, exp, tr!("<fayl yo'q>", "<no such file>").to_string()),
                    Ok(raw) => (raw.contains(needle.as_str()), exp, normalize(&raw)),
                }
            }
            Assertion::Perm { path, mode } => {
                let exp = format!("{mode:04o}");
                let actual_mode = file_mode(&resolve_against(base, path));
                match actual_mode {
                    Some(bits) => (bits == *mode, exp, format!("{bits:04o}")),
                    None => (false, exp, tr!("<fayl yo'q>", "<no such file>").to_string()),
                }
            }
            Assertion::Tree(paths) => {
                let mut exp_list = paths.clone();
                exp_list.sort();
                let act_list = collect_tree(base);
                (
                    exp_list == act_list,
                    exp_list.join("\n"),
                    act_list.join("\n"),
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

/// Convenience: parse, run in a fresh sandbox, evaluate — and return everything.
///
/// `workspace_root` only locates `.bashlings/sandbox/`; the script itself never
/// runs there. The sandbox is removed when this function returns.
pub fn run_full(path: &Path, workspace_root: &Path) -> Result<TestReport> {
    let content = std::fs::read_to_string(path)
        .with_context(|| tr!("'{}' faylini o'qib bo'lmadi", "could not read file '{}'", path.display()))?;
    let assertions = parse_assertions(&content)?;

    let name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("exercise");
    let plan = crate::setup::parse(&content)?;
    let sandbox = Sandbox::create(workspace_root, name)?;
    // The fixture lands in `work/`, the script copy beside it, so an exercise
    // that runs `ls` sees its fixture and nothing else.
    plan.apply(&sandbox, path)?;
    let script = sandbox.install_script(path)?;

    // Author-declared `@setup:env` wins over the sandbox defaults.
    let mut env = sandbox.env();
    if plan.isolate_home {
        env.push(sandbox.isolated_home()?);
    }
    env.extend(plan.env.iter().cloned());

    // Outside `work/`, so `ls` and `@test:tree` never see it.
    let stdin_path = match &plan.stdin {
        None => None,
        Some(text) => {
            let p = sandbox.root().join(".stdin");
            std::fs::write(&p, text).with_context(|| {
                tr!(
                    "stdin faylini yozib bo'lmadi: {}",
                    "could not write the stdin file: {}",
                    p.display()
                )
            })?;
            Some(p)
        }
    };

    let timeout = plan
        .timeout_secs
        .map(Duration::from_secs)
        .unwrap_or(SCRIPT_TIMEOUT);
    let ctx = RunCtx {
        workdir: sandbox.workdir(),
        env: &env,
        args: &plan.args,
        stdin: stdin_path.as_deref(),
        timeout,
    };

    let run = run_script(&script, &ctx)?;

    // bash reports errors against the file it ran — the sandbox copy. Show the
    // learner the exercise they actually edit instead, so a "command not found"
    // reads `exercises/01_intro/intro1.sh: line 4: ...`. Done before `evaluate`
    // so `@test:stderr` compares stable text too.
    let stderr = run.stderr.replace(
        &script.display().to_string(),
        &path
            .strip_prefix(workspace_root)
            .unwrap_or(path)
            .display()
            .to_string(),
    );

    // Assertions read files the script produced, so evaluate before the
    // sandbox is dropped at the end of this scope.
    let results = evaluate(&assertions, &run.stdout, &stderr, run.exit_code, &ctx)?;

    Ok(TestReport {
        results,
        stdout: run.stdout,
        stderr,
        exit_code: run.exit_code,
        timed_out: run.timed_out,
        timeout_secs: timeout.as_secs(),
        sandbox: sandbox.kept_path(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Context for assertions that don't touch the filesystem. Tests run with
    /// `cli/` as the working directory.
    fn test_ctx() -> RunCtx<'static> {
        RunCtx::new(Path::new("."), &[])
    }

    /// Write a throwaway script and run it in its own directory.
    fn run_snippet(tag: &str, body: &str, env: &[(String, String)]) -> ScriptOutput {
        run_snippet_full(tag, body, env, &[], None, SCRIPT_TIMEOUT)
    }

    /// Same, with full control over arguments, stdin and the timeout.
    fn run_snippet_full(
        tag: &str,
        body: &str,
        env: &[(String, String)],
        args: &[String],
        stdin: Option<&str>,
        timeout: Duration,
    ) -> ScriptOutput {
        let dir = std::env::temp_dir().join(format!(
            "bashlings-run-test-{}-{}",
            std::process::id(),
            tag
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let script = dir.join("snippet.sh");
        std::fs::write(&script, body).unwrap();
        // Mirrors `run_full`: stdin is served from a file, never a pipe.
        let stdin_path = stdin.map(|text| {
            let p = dir.join(".stdin");
            std::fs::write(&p, text).unwrap();
            p
        });
        let ctx = RunCtx {
            workdir: &dir,
            env,
            args,
            stdin: stdin_path.as_deref(),
            timeout,
        };
        let out = run_script(&script, &ctx).unwrap();
        let _ = std::fs::remove_dir_all(&dir);
        out
    }

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
        let results = evaluate(&assertions, "hi\n", "", 0, &test_ctx()).unwrap();
        assert!(results[0].passed);
    }

    #[test]
    fn evaluate_stdout_mismatch_fails() {
        let assertions = vec![Assertion::Stdout("hi".into())];
        let results = evaluate(&assertions, "ho\n", "", 0, &test_ctx()).unwrap();
        assert!(!results[0].passed);
        assert_eq!(results[0].actual, "ho");
    }

    #[test]
    fn evaluate_exit_match() {
        let assertions = vec![Assertion::Exit(0)];
        let results = evaluate(&assertions, "", "", 0, &test_ctx()).unwrap();
        assert!(results[0].passed);

        let results = evaluate(&assertions, "", "", 1, &test_ctx()).unwrap();
        assert!(!results[0].passed);
    }

    #[test]
    fn evaluate_stdout_contains() {
        let assertions = vec![Assertion::StdoutContains("ll".into())];
        assert!(evaluate(&assertions, "hello\n", "", 0, &test_ctx()).unwrap()[0].passed);
        assert!(!evaluate(&assertions, "hi\n", "", 0, &test_ctx()).unwrap()[0].passed);
    }

    #[test]
    fn evaluate_stdout_regex() {
        let assertions = vec![Assertion::StdoutRegex(r"^\d{4}-\d{2}-\d{2}$".into())];
        assert!(evaluate(&assertions, "2026-06-13\n", "", 0, &test_ctx()).unwrap()[0].passed);
        assert!(!evaluate(&assertions, "13/06/2026\n", "", 0, &test_ctx()).unwrap()[0].passed);
    }

    #[test]
    fn evaluate_stderr_match() {
        let assertions = vec![Assertion::Stderr("boom".into())];
        assert!(evaluate(&assertions, "", "boom\n", 1, &test_ctx()).unwrap()[0].passed);
        assert!(!evaluate(&assertions, "", "other\n", 1, &test_ctx()).unwrap()[0].passed);
    }

    #[test]
    fn evaluate_file_exists() {
        let present = vec![Assertion::FileExists("Cargo.toml".into())];
        assert!(evaluate(&present, "", "", 0, &test_ctx()).unwrap()[0].passed);
        let absent = vec![Assertion::FileExists("no-such-file-xyz".into())];
        assert!(!evaluate(&absent, "", "", 0, &test_ctx()).unwrap()[0].passed);
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

    // ─── filesystem assertions ─────────────────────────────────────────

    /// Directory holding a small fixture, plus a context pointing at it.
    fn fixture_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "bashlings-assert-test-{}-{}",
            std::process::id(),
            tag
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("sub")).unwrap();
        std::fs::write(dir.join("a.txt"), "olma\nanor\n").unwrap();
        std::fs::write(dir.join("sub").join("b.txt"), "deep").unwrap();
        dir
    }

    #[test]
    fn parses_new_filesystem_directives() {
        let src = "# @test:file-content: out.txt :: 3\n\
                   # @test:file-contains: log.txt :: XATO\n\
                   # @test:file-missing: .lock\n\
                   # @test:perm: my file.sh 0755\n\
                   # @test:tree: a.txt, sub/b.txt\n";
        let out = parse_assertions(src).unwrap();
        assert_eq!(out.len(), 5);
        assert!(matches!(&out[0], Assertion::FileContent { path, expected }
            if path == "out.txt" && expected == "3"));
        assert!(matches!(&out[1], Assertion::FileContains { needle, .. } if needle == "XATO"));
        assert!(matches!(&out[2], Assertion::FileMissing(p) if p == ".lock"));
        // Path with a space keeps its space; the mode is the last field.
        assert!(matches!(&out[3], Assertion::Perm { path, mode }
            if path == "my file.sh" && *mode == 0o755));
        assert!(matches!(&out[4], Assertion::Tree(p) if p == &["a.txt", "sub/b.txt"]));
    }

    #[test]
    fn malformed_filesystem_directives_error() {
        assert!(parse_assertions("# @test:file-content: no-separator\n").is_err());
        assert!(parse_assertions("# @test:file-contains: no-separator\n").is_err());
        assert!(parse_assertions("# @test:perm: nomode\n").is_err());
        assert!(parse_assertions("# @test:perm: x.sh 0999\n").is_err());
    }

    #[test]
    fn evaluate_file_content() {
        let dir = fixture_dir("content");
        let ctx = RunCtx::new(&dir, &[]);

        let ok = vec![Assertion::FileContent {
            path: "a.txt".into(),
            // Trailing newlines are normalised on both sides.
            expected: "olma\nanor".into(),
        }];
        assert!(evaluate(&ok, "", "", 0, &ctx).unwrap()[0].passed);

        let wrong = vec![Assertion::FileContent {
            path: "a.txt".into(),
            expected: "olma".into(),
        }];
        assert!(!evaluate(&wrong, "", "", 0, &ctx).unwrap()[0].passed);

        // A missing file fails the assertion instead of erroring the run.
        let absent = vec![Assertion::FileContent {
            path: "nope.txt".into(),
            expected: "x".into(),
        }];
        let results = evaluate(&absent, "", "", 0, &ctx).unwrap();
        assert!(!results[0].passed);
        assert!(results[0].actual.contains("fayl") || results[0].actual.contains("file"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn evaluate_file_contains() {
        let dir = fixture_dir("contains");
        let ctx = RunCtx::new(&dir, &[]);
        let hit = vec![Assertion::FileContains {
            path: "a.txt".into(),
            needle: "anor".into(),
        }];
        assert!(evaluate(&hit, "", "", 0, &ctx).unwrap()[0].passed);
        let miss = vec![Assertion::FileContains {
            path: "a.txt".into(),
            needle: "uzum".into(),
        }];
        assert!(!evaluate(&miss, "", "", 0, &ctx).unwrap()[0].passed);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn evaluate_file_missing() {
        let dir = fixture_dir("missing");
        let ctx = RunCtx::new(&dir, &[]);
        assert!(evaluate(&[Assertion::FileMissing("nope".into())], "", "", 0, &ctx).unwrap()[0].passed);
        assert!(!evaluate(&[Assertion::FileMissing("a.txt".into())], "", "", 0, &ctx).unwrap()[0].passed);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[cfg(unix)]
    #[test]
    fn file_missing_sees_a_broken_symlink() {
        let dir = fixture_dir("broken");
        std::os::unix::fs::symlink("nowhere", dir.join("dangling")).unwrap();
        let ctx = RunCtx::new(&dir, &[]);
        // `exists()` would say "missing" here — the link itself is still there.
        let results = evaluate(
            &[Assertion::FileMissing("dangling".into())],
            "",
            "",
            0,
            &ctx,
        )
        .unwrap();
        assert!(!results[0].passed);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[cfg(unix)]
    #[test]
    fn evaluate_perm() {
        use std::os::unix::fs::PermissionsExt;
        let dir = fixture_dir("perm");
        std::fs::set_permissions(dir.join("a.txt"), std::fs::Permissions::from_mode(0o600))
            .unwrap();
        let ctx = RunCtx::new(&dir, &[]);

        let ok = vec![Assertion::Perm {
            path: "a.txt".into(),
            mode: 0o600,
        }];
        assert!(evaluate(&ok, "", "", 0, &ctx).unwrap()[0].passed);

        let wrong = vec![Assertion::Perm {
            path: "a.txt".into(),
            mode: 0o644,
        }];
        let results = evaluate(&wrong, "", "", 0, &ctx).unwrap();
        assert!(!results[0].passed);
        assert_eq!(results[0].actual, "0600");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn evaluate_tree_compares_the_exact_file_set() {
        let dir = fixture_dir("tree");
        let ctx = RunCtx::new(&dir, &[]);

        // Order doesn't matter; both sides are sorted.
        let ok = vec![Assertion::Tree(vec!["sub/b.txt".into(), "a.txt".into()])];
        assert!(evaluate(&ok, "", "", 0, &ctx).unwrap()[0].passed);

        // An extra file on disk is a failure.
        std::fs::write(dir.join("extra.log"), "x").unwrap();
        let results = evaluate(&ok, "", "", 0, &ctx).unwrap();
        assert!(!results[0].passed);
        assert!(results[0].actual.contains("extra.log"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn tree_ignores_empty_directories() {
        let dir = fixture_dir("emptydir");
        std::fs::create_dir(dir.join("empty")).unwrap();
        let ctx = RunCtx::new(&dir, &[]);
        let ok = vec![Assertion::Tree(vec!["a.txt".into(), "sub/b.txt".into()])];
        assert!(evaluate(&ok, "", "", 0, &ctx).unwrap()[0].passed);
        let _ = std::fs::remove_dir_all(&dir);
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
            timeout_secs: SCRIPT_TIMEOUT_SECS,
            sandbox: None,
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
            timeout_secs: SCRIPT_TIMEOUT_SECS,
            sandbox: None,
        };
        assert_eq!(report.passed_count(), 1);
        assert_eq!(report.total(), 2);
        assert!(!report.all_passed());
    }

    // ─── run_script ────────────────────────────────────────────────────

    #[test]
    fn run_script_runs_in_workdir() {
        let out = run_snippet("cwd", "pwd\n", &[]);
        assert_eq!(out.exit_code, 0);
        assert!(
            out.stdout.trim().contains("bashlings-run-test"),
            "script should run in the given workdir, got {:?}",
            out.stdout
        );
    }

    #[test]
    fn run_script_applies_env_overrides() {
        let env = vec![
            ("BASHLINGS_SANDBOX".to_string(), "1".to_string()),
            ("TZ".to_string(), "UTC".to_string()),
        ];
        let out = run_snippet("env", "echo \"$BASHLINGS_SANDBOX:$TZ\"\n", &env);
        assert_eq!(out.stdout.trim(), "1:UTC");
    }

    #[test]
    fn run_script_reports_exit_code() {
        let out = run_snippet("exit", "exit 3\n", &[]);
        assert_eq!(out.exit_code, 3);
        assert!(!out.timed_out);
    }

    #[test]
    fn run_script_passes_positional_arguments() {
        let args = vec!["--verbose".to_string(), "my file.txt".to_string()];
        let out = run_snippet_full(
            "args",
            "printf '%s|' \"$#\" \"$@\"\n",
            &[],
            &args,
            None,
            SCRIPT_TIMEOUT,
        );
        // Two arguments, and the spacey one arrives as a single word.
        assert_eq!(out.stdout, "2|--verbose|my file.txt|");
    }

    #[test]
    fn run_script_feeds_stdin() {
        let out = run_snippet_full(
            "stdin",
            "wc -l\n",
            &[],
            &[],
            Some("a\nb\nc\n"),
            SCRIPT_TIMEOUT,
        );
        assert_eq!(out.stdout.trim(), "3");
    }

    #[test]
    fn large_stdin_does_not_deadlock_a_script_that_ignores_it() {
        // 1 MB is far past the pipe buffer: a synchronous write would block.
        let big = "x".repeat(1024 * 1024);
        let out = run_snippet_full("bigstdin", "echo ok\n", &[], &[], Some(&big), SCRIPT_TIMEOUT);
        assert_eq!(out.stdout.trim(), "ok");
        assert!(!out.timed_out);
    }

    #[cfg(unix)]
    #[test]
    fn unread_stdin_does_not_kill_the_process() {
        // The binary sets SIGPIPE back to SIG_DFL (so `bashlings list | head`
        // exits quietly); the test harness does not. Reproduce the binary's
        // disposition here, or this regression stays invisible.
        unsafe { libc::signal(libc::SIGPIPE, libc::SIG_DFL) };

        let big = "x".repeat(4 * 1024 * 1024);
        let out = run_snippet_full("sigpipe", "echo ok\n", &[], &[], Some(&big), SCRIPT_TIMEOUT);

        unsafe { libc::signal(libc::SIGPIPE, libc::SIG_IGN) };
        assert_eq!(out.stdout.trim(), "ok");
    }

    #[test]
    fn timeout_is_configurable() {
        let start = Instant::now();
        let out = run_snippet_full(
            "timeout",
            "sleep 30\n",
            &[],
            &[],
            None,
            Duration::from_millis(300),
        );
        assert!(out.timed_out);
        assert_eq!(out.exit_code, 124);
        assert!(
            start.elapsed() < Duration::from_secs(5),
            "custom timeout ignored: {:?}",
            start.elapsed()
        );
    }

    // ─── run_full / sandbox isolation ──────────────────────────────────

    #[test]
    fn destructive_script_cannot_reach_the_workspace() {
        let root = std::env::temp_dir().join(format!(
            "bashlings-workspace-test-{}-destructive",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("exercises")).unwrap();
        let canary = root.join("canary.txt");
        std::fs::write(&canary, "keep me").unwrap();

        let exercise = root.join("exercises").join("boom.sh");
        std::fs::write(
            &exercise,
            "rm -rf *\necho boom\n# @test:stdout: boom\n# @test:exit: 0\n",
        )
        .unwrap();

        let report = run_full(&exercise, &root).unwrap();

        assert!(report.all_passed(), "script itself should still succeed");
        assert!(canary.is_file(), "`rm -rf *` must not reach the workspace");
        assert!(exercise.is_file(), "the exercise file must survive its own run");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn stderr_names_the_exercise_not_the_sandbox_copy() {
        let root = std::env::temp_dir().join(format!(
            "bashlings-workspace-test-{}-stderr",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("exercises")).unwrap();
        let exercise = root.join("exercises").join("typo.sh");
        std::fs::write(&exercise, "eko hi\n# @test:exit: 0\n").unwrap();

        let report = run_full(&exercise, &root).unwrap();

        assert!(
            report.stderr.contains("exercises/typo.sh"),
            "stderr should point at the exercise: {:?}",
            report.stderr
        );
        assert!(
            !report.stderr.contains("sandbox"),
            "stderr should not leak the sandbox path: {:?}",
            report.stderr
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    /// Build a throwaway workspace with one exercise and run it end to end.
    fn run_exercise_source(tag: &str, source: &str) -> (PathBuf, TestReport) {
        let root = std::env::temp_dir().join(format!(
            "bashlings-workspace-test-{}-{}",
            std::process::id(),
            tag
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("exercises")).unwrap();
        let exercise = root.join("exercises").join(format!("{tag}.sh"));
        std::fs::write(&exercise, source).unwrap();
        let report = run_full(&exercise, &root).unwrap();
        (root, report)
    }

    #[test]
    fn setup_args_and_stdin_reach_the_script() {
        let (root, report) = run_exercise_source(
            "argsin",
            "printf '%s %s\\n' \"$1\" \"$(cat)\"\n\
             # @setup:args: salom\n\
             # @setup:stdin:\n\
             # |dunyo\n\
             # @test:stdout: salom dunyo\n",
        );
        assert!(report.all_passed(), "{:?}", report.results);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn filesystem_assertions_run_against_the_sandbox() {
        let (root, report) = run_exercise_source(
            "fsassert",
            "cp data/in.txt out.txt\n\
             chmod 0640 out.txt\n\
             rm data/in.txt\n\
             rmdir data\n\
             # @setup:file: data/in.txt\n\
             # |olma\n\
             # @test:file-content: out.txt :: olma\n\
             # @test:file-contains: out.txt :: lm\n\
             # @test:file-missing: data/in.txt\n\
             # @test:perm: out.txt 0640\n\
             # @test:tree: out.txt\n",
        );
        assert!(report.all_passed(), "{:?}", report.results);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn setup_timeout_overrides_the_default() {
        let start = Instant::now();
        let (root, report) = run_exercise_source(
            "shorttimeout",
            "sleep 30\n# @setup:timeout: 1\n# @test:exit: 0\n",
        );
        assert!(report.timed_out);
        assert!(
            start.elapsed() < Duration::from_secs(5),
            "'@setup:timeout' ignored: {:?}",
            start.elapsed()
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn isolate_home_points_home_into_the_sandbox() {
        let (root, report) = run_exercise_source(
            "isohome",
            "case \"$HOME\" in *isohome*) echo yes ;; *) echo \"$HOME\" ;; esac\n\
             # @setup:isolate-home\n\
             # @test:stdout: yes\n",
        );
        assert!(report.all_passed(), "{:?}", report.results);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn home_is_untouched_without_isolate_home() {
        let real_home = std::env::var("HOME").unwrap_or_default();
        let (root, report) = run_exercise_source(
            "realhome",
            "echo \"$HOME\"\n# @test:stdout-cmd: echo \"$HOME\"\n",
        );
        assert!(report.all_passed(), "{:?}", report.results);
        assert_eq!(report.stdout.trim(), real_home);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn expected_cmd_sees_the_same_cwd_as_the_script() {
        // Regression guard for `# @test:stdout-cmd: pwd`: both sides must run
        // in the sandbox, or the exercise can never pass.
        let root = std::env::temp_dir().join(format!(
            "bashlings-workspace-test-{}-pwd",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("exercises")).unwrap();
        let exercise = root.join("exercises").join("nav.sh");
        std::fs::write(&exercise, "pwd\n# @test:stdout-cmd: pwd\n").unwrap();

        let report = run_full(&exercise, &root).unwrap();

        assert!(report.all_passed(), "{:?}", report.results);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn run_script_does_not_hang_on_background_job() {
        // The background `sleep` inherits stdout, so reading to EOF would block
        // for 30s without the drain grace period + process-group kill.
        let start = Instant::now();
        let out = run_snippet("bgjob", "sleep 30 &\necho done\n", &[]);
        assert_eq!(out.stdout.trim(), "done");
        assert!(
            start.elapsed() < Duration::from_secs(5),
            "leftover background job hung the runner for {:?}",
            start.elapsed()
        );
    }
}
