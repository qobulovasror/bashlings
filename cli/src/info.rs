use crate::tr;
use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub struct Info {
    pub exercises: Vec<Exercise>,
}

impl Info {
    /// Find an exercise by its `name` field. Returns `None` if not registered.
    pub fn find(&self, name: &str) -> Option<&Exercise> {
        self.exercises.iter().find(|e| e.name == name)
    }
}

#[derive(Debug, Deserialize)]
pub struct Exercise {
    pub name: String,
    pub path: String,
    #[allow(dead_code)]
    pub mode: String,
    #[allow(dead_code)]
    pub hint: Option<String>,
    pub chapter: Option<String>,
}

pub const DONE_MARKER: &str = "# I AM NOT DONE";

impl Exercise {
    /// Full filesystem path to the exercise script.
    pub fn full_path(&self, workspace_root: &Path) -> PathBuf {
        workspace_root.join("exercises").join(&self.path)
    }

    /// Full filesystem path to the (hidden) solution script.
    pub fn solution_path(&self, workspace_root: &Path) -> PathBuf {
        workspace_root.join(".solutions").join(&self.path)
    }

    /// True when the `# I AM NOT DONE` marker is absent (as a standalone line).
    pub fn is_done(&self, workspace_root: &Path) -> Result<bool> {
        let path = self.full_path(workspace_root);
        let content = std::fs::read_to_string(&path)
            .with_context(|| tr!("'{}' faylini o'qib bo'lmadi", "could not read file '{}'", path.display()))?;
        Ok(!has_marker_line(&content))
    }

    /// True when the exercise file STILL has the `# I AM NOT DONE` marker.
    #[allow(dead_code)]
    pub fn has_marker(&self, workspace_root: &Path) -> Result<bool> {
        let path = self.full_path(workspace_root);
        let content = std::fs::read_to_string(&path)
            .with_context(|| tr!("'{}' faylini o'qib bo'lmadi", "could not read file '{}'", path.display()))?;
        Ok(has_marker_line(&content))
    }
}

/// Marker check that matches the marker AS A STANDALONE LINE only.
/// This avoids false positives when the string appears inside a description
/// comment (e.g. "Tuzating va `# I AM NOT DONE` qatorini o'chiring").
pub fn has_marker_line(content: &str) -> bool {
    content.lines().any(|l| l.trim() == DONE_MARKER)
}

/// Pure-string transformation: strip the standalone `# I AM NOT DONE` line.
/// Returns `Some(new_content)` if a change was made, `None` otherwise.
/// Preserves trailing newline state.
pub fn strip_done_marker_str(content: &str) -> Option<String> {
    if !has_marker_line(content) {
        return None;
    }
    let trailing_nl = content.ends_with('\n');
    let new_content: String = content
        .lines()
        .filter(|l| l.trim() != DONE_MARKER)
        .collect::<Vec<_>>()
        .join("\n");
    Some(if trailing_nl {
        format!("{new_content}\n")
    } else {
        new_content
    })
}

/// Pure-string transformation: insert `# I AM NOT DONE` near the top, after
/// any shebang + leading comment header block. Idempotent.
pub fn restore_done_marker_str(content: &str) -> Option<String> {
    if has_marker_line(content) {
        return None;
    }

    let mut lines: Vec<&str> = content.lines().collect();
    let mut insert_at = lines.len();
    let mut seen_header = false;
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            seen_header = true;
            continue;
        }
        if seen_header && trimmed.is_empty() {
            insert_at = i + 1;
            break;
        }
        if seen_header && !trimmed.is_empty() {
            insert_at = i;
            break;
        }
    }

    let trailing_nl = content.ends_with('\n');
    lines.insert(insert_at, DONE_MARKER);
    if insert_at + 1 < lines.len() && !lines[insert_at + 1].is_empty() {
        lines.insert(insert_at + 1, "");
    }
    let mut new_content = lines.join("\n");
    if trailing_nl {
        new_content.push('\n');
    }
    Some(new_content)
}

/// Strip the `# I AM NOT DONE` line (if present) from a file. No-op if absent.
/// Returns `true` if a change was written.
pub fn strip_done_marker(path: &Path) -> Result<bool> {
    let content = std::fs::read_to_string(path)
        .with_context(|| tr!("'{}' faylini o'qib bo'lmadi", "could not read file '{}'", path.display()))?;
    let Some(new_content) = strip_done_marker_str(&content) else {
        return Ok(false);
    };
    std::fs::write(path, new_content)
        .with_context(|| tr!("'{}' ga yoza olmadik", "could not write to '{}'", path.display()))?;
    Ok(true)
}

/// Insert the `# I AM NOT DONE` marker into a file. Idempotent — no-op if
/// the marker is already present. Returns `true` if a change was written.
pub fn restore_done_marker(path: &Path) -> Result<bool> {
    let content = std::fs::read_to_string(path)
        .with_context(|| tr!("'{}' faylini o'qib bo'lmadi", "could not read file '{}'", path.display()))?;
    let Some(new_content) = restore_done_marker_str(&content) else {
        return Ok(false);
    };
    std::fs::write(path, new_content)
        .with_context(|| tr!("'{}' ga yoza olmadik", "could not write to '{}'", path.display()))?;
    Ok(true)
}

/// Load `exercises/info.toml` from the discovered workspace root.
pub fn load(workspace_root: &Path) -> Result<Info> {
    let path = workspace_root.join("exercises").join("info.toml");
    let content = std::fs::read_to_string(&path)
        .with_context(|| tr!("'{}' faylini o'qib bo'lmadi", "could not read file '{}'", path.display()))?;
    let info: Info = toml::from_str(&content)
        .with_context(|| tr!("'{}' tahlilida xato", "error parsing '{}'", path.display()))?;
    Ok(info)
}

/// Walk up from the current directory until we find `exercises/info.toml`.
pub fn find_workspace_root() -> Result<PathBuf> {
    let mut current = std::env::current_dir()
        .context(tr!("joriy katalogni o'qib bo'lmadi", "could not read current directory"))?;
    loop {
        if current.join("exercises").join("info.toml").is_file() {
            return Ok(current);
        }
        if !current.pop() {
            return Err(anyhow!(tr!(
                "workspace topilmadi: 'exercises/info.toml' fayli yo'q. \
                 bashlings ni repo ichidan ishga tushiring.",
                "workspace not found: no 'exercises/info.toml'. \
                 Run bashlings from inside the repo."
            )));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─── has_marker_line ───────────────────────────────────────────────

    #[test]
    fn has_marker_line_finds_standalone() {
        let content = "#!/bin/bash\n\n# I AM NOT DONE\n\necho hi\n";
        assert!(has_marker_line(content));
    }

    #[test]
    fn has_marker_line_finds_with_leading_space() {
        let content = "#!/bin/bash\n   # I AM NOT DONE\necho hi\n";
        assert!(has_marker_line(content));
    }

    #[test]
    fn has_marker_line_ignores_marker_in_description() {
        // This is the bug we fixed — substring match would falsely flag this.
        let content = "# Tuzating va `# I AM NOT DONE` qatorini o'chiring.\necho hi\n";
        assert!(!has_marker_line(content));
    }

    #[test]
    fn has_marker_line_empty_input() {
        assert!(!has_marker_line(""));
    }

    #[test]
    fn has_marker_line_no_marker_anywhere() {
        let content = "#!/bin/bash\necho hi\n";
        assert!(!has_marker_line(content));
    }

    // ─── strip_done_marker_str ────────────────────────────────────────

    #[test]
    fn strip_removes_standalone_line() {
        let input = "#!/bin/bash\n\n# I AM NOT DONE\n\necho hi\n";
        let out = strip_done_marker_str(input).unwrap();
        assert!(!has_marker_line(&out));
        assert!(out.contains("echo hi"));
    }

    #[test]
    fn strip_preserves_trailing_newline() {
        let with_nl = "# I AM NOT DONE\necho x\n";
        let without_nl = "# I AM NOT DONE\necho x";
        assert!(strip_done_marker_str(with_nl).unwrap().ends_with('\n'));
        assert!(!strip_done_marker_str(without_nl).unwrap().ends_with('\n'));
    }

    #[test]
    fn strip_noop_when_marker_absent() {
        let input = "#!/bin/bash\necho hi\n";
        assert!(strip_done_marker_str(input).is_none());
    }

    #[test]
    fn strip_does_not_touch_marker_inside_description() {
        let input = "# Tuzating va `# I AM NOT DONE` qatorini o'chiring.\necho hi\n";
        assert!(strip_done_marker_str(input).is_none());
    }

    #[test]
    fn strip_handles_multiple_marker_lines() {
        let input = "# I AM NOT DONE\necho a\n# I AM NOT DONE\necho b\n";
        let out = strip_done_marker_str(input).unwrap();
        assert!(!has_marker_line(&out));
        assert!(out.contains("echo a"));
        assert!(out.contains("echo b"));
    }

    // ─── restore_done_marker_str ──────────────────────────────────────

    #[test]
    fn restore_inserts_after_header_comments() {
        let input = "#!/bin/bash\n# MASHQ: intro1\n# DARAJA: ★\n\necho hi\n";
        let out = restore_done_marker_str(input).unwrap();
        assert!(has_marker_line(&out));
        // Marker should be placed before "echo hi"
        let marker_idx = out.find(DONE_MARKER).unwrap();
        let echo_idx = out.find("echo hi").unwrap();
        assert!(marker_idx < echo_idx);
    }

    #[test]
    fn restore_idempotent_when_marker_present() {
        let input = "#!/bin/bash\n\n# I AM NOT DONE\n\necho hi\n";
        assert!(restore_done_marker_str(input).is_none());
    }

    #[test]
    fn restore_preserves_trailing_newline() {
        let with_nl = "#!/bin/bash\necho hi\n";
        let without_nl = "#!/bin/bash\necho hi";
        assert!(restore_done_marker_str(with_nl).unwrap().ends_with('\n'));
        assert!(!restore_done_marker_str(without_nl).unwrap().ends_with('\n'));
    }

    #[test]
    fn restore_into_empty_file() {
        let out = restore_done_marker_str("").unwrap();
        assert!(has_marker_line(&out));
    }

    // ─── round-trip ───────────────────────────────────────────────────

    #[test]
    fn strip_after_restore_returns_close_to_original() {
        // Round-trip: restore then strip. Result should not have marker.
        let original = "#!/bin/bash\necho hi\n";
        let restored = restore_done_marker_str(original).unwrap();
        assert!(has_marker_line(&restored));
        let stripped = strip_done_marker_str(&restored).unwrap();
        assert!(!has_marker_line(&stripped));
        // Echo line should survive intact.
        assert!(stripped.contains("echo hi"));
    }
}
