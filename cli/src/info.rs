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
            .with_context(|| format!("'{}' faylini o'qib bo'lmadi", path.display()))?;
        Ok(!has_marker_line(&content))
    }

    /// True when the exercise file STILL has the `# I AM NOT DONE` marker.
    #[allow(dead_code)]
    pub fn has_marker(&self, workspace_root: &Path) -> Result<bool> {
        let path = self.full_path(workspace_root);
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("'{}' faylini o'qib bo'lmadi", path.display()))?;
        Ok(has_marker_line(&content))
    }
}

/// Marker check that matches the marker AS A STANDALONE LINE only.
/// This avoids false positives when the string appears inside a description
/// comment (e.g. "Tuzating va `# I AM NOT DONE` qatorini o'chiring").
fn has_marker_line(content: &str) -> bool {
    content.lines().any(|l| l.trim() == DONE_MARKER)
}

/// Strip the `# I AM NOT DONE` line (if present) from a file, preserving
/// the rest of the content as-is. No-op when the marker is absent.
pub fn strip_done_marker(path: &Path) -> Result<bool> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("'{}' faylini o'qib bo'lmadi", path.display()))?;
    if !content.contains(DONE_MARKER) {
        return Ok(false);
    }
    let trailing_nl = content.ends_with('\n');
    let new_content: String = content
        .lines()
        .filter(|l| l.trim() != DONE_MARKER)
        .collect::<Vec<_>>()
        .join("\n");
    let new_content = if trailing_nl {
        format!("{new_content}\n")
    } else {
        new_content
    };
    std::fs::write(path, new_content)
        .with_context(|| format!("'{}' ga yoza olmadik", path.display()))?;
    Ok(true)
}

/// Insert the `# I AM NOT DONE` marker near the top of a file (after the
/// shebang/comment header). Idempotent — no-op if the marker is already present.
pub fn restore_done_marker(path: &Path) -> Result<bool> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("'{}' faylini o'qib bo'lmadi", path.display()))?;
    if content.contains(DONE_MARKER) {
        return Ok(false);
    }

    // Find the first blank line after the leading comment block — that's a
    // natural place for the marker.
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
    std::fs::write(path, new_content)
        .with_context(|| format!("'{}' ga yoza olmadik", path.display()))?;
    Ok(true)
}

/// Load `exercises/info.toml` from the discovered workspace root.
pub fn load(workspace_root: &Path) -> Result<Info> {
    let path = workspace_root.join("exercises").join("info.toml");
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("'{}' faylini o'qib bo'lmadi", path.display()))?;
    let info: Info = toml::from_str(&content)
        .with_context(|| format!("'{}' tahlilida xato", path.display()))?;
    Ok(info)
}

/// Walk up from the current directory until we find `exercises/info.toml`.
pub fn find_workspace_root() -> Result<PathBuf> {
    let mut current = std::env::current_dir().context("joriy katalogni o'qib bo'lmadi")?;
    loop {
        if current.join("exercises").join("info.toml").is_file() {
            return Ok(current);
        }
        if !current.pop() {
            return Err(anyhow!(
                "workspace topilmadi: 'exercises/info.toml' fayli yo'q. \
                 bashlings ni repo ichidan ishga tushiring."
            ));
        }
    }
}
