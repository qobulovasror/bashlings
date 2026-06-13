use crate::info;
use crate::style::Style;
use anyhow::Result;
use serde::Serialize;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Filter {
    All,
    Pending,
    Done,
}

#[derive(Serialize)]
struct JsonExercise<'a> {
    name: &'a str,
    chapter: &'a str,
    done: bool,
}

#[derive(Serialize)]
struct JsonList<'a> {
    total: usize,
    done: usize,
    exercises: Vec<JsonExercise<'a>>,
}

pub fn run(filter: Filter, json: bool) -> Result<()> {
    let root = info::find_workspace_root()?;
    let info_data = info::load(&root)?;

    if json {
        let mut done = 0usize;
        let mut exercises = Vec::new();
        for ex in &info_data.exercises {
            let is_done = ex.is_done(&root)?;
            if is_done {
                done += 1;
            }
            let include = match filter {
                Filter::All => true,
                Filter::Pending => !is_done,
                Filter::Done => is_done,
            };
            if include {
                exercises.push(JsonExercise {
                    name: &ex.name,
                    chapter: ex.chapter.as_deref().unwrap_or(""),
                    done: is_done,
                });
            }
        }
        let out = JsonList {
            total: info_data.exercises.len(),
            done,
            exercises,
        };
        println!("{}", serde_json::to_string_pretty(&out)?);
        return Ok(());
    }

    let total = info_data.exercises.len();
    let mut done = 0usize;
    let mut shown = 0usize;

    println!();
    println!(
        "  {}  ·  {} ta mashq{}",
        "Bashlings".bold().green(),
        total.to_string().bold(),
        match filter {
            Filter::All => "".into(),
            Filter::Pending => "  (faqat hali tugatilmaganlari)".dimmed().to_string(),
            Filter::Done => "  (faqat tugatilganlari)".dimmed().to_string(),
        }
    );
    println!();

    let name_width = info_data
        .exercises
        .iter()
        .map(|e| e.name.len())
        .max()
        .unwrap_or(10);

    for ex in &info_data.exercises {
        let is_done = ex.is_done(&root)?;
        if is_done {
            done += 1;
        }

        let include = match filter {
            Filter::All => true,
            Filter::Pending => !is_done,
            Filter::Done => is_done,
        };
        if !include {
            continue;
        }
        shown += 1;

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

    if shown == 0 {
        println!(
            "  {}",
            match filter {
                Filter::Pending => "Hech qaysi mashq pending emas — hammasi tugatilgan! 🎉",
                Filter::Done => "Hali bironta ham mashq tugatilmagan.",
                Filter::All => "Mashqlar yo'q.",
            }
            .dimmed()
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

    if done < total && filter != Filter::Done {
        let next = info_data
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
    } else if done == total {
        println!("  🎉 {}", "Hammasi tugadi!".bold().green());
        println!();
    }

    Ok(())
}
