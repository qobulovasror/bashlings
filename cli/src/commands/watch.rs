//! `bashlings watch` — re-runs the first pending exercise every time a `.sh`
//! file under `exercises/` is saved. Auto-advances when an exercise passes
//! (the `# I AM NOT DONE` marker is auto-removed by `run::run_exercise`).
//!
//! Also accepts keyboard hotkeys via crossterm raw mode:
//!   - `h`        — show hint for current exercise
//!   - `s`        — show solution (gated on test pass)
//!   - `r`        — re-run current exercise
//!   - `l`        — list compact progress
//!   - `q` / Esc  — quit
//!   - Ctrl+C     — quit

use crate::style::Style;
use crate::{commands, info, tr};
use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use notify::{Event as NotifyEvent, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::io::Write;
use std::sync::mpsc;
use std::time::{Duration, Instant};

enum Action {
    Rerun,
    Hint,
    Solution,
    List,
    Quit,
}

/// RAII guard: enables terminal raw mode on creation and ALWAYS restores
/// cooked mode on drop — including early `?` returns and panics. Without this,
/// an error inside the keypress loop would leave the terminal broken.
struct RawMode;

impl RawMode {
    fn enable() -> Result<Self> {
        enable_raw_mode().context("terminal raw mode'ga o'tkaza olmadik")?;
        Ok(RawMode)
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}

pub fn run() -> Result<bool> {
    crate::test::warn_if_old_bash();
    let root = info::find_workspace_root()?;
    let watch_dir = root.join("exercises");

    let (tx, rx) = mpsc::channel::<notify::Result<NotifyEvent>>();
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

        let _ = std::io::stdout().flush();

        last_name = Some(ex.name.clone());

        let name = ex.name.clone();
        let action = wait_for_action(&rx, &mut last_event)?;
        match action {
            Action::Rerun => {
                // Loop continues — re-render and re-test.
            }
            Action::Quit => {
                println!();
                println!("  👋 {}", tr!("Chiqildi.", "Exited.").dimmed());
                println!();
                return Ok(true);
            }
            Action::Hint => {
                clear_screen();
                // Each `h` press reveals the next progressive hint step.
                let _ = commands::hint::run(&name, false, false);
                pause_until_keypress()?;
            }
            Action::Solution => {
                clear_screen();
                let _ = commands::solution::run(&name);
                pause_until_keypress()?;
            }
            Action::List => {
                clear_screen();
                let _ = commands::progress::run(false);
                pause_until_keypress()?;
            }
        }
    }
}

fn print_header(done: usize, total: usize, advanced: bool) {
    println!();
    println!(
        "  {}  {}",
        "Bashlings".bold().green(),
        tr!("watch rejimi", "watch mode").dimmed()
    );
    println!();

    println!("  {}  {}/{}", progress_bar(done, total), done, total);

    if advanced && done > 0 {
        println!();
        println!(
            "  🎯 {}",
            tr!("Yangi mashqqa o'tdik!", "Moved to a new exercise!")
                .bold()
                .yellow()
        );
    }
}

fn print_footer() {
    println!();
    println!(
        "  👀 {}",
        tr!("Faylni saqlang yoki tugma bosing:", "Save a file or press a key:").dimmed()
    );
    println!(
        "     {}  hint    {}  solution {}    {}  re-run    {}  progress    {}  quit",
        "h".bold().cyan(),
        "s".bold().cyan(),
        "🔒".dimmed(),
        "r".bold().cyan(),
        "l".bold().cyan(),
        "q".bold().cyan()
    );
    println!(
        "     {}",
        tr!(
            "(🔒 = test pass'dan keyin ochiladi)",
            "(🔒 = unlocks after tests pass)"
        )
        .dimmed()
    );
}

fn clear_screen() {
    // ANSI: clear screen + move cursor to row 1 col 1
    print!("\x1B[2J\x1B[1;1H");
    let _ = std::io::stdout().flush();
}

fn celebrate(total: usize) {
    println!();
    println!("  🎉 {}", tr!("Hammasi tugadi!", "All done!").bold().green());
    println!();
    println!(
        "  {}",
        tr!(
            "Siz {} ta mashqning hammasini muvaffaqiyatli yechib chiqdingiz.",
            "You solved all {} exercises. Great work!",
            total.to_string().bold()
        )
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
    let pct = (done * 100).checked_div(total).unwrap_or(0);

    format!(
        "[{}{}]  {}%",
        "█".repeat(filled).green(),
        "░".repeat(empty).dimmed(),
        pct
    )
}

/// Wait for either a `.sh` file change OR a keyboard hotkey.
/// Uses crossterm raw mode just for the wait window — restores cooked mode
/// on every exit path so terminal output stays clean.
fn wait_for_action(
    file_rx: &mpsc::Receiver<notify::Result<NotifyEvent>>,
    last_event: &mut Instant,
) -> Result<Action> {
    let _raw = RawMode::enable()?;
    wait_for_action_inner(file_rx, last_event)
    // `_raw` dropped here — cooked mode restored on every path, errors included.
}

fn wait_for_action_inner(
    file_rx: &mpsc::Receiver<notify::Result<NotifyEvent>>,
    last_event: &mut Instant,
) -> Result<Action> {
    loop {
        // Drain any pending file events first.
        let mut got_file_event = false;
        while let Ok(event) = file_rx.try_recv() {
            if event_is_relevant(event) {
                got_file_event = true;
            }
        }
        if got_file_event {
            let now = Instant::now();
            if now.duration_since(*last_event) >= Duration::from_millis(250) {
                *last_event = now;
                // Coalesce burst from editors (vim, VS Code) that emit several
                // events per save.
                std::thread::sleep(Duration::from_millis(80));
                while file_rx.try_recv().is_ok() {}
                return Ok(Action::Rerun);
            }
        }

        // Then poll keypress with short timeout.
        if event::poll(Duration::from_millis(150))? {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                if modifiers.contains(KeyModifiers::CONTROL) {
                    if let KeyCode::Char('c') = code {
                        return Ok(Action::Quit);
                    }
                }
                match code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(Action::Quit),
                    KeyCode::Char('h') => return Ok(Action::Hint),
                    KeyCode::Char('s') => return Ok(Action::Solution),
                    KeyCode::Char('r') | KeyCode::Enter => return Ok(Action::Rerun),
                    KeyCode::Char('l') | KeyCode::Char('p') => return Ok(Action::List),
                    _ => continue,
                }
            }
        }
    }
}

fn event_is_relevant(event: notify::Result<NotifyEvent>) -> bool {
    let Ok(event) = event else { return false };
    if !matches!(
        event.kind,
        EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
    ) {
        return false;
    }
    event
        .paths
        .iter()
        .any(|p| p.extension().and_then(|s| s.to_str()) == Some("sh"))
}

/// After a non-rerun action (hint, solution, progress), wait for any key
/// before going back to the watch loop. Keeps the output visible.
fn pause_until_keypress() -> Result<()> {
    println!(
        "  {} {}",
        "↩".dimmed(),
        tr!(
            "Davom etish uchun istalgan tugmani bosing...",
            "Press any key to continue..."
        )
        .dimmed()
    );
    let _ = std::io::stdout().flush();
    let _raw = RawMode::enable()?;
    loop {
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }
    }
    Ok(())
    // `_raw` dropped here — raw mode restored even if `poll`/`read` errored.
}
