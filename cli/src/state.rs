//! Tiny persisted state: tracks how many hint steps the learner has revealed
//! per exercise. Stored at `<workspace>/.bashlings/state.toml` (gitignored).

use crate::tr;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    /// Exercise name → number of hint steps already revealed.
    #[serde(default)]
    pub hints: BTreeMap<String, usize>,
}

fn state_path(root: &Path) -> PathBuf {
    root.join(".bashlings").join("state.toml")
}

/// Load state, returning the default (empty) state when the file is missing
/// or unreadable — state is best-effort, never fatal.
pub fn load(root: &Path) -> State {
    let path = state_path(root);
    match std::fs::read_to_string(&path) {
        Ok(content) => toml::from_str(&content).unwrap_or_default(),
        Err(_) => State::default(),
    }
}

/// Persist state to `<workspace>/.bashlings/state.toml`.
pub fn save(root: &Path, state: &State) -> Result<()> {
    let path = state_path(root);
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).with_context(|| {
            tr!("'{}' katalogini yarata olmadik", "could not create directory '{}'", dir.display())
        })?;
    }
    let content = toml::to_string(state)
        .context(tr!("state'ni serialize qila olmadik", "could not serialize state"))?;
    std::fs::write(&path, content)
        .with_context(|| tr!("'{}' ga yoza olmadik", "could not write to '{}'", path.display()))?;
    Ok(())
}

/// How many hint steps have been revealed for `name` (0 if none yet).
pub fn hint_level(root: &Path, name: &str) -> usize {
    load(root).hints.get(name).copied().unwrap_or(0)
}

/// Record the revealed hint level for `name`.
pub fn set_hint_level(root: &Path, name: &str, level: usize) -> Result<()> {
    let mut state = load(root);
    state.hints.insert(name.to_string(), level);
    save(root, &state)
}
