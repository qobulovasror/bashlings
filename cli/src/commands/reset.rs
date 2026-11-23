//! `bashlings reset <name>` — restore an exercise to its original state.
//!
//! The exercise goes back to pending and its hints start over; the code is
//! restored with `git checkout` of the committed (pristine) file when possible.

use crate::style::Style;
use crate::{info, state, tr};
use anyhow::{anyhow, Result};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn run(name: &str) -> Result<bool> {
    let root = info::find_workspace_root()?;
    let info_data = info::load(&root)?;

    let ex = info_data.find(name).ok_or_else(|| {
        anyhow!(tr!(
            "'{}' nomli mashq topilmadi. `bashlings list` orqali ro'yxatni ko'ring.",
            "exercise '{}' not found. Run `bashlings list` to see them.",
            name
        ))
    })?;

    // Pending again, and the hints start over too.
    state::set_done(&root, name, false)?;
    let _ = state::set_hint_level(&root, name, 0);

    println!();

    let rel = format!("exercises/{}", ex.path);
    if git_restore(&root, &rel) {
        println!(
            "  {} {} — {} ({})",
            "♻".cyan(),
            name.bold(),
            "git checkout".dimmed(),
            tr!("asl kod tiklandi", "original code restored")
        );
        println!();
        return Ok(true);
    }

    // git unavailable or file untracked — only the progress could be reset.
    println!(
        "  {} {} — {}",
        "♻".cyan(),
        name.bold(),
        tr!("pending holatga qaytarildi.", "marked as pending again.")
    );
    println!(
        "  {} {}",
        "⚠".yellow(),
        tr!(
            "Asl kodni to'liq tiklab bo'lmadi (git yo'q yoki fayl kuzatuvda emas).",
            "Could not fully restore original code (no git or file untracked)."
        )
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
