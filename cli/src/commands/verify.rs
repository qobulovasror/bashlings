//! `bashlings verify` — run every exercise in order and stop at the first
//! one that doesn't pass. Passing exercises are shown as a compact ✓ line and
//! their `# I AM NOT DONE` marker is auto-removed (same as `run`). The first
//! failing exercise is shown in full detail — that's where the learner is.

use crate::style::Style;
use crate::{commands, info, test, tr};
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
            // Mark as done so `list`/`progress` agree, then show a compact line.
            let _ = info::strip_done_marker(&path);
            println!(
                "  {}  {}  {}",
                "✓".green(),
                ex.name.bold(),
                format!("({}/{})", report.passed_count(), report.total()).dimmed()
            );
        } else {
            // Re-run through the full display path for the stuck exercise.
            commands::run::run_exercise(&root, ex)?;
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
