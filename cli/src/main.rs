mod commands;
mod info;
mod test;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "bashlings",
    version,
    about = "Interaktiv Bash mashqlar runner'i (uzbek)",
    long_about = "Bashlings — rustlings uslubidagi Bash o'rganish CLI'i.\n\
                  Mashqlar 'exercises/' katalogida, har biri '# I AM NOT DONE' markerli."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Barcha mashqlarni va ularning holatini ko'rsatish
    List {
        /// Faqat hali tugatilmagan mashqlarni ko'rsatish
        #[arg(long)]
        pending: bool,
        /// Faqat tugatilgan mashqlarni ko'rsatish
        #[arg(long, conflicts_with = "pending")]
        done: bool,
    },

    /// Bitta mashqni tekshirish
    Run {
        /// Mashq nomi (masalan: intro1)
        name: String,
    },

    /// Mashqlar saqlanganda avto-tekshirish (interaktiv hotkeys: h/s/r/q)
    Watch,

    /// Maslahat ko'rsatish (markdown hint faylini terminalda render qiladi)
    Hint {
        /// Mashq nomi
        name: String,
    },

    /// Yechimni ko'rsatish (faqat mashq pass bo'lgandan keyin)
    Solution {
        /// Mashq nomi
        name: String,
    },

    /// Mashqni boshlang'ich holatga qaytarish (`# I AM NOT DONE` ni qaytaradi)
    Reset {
        /// Mashq nomi
        name: String,
    },

    /// Compact progress overview — umumiy va qism bo'yicha
    Progress,

    /// Birinchi pending mashq nomini chiqarish (CI/skript uchun)
    Next,
}

fn main() -> ExitCode {
    // SIGPIPE'ni default'ga qaytarish — `bashlings list | head` kabi pipeline'da
    // Rust default panic'ini oldini oladi.
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    let cli = Cli::parse();

    match dispatch(cli) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(err) => {
            eprintln!("Xato: {err:#}");
            ExitCode::from(2)
        }
    }
}

/// Returns `Ok(true)` for full success, `Ok(false)` when a command finished
/// but reported failure (e.g. failing tests), and `Err` for unrecoverable errors.
fn dispatch(cli: Cli) -> Result<bool> {
    match cli.command {
        Commands::List { pending, done } => {
            let filter = if pending {
                commands::list::Filter::Pending
            } else if done {
                commands::list::Filter::Done
            } else {
                commands::list::Filter::All
            };
            commands::list::run(filter).map(|_| true)
        }
        Commands::Run { name } => commands::run::run(&name),
        Commands::Watch => commands::watch::run(),
        Commands::Hint { name } => commands::hint::run(&name),
        Commands::Solution { name } => commands::solution::run(&name),
        Commands::Reset { name } => commands::reset::run(&name),
        Commands::Progress => commands::progress::run(),
        Commands::Next => commands::next::run(),
    }
}
