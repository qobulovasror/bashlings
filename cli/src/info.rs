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

impl Exercise {
    /// Full filesystem path to the exercise script.
    pub fn full_path(&self, workspace_root: &Path) -> PathBuf {
        workspace_root.join("exercises").join(&self.path)
    }

    /// Full filesystem path to the (hidden) solution script.
    pub fn solution_path(&self, workspace_root: &Path) -> PathBuf {
        workspace_root.join(".solutions").join(&self.path)
    }
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
