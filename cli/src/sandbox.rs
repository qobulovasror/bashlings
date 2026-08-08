//! Per-run sandbox: every exercise script executes inside a throwaway directory
//! under `.bashlings/sandbox/`, never in the workspace itself.
//!
//! Three reasons:
//!   - safety      — a stray `rm -rf *` can't reach the repository
//!   - determinism — `ls`, `find`, `wc -l` see a known, empty tree
//!   - content     — exercises may create files, chmod them, make symlinks
//!
//! The directory is removed on `Drop` unless `--keep-sandbox` was passed.

use crate::tr;
use anyhow::{Context, Result};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Sandbox parent, relative to the workspace root. `.bashlings/` is gitignored.
const SANDBOX_DIR: [&str; 2] = [".bashlings", "sandbox"];

static KEEP: AtomicBool = AtomicBool::new(false);
static COUNTER: AtomicUsize = AtomicUsize::new(0);
static SWEPT: AtomicBool = AtomicBool::new(false);

/// `--keep-sandbox`: leave the directory on disk so the learner can inspect it.
pub fn set_keep(keep: bool) {
    KEEP.store(keep, Ordering::Relaxed);
}

pub fn keep() -> bool {
    KEEP.load(Ordering::Relaxed)
}

/// A throwaway working directory for exactly one script run.
///
/// ```text
/// <root>/            .bashlings/sandbox/<name>-<pid>-<n>/
/// ├── <name>.sh      the script copy — outside `work/`, so `ls` never shows it
/// ├── .tmp/          TMPDIR
/// └── work/          the script's cwd; fixtures land here
/// ```
#[derive(Debug)]
pub struct Sandbox {
    root: PathBuf,
    workdir: PathBuf,
    /// Canonical sandbox parent. `Drop` refuses to delete anything outside it.
    guard: PathBuf,
    keep: bool,
}

impl Sandbox {
    /// Create `<root>/.bashlings/sandbox/<name>-<pid>-<n>/` plus `work/` and `.tmp/`.
    ///
    /// Directories left behind by crashed runs are swept once per process.
    pub fn create(workspace_root: &Path, name: &str) -> Result<Self> {
        let mut parent = workspace_root.to_path_buf();
        for part in SANDBOX_DIR {
            parent.push(part);
        }
        fs::create_dir_all(&parent).with_context(|| {
            tr!(
                "sandbox katalogini yarata olmadik: {}",
                "could not create the sandbox directory: {}",
                parent.display()
            )
        })?;

        // Canonical form matters: on macOS `/tmp` and `/var` are symlinks, and the
        // Drop guard compares canonical paths.
        let guard = parent.canonicalize().with_context(|| {
            tr!(
                "sandbox katalogini aniqlab bo'lmadi: {}",
                "could not resolve the sandbox directory: {}",
                parent.display()
            )
        })?;

        sweep_stale_once(&guard);

        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = guard.join(format!("{}-{}-{}", sanitize(name), std::process::id(), n));
        let workdir = root.join("work");
        for dir in [&workdir, &root.join(".tmp")] {
            fs::create_dir_all(dir).with_context(|| {
                tr!(
                    "sandbox yarata olmadik: {}",
                    "could not create the sandbox: {}",
                    dir.display()
                )
            })?;
        }

        Ok(Self {
            root,
            workdir,
            guard,
            keep: keep(),
        })
    }

    /// The script's working directory — what `ls` and relative paths see.
    pub fn workdir(&self) -> &Path {
        &self.workdir
    }

    /// The sandbox as a whole (script copy + `.tmp/` + `work/`).
    #[allow(dead_code)] // used by tests and kept for callers that need the root
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Copy the learner's script into the sandbox and return the copy's path.
    ///
    /// The copy is what runs, so the original file is never mutated by a
    /// misbehaving script. It lands *beside* `work/` rather than in it, so an
    /// exercise that runs `ls` sees only its fixture.
    pub fn install_script(&self, src: &Path) -> Result<PathBuf> {
        let file_name = src.file_name().unwrap_or_else(|| OsStr::new("exercise.sh"));
        let dest = self.root.join(file_name);
        fs::copy(src, &dest).with_context(|| {
            tr!(
                "mashqni sandbox'ga nusxalab bo'lmadi: {}",
                "could not copy the exercise into the sandbox: {}",
                src.display()
            )
        })?;
        Ok(dest)
    }

    /// Environment overrides applied to the script AND to `@test:stdout-cmd`
    /// expected commands — both sides must observe the same world.
    ///
    /// `HOME` is deliberately left alone: exercises that teach `$HOME` should
    /// show the learner's real home directory.
    pub fn env(&self) -> Vec<(String, String)> {
        vec![
            (
                "TMPDIR".to_string(),
                self.root.join(".tmp").display().to_string(),
            ),
            // Deterministic `date` and `sort` without touching LANG (UTF-8 stays).
            ("TZ".to_string(), "UTC".to_string()),
            ("LC_COLLATE".to_string(), "C".to_string()),
            ("BASHLINGS_SANDBOX".to_string(), "1".to_string()),
        ]
    }

    /// `HOME` override for `@setup:isolate-home`: a private, empty directory
    /// inside the sandbox. Opt-in, because an exercise that teaches `$HOME`
    /// should normally show the learner's real one.
    pub fn isolated_home(&self) -> Result<(String, String)> {
        let home = self.root.join(".home");
        fs::create_dir_all(&home).with_context(|| {
            tr!(
                "sandbox HOME katalogini yarata olmadik: {}",
                "could not create the sandbox HOME: {}",
                home.display()
            )
        })?;
        Ok(("HOME".to_string(), home.display().to_string()))
    }

    /// Path worth showing the learner — only when the directory survives the run.
    pub fn kept_path(&self) -> Option<PathBuf> {
        if self.keep {
            Some(self.root.clone())
        } else {
            None
        }
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        if self.keep {
            return;
        }
        // Resolve first, then prove the target sits under the sandbox parent.
        // A script that replaced the directory with a symlink can't redirect us.
        let Ok(real) = self.root.canonicalize() else {
            return;
        };
        if !real.starts_with(&self.guard) {
            eprintln!(
                "{}",
                tr!(
                    "⚠  sandbox '{}' katalogi sandbox ildizidan tashqarida — o'chirilmadi",
                    "⚠  sandbox '{}' resolved outside the sandbox root — not removed",
                    real.display()
                )
            );
            return;
        }
        // An exercise may have chmod'ed a directory to 0500; without this the
        // recursive delete fails and the sandbox leaks.
        restore_write_permissions(&real);
        let _ = fs::remove_dir_all(&real);
    }
}

/// Keep directory names free of `-` so the pid stays unambiguous in [`owner_pid`].
fn sanitize(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    if cleaned.is_empty() {
        "exercise".to_string()
    } else {
        cleaned
    }
}

/// Remove sandboxes orphaned by a crashed run — once per process, before we
/// create our own. Directories owned by a *live* pid (e.g. `bashlings watch`
/// in another terminal) are left untouched.
fn sweep_stale_once(parent: &Path) {
    if SWEPT.swap(true, Ordering::Relaxed) {
        return;
    }
    let Ok(entries) = fs::read_dir(parent) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        if matches!(owner_pid(&path), Some(pid) if pid_alive(pid)) {
            continue;
        }
        restore_write_permissions(&path);
        let _ = fs::remove_dir_all(&path);
    }
}

/// Extract the pid from a `<name>-<pid>-<counter>` directory name.
fn owner_pid(path: &Path) -> Option<u32> {
    let name = path.file_name()?.to_str()?;
    let mut parts = name.rsplit('-');
    let _counter = parts.next()?;
    parts.next()?.parse().ok()
}

#[cfg(unix)]
fn pid_alive(pid: u32) -> bool {
    // Signal 0 performs the permission/existence check without delivering.
    unsafe { libc::kill(pid as i32, 0) == 0 }
}

#[cfg(not(unix))]
fn pid_alive(_pid: u32) -> bool {
    // No cheap liveness probe — err on the side of not deleting.
    true
}

/// Give ourselves `u+rwx` on every directory below `dir` (and on `dir` itself)
/// so `remove_dir_all` can descend. Symlinks are never followed.
#[cfg(unix)]
fn restore_write_permissions(dir: &Path) {
    use std::os::unix::fs::PermissionsExt;

    let Ok(meta) = fs::symlink_metadata(dir) else {
        return;
    };
    if !meta.is_dir() {
        return;
    }
    let mut perms = meta.permissions();
    perms.set_mode(perms.mode() | 0o700);
    let _ = fs::set_permissions(dir, perms);

    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        restore_write_permissions(&entry.path());
    }
}

#[cfg(not(unix))]
fn restore_write_permissions(_dir: &Path) {}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each test needs its own workspace root, otherwise the once-per-process
    /// sweep and the shared counter make them order-dependent.
    fn temp_root(tag: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "bashlings-sandbox-test-{}-{}",
            std::process::id(),
            tag
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        root
    }

    #[test]
    fn create_makes_directory_and_tmp() {
        let root = temp_root("create");
        let sb = Sandbox::create(&root, "intro1").unwrap();
        assert!(sb.root().is_dir());
        assert!(sb.root().join(".tmp").is_dir());
        assert!(sb.root().starts_with(&sb.guard));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn drop_removes_directory() {
        let root = temp_root("drop");
        let path = {
            let sb = Sandbox::create(&root, "intro2").unwrap();
            sb.root().to_path_buf()
        };
        assert!(!path.exists(), "sandbox should be gone after drop");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn drop_removes_read_only_subdirectories() {
        let root = temp_root("readonly");
        let path = {
            let sb = Sandbox::create(&root, "robust1").unwrap();
            let locked = sb.root().join("locked");
            fs::create_dir(&locked).unwrap();
            fs::write(locked.join("x.txt"), "x").unwrap();
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&locked, fs::Permissions::from_mode(0o500)).unwrap();
            }
            sb.root().to_path_buf()
        };
        assert!(!path.exists(), "chmod 0500 subdir must not leak the sandbox");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn kept_sandbox_survives_drop() {
        let root = temp_root("keep");
        let path = {
            // Set the flag on the instance rather than the process-wide `KEEP`,
            // so a parallel test can't observe it.
            let mut sb = Sandbox::create(&root, "intro3").unwrap();
            sb.keep = true;
            let kept = sb.kept_path().expect("kept_path when --keep-sandbox");
            assert_eq!(kept, sb.root());
            kept
        };
        assert!(path.is_dir(), "--keep-sandbox must leave the directory");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn install_script_copies_into_sandbox() {
        let root = temp_root("install");
        let src = root.join("intro1.sh");
        fs::write(&src, "echo hi\n").unwrap();
        let sb = Sandbox::create(&root, "intro1").unwrap();
        let dest = sb.install_script(&src).unwrap();
        assert!(dest.starts_with(sb.root()));
        assert_eq!(fs::read_to_string(&dest).unwrap(), "echo hi\n");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn env_points_tmpdir_into_sandbox() {
        let root = temp_root("env");
        let sb = Sandbox::create(&root, "intro1").unwrap();
        let env = sb.env();
        let tmpdir = env.iter().find(|(k, _)| k == "TMPDIR").unwrap();
        assert!(Path::new(&tmpdir.1).starts_with(sb.root()));
        assert!(env.iter().any(|(k, v)| k == "BASHLINGS_SANDBOX" && v == "1"));
        // HOME must NOT be overridden — `echo $HOME` exercises depend on it.
        assert!(!env.iter().any(|(k, _)| k == "HOME"));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn sanitize_strips_dashes_so_pid_stays_parseable() {
        assert_eq!(sanitize("my-exercise"), "my_exercise");
        assert_eq!(sanitize(""), "exercise");
        assert_eq!(owner_pid(Path::new("/x/intro1-4242-7")), Some(4242));
        assert_eq!(owner_pid(Path::new("/x/nodashes")), None);
    }

    #[test]
    fn live_pid_directories_are_not_swept() {
        // A directory owned by this (very much alive) process must survive.
        let root = temp_root("sweep");
        let parent = root.join(".bashlings").join("sandbox");
        fs::create_dir_all(&parent).unwrap();
        let live = parent.join(format!("other-{}-99", std::process::id()));
        let dead = parent.join("other-999999-99");
        fs::create_dir_all(&live).unwrap();
        fs::create_dir_all(&dead).unwrap();

        // Call the sweep directly — the once-per-process latch may already be set.
        let guard = parent.canonicalize().unwrap();
        SWEPT.store(false, Ordering::Relaxed);
        sweep_stale_once(&guard);

        assert!(live.is_dir(), "live-pid sandbox must be left alone");
        assert!(!dead.exists(), "orphaned sandbox should be swept");
        let _ = fs::remove_dir_all(&root);
    }
}
