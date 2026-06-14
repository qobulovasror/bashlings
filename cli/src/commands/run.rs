use crate::{info, test};
use anyhow::{anyhow, Result};
use crate::style::Style;
use std::path::Path;

/// CLI entry. With a `name`, run that exercise; without one, run the first
/// pending exercise (the learner's current spot).
pub fn run(name: Option<&str>) -> Result<bool> {
    test::warn_if_old_bash();
    let root = info::find_workspace_root()?;
    let info = info::load(&root)?;

    let ex = match name {
        Some(n) => info.find(n).ok_or_else(|| {
            anyhow!("'{n}' nomli mashq topilmadi. `bashlings list` orqali ro'yxatni ko'ring.")
        })?,
        None => match info
            .exercises
            .iter()
            .find(|e| !e.is_done(&root).unwrap_or(false))
        {
            Some(e) => e,
            None => {
                println!();
                println!(
                    "  🎉 {}",
                    "Hammasi tugadi! Tekshirish uchun pending mashq qolmadi.".green()
                );
                println!();
                return Ok(true);
            }
        },
    };

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

    let report = test::run_full(&path, root)?;

    if report.timed_out {
        println!(
            "  {}  {}",
            "⏱".red(),
            format!(
                "Skript {}s ichida tugamadi — to'xtatildi (cheksiz tsikl?).",
                test::SCRIPT_TIMEOUT_SECS
            )
            .red()
        );
        println!();
    }

    if report.results.is_empty() {
        println!(
            "  {}  {}",
            "⚠".yellow(),
            "Test direktivasi yo'q (`# @test:...` qatorlari topilmadi) — tekshirib bo'lmadi."
                .yellow()
        );
        println!(
            "     {}",
            "Mashq fayli oxiriga `# @test:...` qatori bo'lishi kerak.".dimmed()
        );
        println!();
        // Hech narsa tekshirilmadi — buni muvaffaqiyat deb hisoblay olmaymiz,
        // aks holda CI/`test-solutions.sh` soxta "pass" beradi.
        return Ok(false);
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
                "     {:<12} actual:    {}",
                "",
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

        // Auto-strip the marker so subsequent runs (and `list`) treat this as done.
        let removed = info::strip_done_marker(&path)?;
        if removed {
            println!();
            println!(
                "  {} {} marker avto-o'chirildi.",
                "✨".cyan(),
                "`# I AM NOT DONE`".dimmed()
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
            "  {} Maslahat:  {}",
            "💡".yellow(),
            format!("bashlings hint {name}").cyan().bold()
        );
        println!(
            "  {} Yechim:    {} {}",
            "🔒".dimmed(),
            format!("bashlings solution {name}").dimmed(),
            "— test pass bo'lgach ochiladi".dimmed()
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
