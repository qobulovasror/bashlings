//! `bashlings reset <name>` — restore an exercise to its original state.
//!
//! Preferred path: `git checkout` the committed (pristine) file, which brings
//! back BOTH the original broken code AND the `# I AM NOT DONE` marker.
//! Fallback (no git / untracked file): re-insert the marker only.

use crate::{info, state};
use anyhow::{anyhow, Result};
use crate::style::Style;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn run(name: &str) -> Result<bool> {
    let root = info::find_workspace_root()?;
    let info_data = info::load(&root)?;

    let ex = info_data.find(name).ok_or_else(|| {
        anyhow!(
            "'{name}' nomli mashq topilmadi. `bashlings list` orqali ro'yxatni ko'ring."
        )
    })?;

    let path = ex.full_path(&root);

    // Forget any revealed hint steps — a clean reset starts the hints over too.
    let _ = state::set_hint_level(&root, name, 0);

    println!();

    let rel = format!("exercises/{}", ex.path);
    if git_restore(&root, &rel) {
        println!(
            "  {} {} — {} (asl kod + marker tiklandi).",
            "♻".cyan(),
            name.bold(),
            "git checkout".dimmed()
        );
        println!();
        return Ok(true);
    }

    // Fallback: git unavailable or file untracked — restore only the marker.
    let inserted = info::restore_done_marker(&path)?;
    if inserted {
        println!(
            "  {} {} — {} marker qaytarildi.",
            "♻".cyan(),
            name.bold(),
            "`# I AM NOT DONE`".dimmed()
        );
    } else {
        println!(
            "  {} {} — marker allaqachon o'rnatilgan.",
            "ℹ".cyan(),
            name.bold()
        );
    }
    println!(
        "  {} Asl kodni to'liq tiklab bo'lmadi (git yo'q yoki fayl kuzatuvda emas).",
        "⚠".yellow()
    );
    println!();

    Ok(true)
}

/// Run `git -C <root> checkout -- <rel>`. Returns `true` on success.
/// Any failure (not a repo, untracked file, git missing) yields `false`.
fn git_restore(root: &Path, rel: &str) -> bool {
    Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["checkout", "--", rel])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
