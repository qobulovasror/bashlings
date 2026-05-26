//! `bashlings progress` — compact overall + per-chapter progress overview.

use crate::info;
use anyhow::Result;
use owo_colors::OwoColorize;
use std::collections::BTreeMap;

pub fn run() -> Result<bool> {
    let root = info::find_workspace_root()?;
    let info_data = info::load(&root)?;

    let total = info_data.exercises.len();
    let mut done = 0usize;
    let mut by_chapter: BTreeMap<String, (usize, usize)> = BTreeMap::new();

    for ex in &info_data.exercises {
        let is_done = ex.is_done(&root).unwrap_or(false);
        if is_done {
            done += 1;
        }
        let chapter = ex.chapter.clone().unwrap_or_else(|| "(noma'lum)".into());
        let entry = by_chapter.entry(chapter).or_insert((0, 0));
        entry.1 += 1;
        if is_done {
            entry.0 += 1;
        }
    }

    println!();
    println!(
        "  {}  ·  umumiy progress",
        "Bashlings".bold().green()
    );
    println!();

    println!(
        "  {}  {}/{}",
        progress_bar(done, total, 32),
        done.to_string().bold().green(),
        total
    );
    println!();

    let name_width = by_chapter.keys().map(|s| s.len()).max().unwrap_or(20);

    for (chapter, (cdone, ctotal)) in &by_chapter {
        let status = if cdone == ctotal {
            "✓".green().to_string()
        } else if *cdone == 0 {
            "·".dimmed().to_string()
        } else {
            "▸".yellow().to_string()
        };
        println!(
            "  {}  {:<width$}  {}  {}/{}",
            status,
            chapter,
            progress_bar(*cdone, *ctotal, 14),
            cdone,
            ctotal,
            width = name_width
        );
    }

    println!();

    if done == total {
        println!("  🎉 {}", "Hammasi tugadi!".bold().green());
    } else {
        let next = info_data
            .exercises
            .iter()
            .find(|e| !e.is_done(&root).unwrap_or(true));
        if let Some(ex) = next {
            println!("  Keyingi: {}", ex.name.cyan().bold());
        }
    }
    println!();

    Ok(true)
}

fn progress_bar(done: usize, total: usize, width: usize) -> String {
    let filled = if total == 0 {
        0
    } else {
        ((done as f64 / total as f64) * width as f64).round() as usize
    };
    let filled = filled.min(width);
    let empty = width - filled;
    let pct = if total == 0 { 0 } else { (done * 100) / total };

    format!(
        "[{}{}] {:>3}%",
        "█".repeat(filled).green(),
        "░".repeat(empty).dimmed(),
        pct
    )
}
