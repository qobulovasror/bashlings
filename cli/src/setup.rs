//! `# @setup:...` directives — the fixture an exercise runs against.
//!
//! Everything is created inside the run's [`Sandbox`], before the script starts:
//!
//! ```text
//! # @setup:fixture: nav3.fixture          — copy exercises/<chapter>/nav3.fixture/ in
//! # @setup:mkdir: data/sub
//! # @setup:file: data/a.txt               — content follows on `# |` lines
//! # |olma
//! # |anor
//! # @setup:perm: data/a.txt 0600
//! # @setup:symlink: link.txt :: data/a.txt
//! # @setup:env: LOG_LEVEL=debug
//! # @setup:args: --verbose "my file.txt"    — becomes $1, $2, $@
//! # @setup:stdin:                           — content follows on `# |` lines
//! # |1
//! # |2
//! # @setup:timeout: 30                      — seconds; default 10
//! # @setup:isolate-home                     — HOME points inside the sandbox
//! ```
//!
//! Unknown kinds warn and are ignored, matching `# @test:` behaviour.

use crate::sandbox::Sandbox;
use crate::tr;
use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::{Component, Path, PathBuf};

/// One fixture action, applied in declaration order.
#[derive(Debug, Clone, PartialEq)]
pub enum Step {
    /// Copy a directory that sits next to the exercise file into the sandbox.
    Fixture(String),
    Mkdir(String),
    File { path: String, content: String },
    Perm { path: String, mode: u32 },
    Symlink { link: String, target: String },
}

/// Everything `# @setup:` asks for in one exercise.
#[derive(Debug, Default, PartialEq)]
pub struct SetupPlan {
    pub steps: Vec<Step>,
    /// Extra environment for the script (and for `@test:stdout-cmd`).
    pub env: Vec<(String, String)>,
    /// Positional parameters — `$1`, `$@`, `$#`.
    pub args: Vec<String>,
    /// Text fed to the script's stdin; `None` means `/dev/null`.
    pub stdin: Option<String>,
    /// Wall-clock limit override, in seconds.
    pub timeout_secs: Option<u64>,
    /// Point `HOME` at a private directory inside the sandbox.
    pub isolate_home: bool,
}

#[cfg(test)]
impl SetupPlan {
    fn is_empty(&self) -> bool {
        *self == SetupPlan::default()
    }
}

/// Upper bound for `@setup:timeout` — a typo must not hang CI for an hour.
const MAX_TIMEOUT_SECS: u64 = 300;

/// What an open `# |` block is collecting content for.
#[derive(Debug)]
enum Block {
    File(String),
    Stdin,
}

/// Text of a `# |...` continuation line, verbatim (leading spaces preserved).
///
/// Only a `|` immediately after `#` (or after exactly one space) counts, so an
/// ordinary comment like `# ls | wc -l` is never mistaken for content.
fn continuation_text(raw_line: &str) -> Option<&str> {
    let after_hash = raw_line.trim_start().strip_prefix('#')?;
    let rest = after_hash.strip_prefix(' ').unwrap_or(after_hash);
    rest.strip_prefix('|')
}

/// Parse the `# @setup:` directives out of an exercise file.
pub fn parse(content: &str) -> Result<SetupPlan> {
    let mut plan = SetupPlan::default();
    // Open `# |` content block and the lines collected so far.
    let mut open_block: Option<(Block, Vec<String>)> = None;

    for (idx, raw_line) in content.lines().enumerate() {
        let lineno = idx + 1;

        if let Some(text) = continuation_text(raw_line) {
            if let Some((_, lines)) = open_block.as_mut() {
                lines.push(text.to_string());
            }
            // A `# |` line outside a block is just a comment — ignore it.
            continue;
        }

        // Any other line closes an open block.
        close_block(&mut open_block, &mut plan);

        let line = raw_line.trim();
        if !line.starts_with('#') {
            continue;
        }
        let rest = line.trim_start_matches('#').trim_start();
        let Some(rest) = rest.strip_prefix("@setup:") else {
            continue;
        };

        // Directives with no `: VALUE` part. Handled before the split so that a
        // genuinely malformed `@setup:mkdir-no-colon` still errors below.
        if rest.trim() == "isolate-home" {
            plan.isolate_home = true;
            continue;
        }

        let Some((kind, value)) = rest.split_once(':') else {
            return Err(anyhow!(tr!(
                "{}-qator: noto'g'ri '@setup:' direktiva — '# @setup:KIND: VALUE' kutilgan",
                "line {}: invalid '@setup:' directive — expected '# @setup:KIND: VALUE'",
                lineno
            )));
        };
        let kind = kind.trim();
        let value = value.trim();

        match kind {
            "fixture" => plan.steps.push(Step::Fixture(value.to_string())),
            "mkdir" => plan.steps.push(Step::Mkdir(value.to_string())),
            "file" => open_block = Some((Block::File(value.to_string()), Vec::new())),
            "stdin" => open_block = Some((Block::Stdin, Vec::new())),
            "args" => plan.args = split_args(value, lineno)?,
            "timeout" => {
                let secs: u64 = value.parse().with_context(|| {
                    tr!(
                        "{}-qator: '@setup:timeout' soniyada son bo'lishi kerak: '{}'",
                        "line {}: '@setup:timeout' must be a number of seconds: '{}'",
                        lineno,
                        value
                    )
                })?;
                if secs == 0 || secs > MAX_TIMEOUT_SECS {
                    return Err(anyhow!(tr!(
                        "{}-qator: '@setup:timeout' 1..{} oralig'ida bo'lsin",
                        "line {}: '@setup:timeout' must be between 1 and {}",
                        lineno,
                        MAX_TIMEOUT_SECS
                    )));
                }
                plan.timeout_secs = Some(secs);
            }
            "perm" => {
                // The mode never contains a space, so the last field is unambiguous
                // even when the path does.
                let (path, mode) = value.rsplit_once(' ').ok_or_else(|| {
                    anyhow!(tr!(
                        "{}-qator: '@setup:perm' formati — '<yo'l> <0755>'",
                        "line {}: '@setup:perm' format is '<path> <0755>'",
                        lineno
                    ))
                })?;
                plan.steps.push(Step::Perm {
                    path: path.trim().to_string(),
                    mode: parse_mode(mode.trim(), lineno)?,
                });
            }
            "symlink" => {
                // `::` rather than a space: file names with spaces are course material.
                let (link, target) = value.split_once("::").ok_or_else(|| {
                    anyhow!(tr!(
                        "{}-qator: '@setup:symlink' formati — '<link> :: <nishon>'",
                        "line {}: '@setup:symlink' format is '<link> :: <target>'",
                        lineno
                    ))
                })?;
                plan.steps.push(Step::Symlink {
                    link: link.trim().to_string(),
                    target: target.trim().to_string(),
                });
            }
            "env" => {
                let (key, val) = value.split_once('=').ok_or_else(|| {
                    anyhow!(tr!(
                        "{}-qator: '@setup:env' formati — 'KALIT=qiymat'",
                        "line {}: '@setup:env' format is 'KEY=value'",
                        lineno
                    ))
                })?;
                plan.env.push((key.trim().to_string(), val.to_string()));
            }
            other => {
                eprintln!(
                    "{}",
                    tr!(
                        "⚠  {}-qator: noma'lum direktiva @setup:{} (e'tibordan chetda)",
                        "⚠  line {}: unknown directive @setup:{} (ignored)",
                        lineno,
                        other
                    )
                );
            }
        }
    }

    close_block(&mut open_block, &mut plan);
    Ok(plan)
}

/// Turn an open `# |` block into a file step or the stdin text. A block with no
/// `# |` lines yields empty content.
fn close_block(open: &mut Option<(Block, Vec<String>)>, plan: &mut SetupPlan) {
    let Some((block, lines)) = open.take() else {
        return;
    };
    let content = if lines.is_empty() {
        String::new()
    } else {
        format!("{}\n", lines.join("\n"))
    };
    match block {
        Block::File(path) => plan.steps.push(Step::File { path, content }),
        Block::Stdin => plan.stdin = Some(content),
    }
}

/// Split `@setup:args` the way a shell would: whitespace separates, `'` and `"`
/// group, `\` escapes the next character.
fn split_args(value: &str, lineno: usize) -> Result<Vec<String>> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut started = false;
    let mut quote: Option<char> = None;
    let mut chars = value.chars();

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                // Inside single quotes a backslash is literal, as in the shell.
                if quote == Some('\'') {
                    current.push(c);
                } else {
                    match chars.next() {
                        Some(escaped) => current.push(escaped),
                        None => {
                            return Err(anyhow!(tr!(
                                "{}-qator: '@setup:args' teskari chiziq bilan tugadi",
                                "line {}: '@setup:args' ends with a trailing backslash",
                                lineno
                            )))
                        }
                    }
                }
                started = true;
            }
            '\'' | '"' => {
                match quote {
                    Some(q) if q == c => quote = None,
                    Some(_) => current.push(c),
                    None => quote = Some(c),
                }
                started = true;
            }
            c if c.is_whitespace() && quote.is_none() => {
                if started {
                    args.push(std::mem::take(&mut current));
                    started = false;
                }
            }
            c => {
                current.push(c);
                started = true;
            }
        }
    }

    if quote.is_some() {
        return Err(anyhow!(tr!(
            "{}-qator: '@setup:args' da yopilmagan tirnoq",
            "line {}: unclosed quote in '@setup:args'",
            lineno
        )));
    }
    if started {
        args.push(current);
    }
    Ok(args)
}

fn parse_mode(value: &str, lineno: usize) -> Result<u32> {
    u32::from_str_radix(value, 8).with_context(|| {
        tr!(
            "{}-qator: rejim sakkizlik son bo'lishi kerak (masalan 0644): '{}'",
            "line {}: mode must be octal (e.g. 0644): '{}'",
            lineno,
            value
        )
    })
}

impl SetupPlan {
    /// Materialise the fixture inside `sandbox`.
    ///
    /// `exercise_path` locates `@setup:fixture` sources — they live next to the
    /// exercise file, e.g. `exercises/02_navigation/nav3.fixture/`.
    pub fn apply(&self, sandbox: &Sandbox, exercise_path: &Path) -> Result<()> {
        let root = sandbox.workdir();
        for step in &self.steps {
            match step {
                Step::Fixture(name) => {
                    let dir = exercise_path.parent().unwrap_or(Path::new("."));
                    let src = safe_join(dir, name)?;
                    if !src.is_dir() {
                        return Err(anyhow!(tr!(
                            "fixture katalogi topilmadi: {}",
                            "fixture directory not found: {}",
                            src.display()
                        )));
                    }
                    copy_dir_contents(&src, root)?;
                }
                Step::Mkdir(path) => {
                    let dest = safe_join(root, path)?;
                    fs::create_dir_all(&dest).with_context(|| {
                        tr!(
                            "katalog yaratib bo'lmadi: {}",
                            "could not create directory: {}",
                            dest.display()
                        )
                    })?;
                }
                Step::File { path, content } => {
                    let dest = safe_join(root, path)?;
                    if let Some(parent) = dest.parent() {
                        fs::create_dir_all(parent).with_context(|| {
                            tr!(
                                "katalog yaratib bo'lmadi: {}",
                                "could not create directory: {}",
                                parent.display()
                            )
                        })?;
                    }
                    // An earlier `@setup:symlink` must not turn this into a write
                    // to wherever that link points.
                    if is_symlink(&dest) {
                        return Err(anyhow!(tr!(
                            "'@setup:file' symlink ustiga yoza olmaydi: {}",
                            "'@setup:file' refuses to write through a symlink: {}",
                            path
                        )));
                    }
                    fs::write(&dest, content).with_context(|| {
                        tr!(
                            "fayl yozib bo'lmadi: {}",
                            "could not write file: {}",
                            dest.display()
                        )
                    })?;
                }
                Step::Perm { path, mode } => {
                    let dest = safe_join(root, path)?;
                    set_mode(&dest, *mode)?;
                }
                Step::Symlink { link, target } => {
                    let dest = safe_join(root, link)?;
                    if let Some(parent) = dest.parent() {
                        fs::create_dir_all(parent).ok();
                    }
                    make_symlink(target, &dest)?;
                }
            }
        }
        Ok(())
    }
}

/// Join a author-supplied relative path onto `base`, refusing anything that
/// could point outside it. Fixture authors get an error, not a surprise.
fn safe_join(base: &Path, rel: &str) -> Result<PathBuf> {
    let candidate = Path::new(rel);
    let escapes = candidate.is_absolute()
        || candidate
            .components()
            .any(|c| matches!(c, Component::ParentDir | Component::RootDir | Component::Prefix(_)));
    if escapes || rel.is_empty() {
        return Err(anyhow!(tr!(
            "'@setup:' yo'li sandbox ichida bo'lishi kerak (absolut yo'l va '..' mumkin emas): '{}'",
            "'@setup:' paths must stay inside the sandbox (no absolute path, no '..'): '{}'",
            rel
        )));
    }
    Ok(base.join(candidate))
}

fn is_symlink(path: &Path) -> bool {
    fs::symlink_metadata(path)
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
}

/// Copy everything *inside* `src` into `dest` (not `src` itself), preserving
/// directories, file permissions and symlinks.
fn copy_dir_contents(src: &Path, dest: &Path) -> Result<()> {
    let entries = fs::read_dir(src).with_context(|| {
        tr!(
            "fixture katalogini o'qib bo'lmadi: {}",
            "could not read the fixture directory: {}",
            src.display()
        )
    })?;
    for entry in entries {
        let entry = entry?;
        let from = entry.path();
        let to = dest.join(entry.file_name());
        let meta = fs::symlink_metadata(&from)?;

        if meta.file_type().is_symlink() {
            let target = fs::read_link(&from)?;
            make_symlink(&target, &to)?;
        } else if meta.is_dir() {
            fs::create_dir_all(&to)?;
            copy_dir_contents(&from, &to)?;
        } else {
            // `fs::copy` carries the permission bits over, so an executable
            // fixture script stays executable.
            fs::copy(&from, &to).with_context(|| {
                tr!(
                    "fixture faylini nusxalab bo'lmadi: {}",
                    "could not copy the fixture file: {}",
                    from.display()
                )
            })?;
        }
    }
    Ok(())
}

#[cfg(unix)]
fn set_mode(path: &Path, mode: u32) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(mode)).with_context(|| {
        tr!(
            "huquqlarni o'rnatib bo'lmadi: {}",
            "could not set permissions on: {}",
            path.display()
        )
    })
}

#[cfg(not(unix))]
fn set_mode(_path: &Path, _mode: u32) -> Result<()> {
    Err(anyhow!(tr!(
        "'@setup:perm' faqat Unix'da ishlaydi",
        "'@setup:perm' is only supported on Unix"
    )))
}

#[cfg(unix)]
fn make_symlink(target: impl AsRef<Path>, link: &Path) -> Result<()> {
    std::os::unix::fs::symlink(target.as_ref(), link).with_context(|| {
        tr!(
            "symlink yaratib bo'lmadi: {}",
            "could not create the symlink: {}",
            link.display()
        )
    })
}

#[cfg(not(unix))]
fn make_symlink(_target: impl AsRef<Path>, _link: &Path) -> Result<()> {
    Err(anyhow!(tr!(
        "'@setup:symlink' faqat Unix'da ishlaydi",
        "'@setup:symlink' is only supported on Unix"
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sandbox_in(tag: &str) -> (PathBuf, Sandbox) {
        let root = std::env::temp_dir().join(format!(
            "bashlings-setup-test-{}-{}",
            std::process::id(),
            tag
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        let sb = Sandbox::create(&root, tag).unwrap();
        (root, sb)
    }

    // ─── parse ─────────────────────────────────────────────────────────

    #[test]
    fn parses_file_with_continuation_lines() {
        let src = "# @setup:file: data/a.txt\n# |olma\n# |  anor\necho hi\n";
        let plan = parse(src).unwrap();
        assert_eq!(
            plan.steps,
            vec![Step::File {
                path: "data/a.txt".into(),
                // Leading spaces inside the content survive verbatim.
                content: "olma\n  anor\n".into(),
            }]
        );
    }

    #[test]
    fn file_block_ends_at_the_next_non_continuation_line() {
        let src = "# @setup:file: a.txt\n# |one\necho hi\n# |stray\n";
        let plan = parse(src).unwrap();
        assert_eq!(plan.steps.len(), 1);
        match &plan.steps[0] {
            Step::File { content, .. } => assert_eq!(content, "one\n"),
            other => panic!("expected File, got {other:?}"),
        }
    }

    #[test]
    fn file_without_content_lines_is_empty() {
        let plan = parse("# @setup:file: empty.txt\n").unwrap();
        assert_eq!(
            plan.steps,
            vec![Step::File {
                path: "empty.txt".into(),
                content: String::new(),
            }]
        );
    }

    #[test]
    fn ordinary_pipe_comment_is_not_content() {
        // The pipes chapter is full of `# ls | wc -l` style comments.
        let plan = parse("# ls | wc -l\n# @setup:mkdir: data\n").unwrap();
        assert_eq!(plan.steps, vec![Step::Mkdir("data".into())]);
    }

    #[test]
    fn parses_perm_with_spacey_path() {
        let plan = parse("# @setup:perm: my file.txt 0600\n").unwrap();
        assert_eq!(
            plan.steps,
            vec![Step::Perm {
                path: "my file.txt".into(),
                mode: 0o600,
            }]
        );
    }

    #[test]
    fn parses_symlink_and_env() {
        let plan = parse("# @setup:symlink: link.txt :: data/a.txt\n# @setup:env: K=v=1\n").unwrap();
        assert_eq!(
            plan.steps,
            vec![Step::Symlink {
                link: "link.txt".into(),
                target: "data/a.txt".into(),
            }]
        );
        assert_eq!(plan.env, vec![("K".to_string(), "v=1".to_string())]);
    }

    #[test]
    fn malformed_directives_error() {
        assert!(parse("# @setup:perm: nomode\n").is_err());
        assert!(parse("# @setup:symlink: only-one\n").is_err());
        assert!(parse("# @setup:env: NOEQUALS\n").is_err());
        assert!(parse("# @setup:mkdir-no-colon\n").is_err());
        assert!(parse("# @setup:perm: x.txt 0999\n").is_err());
    }

    // ─── args / stdin / timeout / isolate-home ─────────────────────────

    #[test]
    fn args_split_like_a_shell() {
        let plan = parse("# @setup:args: --verbose \"my file.txt\" 'a b' c\\ d\n").unwrap();
        assert_eq!(
            plan.args,
            vec!["--verbose", "my file.txt", "a b", "c d"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn args_edge_cases() {
        assert!(parse("# @setup:args: \"unclosed\n").is_err());
        assert!(parse("# @setup:args: trailing\\\n").is_err());
        // An empty quoted string is still an argument.
        assert_eq!(parse("# @setup:args: '' x\n").unwrap().args, vec!["", "x"]);
        // A backslash inside single quotes stays literal, as in the shell.
        assert_eq!(parse("# @setup:args: 'a\\b'\n").unwrap().args, vec!["a\\b"]);
    }

    #[test]
    fn parses_stdin_block() {
        let plan = parse("# @setup:stdin:\n# |1\n# |2\necho\n").unwrap();
        assert_eq!(plan.stdin.as_deref(), Some("1\n2\n"));
        assert!(plan.steps.is_empty());
    }

    #[test]
    fn stdin_and_file_blocks_do_not_bleed_into_each_other() {
        let plan = parse("# @setup:file: a.txt\n# |file\n# @setup:stdin:\n# |input\n").unwrap();
        assert_eq!(plan.stdin.as_deref(), Some("input\n"));
        assert_eq!(
            plan.steps,
            vec![Step::File {
                path: "a.txt".into(),
                content: "file\n".into(),
            }]
        );
    }

    #[test]
    fn parses_timeout_within_bounds() {
        assert_eq!(parse("# @setup:timeout: 30\n").unwrap().timeout_secs, Some(30));
        assert!(parse("# @setup:timeout: 0\n").is_err());
        assert!(parse("# @setup:timeout: 100000\n").is_err());
        assert!(parse("# @setup:timeout: soon\n").is_err());
    }

    #[test]
    fn isolate_home_needs_no_value() {
        assert!(parse("# @setup:isolate-home\n").unwrap().isolate_home);
        assert!(!parse("echo hi\n").unwrap().isolate_home);
        // A valueless directive must not weaken the missing-colon check.
        assert!(parse("# @setup:mkdir-no-colon\n").is_err());
    }

    #[test]
    fn unknown_kind_is_ignored() {
        let plan = parse("# @setup:teleport: mars\n").unwrap();
        assert!(plan.is_empty());
    }

    #[test]
    fn exercises_without_setup_produce_an_empty_plan() {
        let plan = parse("echo hi\n# @test:stdout: hi\n").unwrap();
        assert!(plan.is_empty());
    }

    // ─── apply ─────────────────────────────────────────────────────────

    #[test]
    fn apply_creates_tree_file_and_perms() {
        let (root, sb) = sandbox_in("apply");
        let plan = parse(
            "# @setup:mkdir: data/sub\n\
             # @setup:file: data/a.txt\n\
             # |olma\n\
             # @setup:perm: data/a.txt 0600\n",
        )
        .unwrap();
        plan.apply(&sb, Path::new("exercises/x/nav.sh")).unwrap();

        assert!(sb.workdir().join("data/sub").is_dir());
        assert_eq!(
            fs::read_to_string(sb.workdir().join("data/a.txt")).unwrap(),
            "olma\n"
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = fs::metadata(sb.workdir().join("data/a.txt"))
                .unwrap()
                .permissions()
                .mode();
            assert_eq!(mode & 0o777, 0o600);
        }
        drop(sb);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn apply_creates_missing_parents_for_files() {
        let (root, sb) = sandbox_in("parents");
        let plan = parse("# @setup:file: deep/er/x.txt\n# |hi\n").unwrap();
        plan.apply(&sb, Path::new("exercises/x/nav.sh")).unwrap();
        assert!(sb.workdir().join("deep/er/x.txt").is_file());
        drop(sb);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn escaping_paths_are_rejected() {
        let (root, sb) = sandbox_in("escape");
        for src in [
            "# @setup:file: ../escape.txt\n# |x\n",
            "# @setup:mkdir: /etc/evil\n",
            "# @setup:symlink: ../link :: target\n",
        ] {
            let plan = parse(src).unwrap();
            assert!(
                plan.apply(&sb, Path::new("exercises/x/nav.sh")).is_err(),
                "should reject: {src}"
            );
        }
        assert!(!root.join("escape.txt").exists());
        drop(sb);
        let _ = fs::remove_dir_all(&root);
    }

    #[cfg(unix)]
    #[test]
    fn file_refuses_to_write_through_a_symlink() {
        let (root, sb) = sandbox_in("symwrite");
        let outside = root.join("outside.txt");
        fs::write(&outside, "original").unwrap();

        let plan = parse(&format!(
            "# @setup:symlink: trap.txt :: {}\n# @setup:file: trap.txt\n# |pwned\n",
            outside.display()
        ))
        .unwrap();

        assert!(plan.apply(&sb, Path::new("exercises/x/nav.sh")).is_err());
        assert_eq!(fs::read_to_string(&outside).unwrap(), "original");
        drop(sb);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn fixture_directory_is_copied_recursively() {
        let (root, sb) = sandbox_in("fixture");
        let exercise_dir = root.join("exercises").join("02_navigation");
        let fixture = exercise_dir.join("nav.fixture");
        fs::create_dir_all(fixture.join("sub")).unwrap();
        fs::write(fixture.join("top.txt"), "top").unwrap();
        fs::write(fixture.join("sub").join("deep.txt"), "deep").unwrap();

        let plan = parse("# @setup:fixture: nav.fixture\n").unwrap();
        plan.apply(&sb, &exercise_dir.join("nav.sh")).unwrap();

        assert_eq!(
            fs::read_to_string(sb.workdir().join("top.txt")).unwrap(),
            "top"
        );
        assert_eq!(
            fs::read_to_string(sb.workdir().join("sub/deep.txt")).unwrap(),
            "deep"
        );
        drop(sb);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn missing_fixture_directory_errors() {
        let (root, sb) = sandbox_in("nofixture");
        let plan = parse("# @setup:fixture: absent.fixture\n").unwrap();
        assert!(plan.apply(&sb, &root.join("nav.sh")).is_err());
        drop(sb);
        let _ = fs::remove_dir_all(&root);
    }
}
