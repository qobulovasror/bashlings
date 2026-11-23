//! Tiny persisted state, stored at `<workspace>/.bashlings/state.toml`
//! (gitignored): which exercises passed, and how many hint steps the learner
//! has revealed per exercise.
//!
//! Progress lives here rather than in the exercise files, so the CLI never
//! rewrites a file the learner has open in an editor, and an exercise only
//! counts as done once its tests actually passed.

use crate::tr;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    /// Exercises whose tests passed on their most recent run.
    #[serde(default)]
    pub done: BTreeSet<String>,
    /// Exercise name → number of hint steps already revealed.
    #[serde(default)]
    pub hints: BTreeMap<String, usize>,
}

impl State {
    pub fn is_done(&self, name: &str) -> bool {
        self.done.contains(name)
    }
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

/// Record the outcome of a run: passing marks `name` done, failing un-marks it.
pub fn set_done(root: &Path, name: &str, done: bool) -> Result<()> {
    let mut state = load(root);
    let changed = if done {
        state.done.insert(name.to_string())
    } else {
        state.done.remove(name)
    };
    if changed {
        save(root, &state)?;
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_root(tag: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("bashlings-state-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        root
    }

    #[test]
    fn done_round_trips_alongside_hints() {
        let root = temp_root("roundtrip");
        set_hint_level(&root, "intro1", 2).unwrap();
        set_done(&root, "intro1", true).unwrap();
        set_done(&root, "intro2", true).unwrap();
        set_done(&root, "intro2", false).unwrap();

        let state = load(&root);
        assert!(state.is_done("intro1"));
        assert!(!state.is_done("intro2"));
        assert_eq!(state.hints.get("intro1"), Some(&2));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn hints_only_file_from_older_versions_still_loads() {
        let root = temp_root("legacy");
        std::fs::create_dir_all(root.join(".bashlings")).unwrap();
        std::fs::write(state_path(&root), "[hints]\nintro3 = 1\n").unwrap();

        let state = load(&root);
        assert!(state.done.is_empty());
        assert_eq!(state.hints.get("intro3"), Some(&1));
        let _ = std::fs::remove_dir_all(&root);
    }
}
