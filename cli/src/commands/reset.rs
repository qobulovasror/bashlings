//! `bashlings reset <name>` — restore the `# I AM NOT DONE` marker so the
//! exercise reverts to its "not yet solved" state.
//!
//! Note: this does NOT restore the original broken code. To fully restore an
//! exercise file, use `git checkout exercises/<path>`.

use crate::info;
use anyhow::{anyhow, Result};
use owo_colors::OwoColorize;

pub fn run(name: &str) -> Result<bool> {
    let root = info::find_workspace_root()?;
    let info_data = info::load(&root)?;

    let ex = info_data.find(name).ok_or_else(|| {
        anyhow!(
            "'{name}' nomli mashq topilmadi. `bashlings list` orqali ro'yxatni ko'ring."
        )
    })?;

    let path = ex.full_path(&root);
    let inserted = info::restore_done_marker(&path)?;

    if inserted {
        println!();
        println!(
            "  {} {} — {} marker qaytarildi.",
            "♻".cyan(),
            name.bold(),
            "`# I AM NOT DONE`".dimmed()
        );
        println!(
            "  {} Original kodni tiklash uchun: {}",
            "💡".yellow(),
            format!("git checkout exercises/{}", ex.path).cyan()
        );
    } else {
        println!();
        println!(
            "  {} {} — marker allaqachon o'rnatilgan.",
            "ℹ".cyan(),
            name.bold()
        );
    }
    println!();

    Ok(true)
}
