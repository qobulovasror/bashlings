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
    List,

    /// Bitta mashqni tekshirish
    Run {
        /// Mashq nomi (masalan: intro1)
        name: String,
    },

    /// Mashqlar saqlanganda avto-tekshirish (birinchi pending mashqqa to'g'rilanadi)
    Watch,

    /// Maslahat ko'rsatish (markdown hint faylini terminalda render qiladi)
    Hint {
        /// Mashq nomi
        name: String,
    },

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
        Commands::List => commands::list::run().map(|_| true),
        Commands::Run { name } => commands::run::run(&name),
        Commands::Watch => commands::watch::run(),
        Commands::Hint { name } => commands::hint::run(&name),
        Commands::Next => commands::next::run(),
    }
}
