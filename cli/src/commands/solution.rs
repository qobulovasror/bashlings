//! `bashlings solution <name>` — reveal the reference solution, but ONLY
//! after the user's exercise actually passes its tests.
//!
//! This is the rustlings-style guard: solutions live in a hidden `.solutions/`
//! directory, and they can only be inspected via this command once the
//! corresponding exercise script produces the expected output.

use crate::{info, test};
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

    let exercise_path = ex.full_path(&root);
    let solution_path = ex.solution_path(&root);

    if !exercise_path.is_file() {
        return Err(anyhow!(
            "mashq fayli mavjud emas: {}",
            exercise_path.display()
        ));
    }

    // Gate: the user's current code must pass all tests right now.
    let report = test::run_full(&exercise_path)?;
    if !report.all_passed() {
        println!();
        println!(
            "  {} {} — {}",
            "🔒".yellow(),
            "Yechim qulflangan".bold(),
            "avval mashqni yeching".dimmed()
        );
        println!();
        println!(
            "  Joriy holat: {}/{} test o'tdi.",
            report.passed_count().to_string().yellow(),
            report.total()
        );
        println!();
        println!(
            "  {} Maslahat olish:  {}",
            "💡".cyan(),
            format!("bashlings hint {name}").cyan().bold()
        );
        println!(
            "  {} Qayta tekshirish: {}",
            "🔁".cyan(),
            format!("bashlings run {name}").cyan().bold()
        );
        println!();
        return Ok(false);
    }

    if !solution_path.is_file() {
        return Err(anyhow!(
            "yechim fayli mavjud emas: {}",
            solution_path.display()
        ));
    }

    let content = std::fs::read_to_string(&solution_path).map_err(|e| {
        anyhow!(
            "'{}' faylini o'qib bo'lmadi: {e}",
            solution_path.display()
        )
    })?;

    println!();
    println!(
        "  {} {} — {}",
        "🔓".green(),
        "Yechim".bold().green(),
        name.bold()
    );
    println!();

    // Pretty-print the solution body with a left bar.
    for line in content.lines() {
        // Hide the test meta footer — it's noise for someone learning.
        if line.trim_start().starts_with("# @test:") {
            continue;
        }
        if line.trim_start().starts_with("# === TEST META") {
            continue;
        }
        println!("    {} {}", "│".dimmed(), line);
    }

    let rel = solution_path
        .strip_prefix(&root)
        .unwrap_or(&solution_path);
    println!();
    println!(
        "  📄 {}",
        rel.display().to_string().dimmed()
    );
    println!();

    Ok(true)
}
