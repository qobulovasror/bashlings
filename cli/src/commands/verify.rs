//! `bashlings verify` — run every exercise in order and stop at the first
//! one that doesn't pass. Passing exercises are shown as a compact ✓ line and
//! recorded as done (same as `run`). The first failing exercise is shown in
//! full detail — that's where the learner is.

use crate::style::Style;
use crate::{commands, info, state, test, tr};
use anyhow::Result;

pub fn run() -> Result<bool> {
    test::warn_if_old_bash();
    let root = info::find_workspace_root()?;
    let info_data = info::load(&root)?;

    println!();
    println!("  {}  ·  {}", "Bashlings".bold().green(), "verify".dimmed());
    println!();

    for ex in &info_data.exercises {
        let path = ex.full_path(&root);
        if !path.is_file() {
            // Treat a missing file as the stopping point.
            return commands::run::run_exercise(&root, ex);
        }

        let report = test::run_full(&path, &root)?;
        if report.all_passed() {
            state::set_done(&root, &ex.name, true)?;
            println!(
                "  {}  {}  {}",
                "✓".green(),
                ex.name.bold(),
                format!("({}/{})", report.passed_count(), report.total()).dimmed()
            );
        } else {
            // Show the stuck exercise in full, from the report we already have —
            // re-running it would double a timeout and could disagree.
            commands::run::print_header(&root, ex);
            commands::run::show_report(&root, ex, &report)?;
            println!(
                "  {} {}",
                "⏸".yellow(),
                tr!(
                    "{} mashqida to'xtadi — tuzating va qayta urinib ko'ring.",
                    "stopped at {} — fix it and try again.",
                    ex.name.bold()
                )
            );
            println!();
            return Ok(false);
        }
    }

    println!();
    println!(
        "  🎉 {}",
        tr!(
            "Barcha mashqlar muvaffaqiyatli o'tdi!",
            "All exercises passed!"
        )
        .bold()
        .green()
    );
    println!();
    Ok(true)
}
