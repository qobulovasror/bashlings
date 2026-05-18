use crate::info;
use anyhow::Result;
use owo_colors::OwoColorize;

pub fn run() -> Result<()> {
    let root = info::find_workspace_root()?;
    let info = info::load(&root)?;

    let total = info.exercises.len();
    let mut done = 0usize;

    println!();
    println!(
        "  {}  ·  {} ta mashq",
        "Bashlings".bold().green(),
        total.to_string().bold()
    );
    println!();

    let name_width = info
        .exercises
        .iter()
        .map(|e| e.name.len())
        .max()
        .unwrap_or(10);

    for ex in &info.exercises {
        let is_done = ex.is_done(&root)?;
        if is_done {
            done += 1;
        }

        let status = if is_done {
            "✓".green().to_string()
        } else {
            "·".yellow().to_string()
        };

        let chapter = ex.chapter.as_deref().unwrap_or("");

        println!(
            "  {}  {:<width$}  {}",
            status,
            ex.name.bold(),
            chapter.dimmed(),
            width = name_width
        );
    }

    let percent = if total == 0 {
        0.0
    } else {
        (done as f64 / total as f64) * 100.0
    };

    println!();
    println!(
        "  Progress: {} / {}  ({:.0}%)",
        done.to_string().green().bold(),
        total,
        percent
    );
    println!();

    if done < total {
        let next = info
            .exercises
            .iter()
            .find(|e| !e.is_done(&root).unwrap_or(true));
        if let Some(ex) = next {
            println!("  Keyingi: {}", ex.name.cyan().bold());
            println!(
                "  Fayl:    {}",
                ex.full_path(&root).display().to_string().dimmed()
            );
            println!();
        }
    } else {
        println!("  🎉 {}", "Hammasi tugadi!".bold().green());
        println!();
    }

    Ok(())
}
