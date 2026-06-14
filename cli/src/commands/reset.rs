//! `bashlings reset <name>` — restore an exercise to its original state.
//!
//! Preferred path: `git checkout` the committed (pristine) file, which brings
//! back BOTH the original broken code AND the `# I AM NOT DONE` marker.
//! Fallback (no git / untracked file): re-insert the marker only.

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

    let path = ex.full_path(&root);

    // Forget any revealed hint steps — a clean reset starts the hints over too.
    let _ = state::set_hint_level(&root, name, 0);

    println!();

    let rel = format!("exercises/{}", ex.path);
    if git_restore(&root, &rel) {
        println!(
            "  {} {} — {} ({})",
            "♻".cyan(),
            name.bold(),
            "git checkout".dimmed(),
            tr!("asl kod + marker tiklandi", "original code + marker restored")
        );
        println!();
        return Ok(true);
    }

    // Fallback: git unavailable or file untracked — restore only the marker.
    let inserted = info::restore_done_marker(&path)?;
    if inserted {
        println!(
            "  {} {} — {} {}",
            "♻".cyan(),
            name.bold(),
            "`# I AM NOT DONE`".dimmed(),
            tr!("marker qaytarildi.", "marker restored.")
        );
    } else {
        println!(
            "  {} {} — {}",
            "ℹ".cyan(),
            name.bold(),
            tr!("marker allaqachon o'rnatilgan.", "marker already present.")
        );
    }
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
