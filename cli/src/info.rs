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

const DONE_MARKER: &str = "# I AM NOT DONE";

impl Exercise {
    /// Full filesystem path to the exercise script.
    pub fn full_path(&self, workspace_root: &Path) -> PathBuf {
        workspace_root.join("exercises").join(&self.path)
    }

    /// True when the `# I AM NOT DONE` marker is absent.
    pub fn is_done(&self, workspace_root: &Path) -> Result<bool> {
        let path = self.full_path(workspace_root);
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("'{}' faylini o'qib bo'lmadi", path.display()))?;
        Ok(!content.contains(DONE_MARKER))
    }
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
