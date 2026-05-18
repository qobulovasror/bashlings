use crate::{info, test};
use anyhow::{anyhow, Result};
use owo_colors::OwoColorize;
use std::path::Path;

const DONE_MARKER: &str = "# I AM NOT DONE";

/// CLI entry: look up `name`, then delegate to [`run_exercise`].
pub fn run(name: &str) -> Result<bool> {
    let root = info::find_workspace_root()?;
    let info = info::load(&root)?;

    let ex = info.find(name).ok_or_else(|| {
        anyhow!(
            "'{name}' nomli mashq topilmadi. `bashlings list` orqali ro'yxatni ko'ring."
        )
    })?;

    run_exercise(&root, ex)
}

/// Execute and verify a single exercise.
///
/// Returns `Ok(true)` when every assertion passed, `Ok(false)` on any failure.
/// Hard I/O errors propagate as `Err`.
pub fn run_exercise(root: &Path, ex: &info::Exercise) -> Result<bool> {
    let name = ex.name.as_str();
    let path = ex.full_path(root);

    if !path.is_file() {
        return Err(anyhow!("fayl mavjud emas: {}", path.display()));
    }

    let rel_path = path.strip_prefix(root).unwrap_or(&path).display().to_string();

    println!();
    println!(
        "  {} {}",
        "Running".dimmed(),
        name.bold().cyan()
    );
    println!("  {} {}", "Fayl:".dimmed(), rel_path.dimmed());
    println!();

    let report = test::run_full(&path)?;

    if report.results.is_empty() {
        println!(
            "  {}  Test direktivasi yo'q (`# @test:...` qatorlari topilmadi).",
            "⚠".yellow()
        );
        println!();
        return Ok(true);
    }

    // Print each assertion result
    for r in &report.results {
        let kind = r.assertion.kind();
        if r.passed {
            let preview = r.assertion.preview();
            println!(
                "  {}  {:<12} {}",
                "✓".green(),
                kind.bold(),
                preview.dimmed()
            );
        } else {
            println!(
                "  {}  {:<12} expected:  {}",
                "✗".red(),
                kind.bold(),
                format_value(&r.expected).red()
            );
            println!(
                "  {}  {:<12} actual:    {}",
                " ",
                " ",
                format_value(&r.actual).red()
            );
        }
    }

    println!();

    let total = report.total();
    let passed = report.passed_count();

    if report.all_passed() {
        println!(
            "  {} {} — {} ({}/{})",
            "✅".green(),
            name.bold(),
            "muvaffaqiyat".green().bold(),
            passed,
            total
        );

        // Friendly nudge to remove the marker so `list` will count it as done.
        let content = std::fs::read_to_string(&path)?;
        if content.contains(DONE_MARKER) {
            println!();
            println!(
                "  {} Endi {} qatorini fayldan o'chiring.",
                "💡".yellow(),
                "`# I AM NOT DONE`".yellow().bold()
            );
        }
        println!();
        Ok(true)
    } else {
        println!(
            "  {} {} — {} ({}/{})",
            "❌".red(),
            name.bold(),
            "xato".red().bold(),
            passed,
            total
        );

        // If the script emitted stderr (e.g., "command not found"), show it.
        if !report.stderr.trim().is_empty() {
            println!();
            println!("  {}", "Skript stderr chiqishi:".dimmed());
            for line in report.stderr.lines() {
                println!("    {}", line.dimmed());
            }
        }

        // Non-zero exit code worth showing if exit assertion failed silently
        if !report
            .results
            .iter()
            .any(|r| matches!(r.assertion, test::Assertion::Exit(_)))
            && report.exit_code != 0
        {
            println!();
            println!(
                "  {} skript exit kodi: {}",
                "ℹ".cyan(),
                report.exit_code.to_string().yellow()
            );
        }

        println!();
        println!(
            "  {} Maslahat: {}",
            "💡".yellow(),
            format!("bashlings hint {name}").cyan().bold()
        );
        println!();
        Ok(false)
    }
}

/// Pretty-print a value for diff output. For multi-line values use a block format.
fn format_value(v: &str) -> String {
    if v.contains('\n') {
        let mut s = String::from("\n");
        for line in v.lines() {
            s.push_str("                    | ");
            s.push_str(line);
            s.push('\n');
        }
        s.trim_end().to_string()
    } else {
        format!("{v:?}")
    }
}
