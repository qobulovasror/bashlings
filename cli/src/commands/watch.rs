//! `bashlings watch` — re-runs the first pending exercise every time a `.sh`
//! file under `exercises/` is saved. Auto-advances when an exercise passes
//! and its `# I AM NOT DONE` marker is removed.

use crate::{commands, info};
use anyhow::{Context, Result};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use owo_colors::OwoColorize;
use std::io::Write;
use std::sync::mpsc;
use std::time::{Duration, Instant};

pub fn run() -> Result<bool> {
    let root = info::find_workspace_root()?;
    let watch_dir = root.join("exercises");

    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher: RecommendedWatcher = notify::recommended_watcher(move |res| {
        let _ = tx.send(res);
    })
    .context("notify watcher'ni yarata olmadik")?;

    watcher
        .watch(&watch_dir, RecursiveMode::Recursive)
        .with_context(|| format!("'{}' ni kuzata olmadik", watch_dir.display()))?;

    let mut last_event = Instant::now() - Duration::from_secs(2);
    let mut last_name: Option<String> = None;
    let mut first_iter = true;

    loop {
        // Re-read registry — info.toml or files may have changed.
        let info_data = info::load(&root)?;
        let total = info_data.exercises.len();
        let done = info_data
            .exercises
            .iter()
            .filter(|e| e.is_done(&root).unwrap_or(false))
            .count();

        let pending = info_data
            .exercises
            .iter()
            .find(|e| !e.is_done(&root).unwrap_or(false));

        let Some(ex) = pending else {
            if !first_iter {
                clear_screen();
            }
            celebrate(total);
            return Ok(true);
        };

        let advanced = last_name.as_deref() != Some(ex.name.as_str());

        if !first_iter {
            clear_screen();
        }
        first_iter = false;

        print_header(done, total, advanced);

        // Reuse the same display path as `bashlings run`.
        let _ = commands::run::run_exercise(&root, ex)?;

        print_footer();

        // Make sure output reaches the terminal before we block on the channel.
        let _ = std::io::stdout().flush();

        last_name = Some(ex.name.clone());

        wait_for_change(&rx, &mut last_event)?;
    }
}

fn print_header(done: usize, total: usize, advanced: bool) {
    println!();
    println!(
        "  {}  {}",
        "Bashlings".bold().green(),
        "watch rejimi".dimmed()
    );
    println!();

    println!("  {}  {}/{}", progress_bar(done, total), done, total);

    if advanced && done > 0 {
        println!();
        println!(
            "  {} {}",
            "🎯".to_string(),
            "Yangi mashqqa o'tdik!".bold().yellow()
        );
    }
}

fn print_footer() {
    println!();
    println!(
        "  {} {}",
        "👀".to_string(),
        "Faylni saqlang — avto-tekshiriladi.  (Ctrl+C — chiqish)".dimmed()
    );
}

fn clear_screen() {
    // ANSI: clear screen + move cursor to row 1 col 1
    print!("\x1B[2J\x1B[1;1H");
    let _ = std::io::stdout().flush();
}

fn celebrate(total: usize) {
    println!();
    println!("  {} {}", "🎉".to_string(), "Hammasi tugadi!".bold().green());
    println!();
    println!(
        "  Siz {} ta mashqning hammasini muvaffaqiyatli yechib chiqdingiz.",
        total.to_string().bold()
    );
    println!();
    println!(
        "  Keyingi qadam: {}",
        "2-qism — Advanced Bash Scripting".cyan().bold()
    );
    println!();
}

/// Build a 24-cell progress bar with percentage suffix.
fn progress_bar(done: usize, total: usize) -> String {
    const WIDTH: usize = 24;
    let filled = if total == 0 {
        0
    } else {
        ((done as f64 / total as f64) * WIDTH as f64).round() as usize
    };
    let filled = filled.min(WIDTH);
    let empty = WIDTH - filled;
    let pct = if total == 0 { 0 } else { (done * 100) / total };

    format!(
        "[{}{}]  {}%",
        "█".repeat(filled).green(),
        "░".repeat(empty).dimmed(),
        pct
    )
}

/// Block until at least one `.sh` modify/create/remove event arrives,
/// then coalesce a short burst before returning.
fn wait_for_change(
    rx: &mpsc::Receiver<notify::Result<Event>>,
    last_event: &mut Instant,
) -> Result<()> {
    loop {
        let event = rx.recv().context("watcher kanali yopildi")?;
        let Ok(event) = event else { continue };

        if !matches!(
            event.kind,
            EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
        ) {
            continue;
        }

        let is_sh = event
            .paths
            .iter()
            .any(|p| p.extension().and_then(|s| s.to_str()) == Some("sh"));
        if !is_sh {
            continue;
        }

        let now = Instant::now();
        if now.duration_since(*last_event) < Duration::from_millis(250) {
            continue;
        }
        *last_event = now;

        // Coalesce: editors (vim, VS Code) emit several events per save.
        std::thread::sleep(Duration::from_millis(80));
        while rx.try_recv().is_ok() {}

        return Ok(());
    }
}
