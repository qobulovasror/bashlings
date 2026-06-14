//! `bashlings progress` — compact overall + per-chapter progress overview.

use crate::style::Style;
use crate::{info, tr};
use anyhow::Result;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize)]
struct JsonChapter {
    chapter: String,
    done: usize,
    total: usize,
}

#[derive(Serialize)]
struct JsonProgress {
    total: usize,
    done: usize,
    chapters: Vec<JsonChapter>,
}

pub fn run(json: bool) -> Result<bool> {
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

    if json {
        let out = JsonProgress {
            total,
            done,
            chapters: by_chapter
                .iter()
                .map(|(chapter, (cdone, ctotal))| JsonChapter {
                    chapter: chapter.clone(),
                    done: *cdone,
                    total: *ctotal,
                })
                .collect(),
        };
        println!("{}", serde_json::to_string_pretty(&out)?);
        return Ok(true);
    }

    println!();
    println!(
        "  {}  ·  {}",
        "Bashlings".bold().green(),
        tr!("umumiy progress", "overall progress")
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
        println!("  🎉 {}", tr!("Hammasi tugadi!", "All done!").bold().green());
    } else {
        let next = info_data
            .exercises
            .iter()
            .find(|e| !e.is_done(&root).unwrap_or(true));
        if let Some(ex) = next {
            println!("  {} {}", tr!("Keyingi:", "Next:"), ex.name.cyan().bold());
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
    let pct = (done * 100).checked_div(total).unwrap_or(0);

    format!(
        "[{}{}] {:>3}%",
        "█".repeat(filled).green(),
        "░".repeat(empty).dimmed(),
        pct
    )
}
