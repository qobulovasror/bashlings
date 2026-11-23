use crate::tr;
use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub struct Info {
    /// Oldest CLI that understands these exercises. The exercises live in a
    /// clone that moves on its own, so an older binary must say so up front
    /// instead of failing them one by one on unknown directives.
    #[serde(default)]
    pub min_cli_version: Option<String>,
    pub exercises: Vec<Exercise>,
}

impl Info {
    /// Find an exercise by its `name` field. Returns `None` if not registered.
    pub fn find(&self, name: &str) -> Option<&Exercise> {
        self.exercises.iter().find(|e| e.name == name)
    }
}

/// `0.1.2` → `[0, 1, 2]`; compares numerically, unlike the raw strings.
fn version_parts(v: &str) -> Vec<u64> {
    v.split('.').map(|p| p.trim().parse().unwrap_or(0)).collect()
}

fn check_min_cli(info: &Info, current: &str) -> Result<()> {
    let Some(min) = info.min_cli_version.as_deref() else {
        return Ok(());
    };
    if version_parts(current) < version_parts(min) {
        return Err(anyhow!(tr!(
            "bu mashqlar bashlings {} yoki yangisini talab qiladi (sizda {}). \
             Yangilang: `brew upgrade bashlings`, install skriptini qayta ishga tushiring \
             yoki `cargo install bashlings`.",
            "these exercises need bashlings {} or newer (you have {}). \
             Update: `brew upgrade bashlings`, re-run the install script, \
             or `cargo install bashlings`.",
            min,
            current
        )));
    }
    Ok(())
}

#[cfg(test)]
mod version_tests {
    use super::*;

    fn info(min: Option<&str>) -> Info {
        Info {
            min_cli_version: min.map(String::from),
            exercises: Vec::new(),
        }
    }

    #[test]
    fn older_cli_is_rejected_newer_accepted() {
        assert!(check_min_cli(&info(Some("0.1.2")), "0.1.1").is_err());
        assert!(check_min_cli(&info(Some("0.1.2")), "0.1.2").is_ok());
        // Numeric, not lexical: 0.10.0 is newer than 0.9.0.
        assert!(check_min_cli(&info(Some("0.9.0")), "0.10.0").is_ok());
        assert!(check_min_cli(&info(None), "0.0.1").is_ok());
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
    check_min_cli(&info, env!("CARGO_PKG_VERSION"))?;
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
