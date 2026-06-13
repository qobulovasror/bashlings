mod commands;
mod info;
mod state;
mod style;
mod test;

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use std::io;
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
        /// Natijani JSON ko'rinishida chiqarish
        #[arg(long)]
        json: bool,
    },

    /// Bitta mashqni tekshirish (nomsiz — birinchi pending mashq)
    Run {
        /// Mashq nomi (masalan: intro1). Berilmasa — keyingi pending mashq.
        name: Option<String>,
    },

    /// Barcha mashqlarni tartibda tekshirish, birinchi xatoda to'xtash
    Verify,

    /// Mashqlar saqlanganda avto-tekshirish (interaktiv hotkeys: h/s/r/q)
    Watch,

    /// Maslahat ko'rsatish — har chaqiruvda keyingi bosqich ochiladi
    Hint {
        /// Mashq nomi
        name: String,
        /// Barcha bosqichlarni birato'la ko'rsatish
        #[arg(long)]
        all: bool,
        /// Ochilgan maslahat bosqichlarini qayta tiklash
        #[arg(long, conflicts_with = "all")]
        reset: bool,
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
    Progress {
        /// Natijani JSON ko'rinishida chiqarish
        #[arg(long)]
        json: bool,
    },

    /// Birinchi pending mashq nomini chiqarish (CI/skript uchun)
    Next {
        /// Natijani JSON ko'rinishida chiqarish
        #[arg(long)]
        json: bool,
    },

    /// Shell completion skriptini chiqarish (bash/zsh/fish/...)
    Completions {
        /// Shell turi: bash | zsh | fish | powershell | elvish
        shell: Shell,
    },
}

fn main() -> ExitCode {
    // SIGPIPE'ni default'ga qaytarish — `bashlings list | head` kabi pipeline'da
    // Rust default panic'ini oldini oladi.
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    style::set_enabled(color_enabled());

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

/// Decide whether ANSI color should be emitted: `NO_COLOR` disables,
/// `CLICOLOR_FORCE` (non-zero) forces on, otherwise only when stdout is a TTY.
fn color_enabled() -> bool {
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if let Some(v) = std::env::var_os("CLICOLOR_FORCE") {
        if v != "0" {
            return true;
        }
    }
    stdout_is_tty()
}

#[cfg(unix)]
fn stdout_is_tty() -> bool {
    unsafe { libc::isatty(libc::STDOUT_FILENO) == 1 }
}

#[cfg(not(unix))]
fn stdout_is_tty() -> bool {
    true
}

/// Returns `Ok(true)` for full success, `Ok(false)` when a command finished
/// but reported failure (e.g. failing tests), and `Err` for unrecoverable errors.
fn dispatch(cli: Cli) -> Result<bool> {
    match cli.command {
        Commands::List {
            pending,
            done,
            json,
        } => {
            let filter = if pending {
                commands::list::Filter::Pending
            } else if done {
                commands::list::Filter::Done
            } else {
                commands::list::Filter::All
            };
            commands::list::run(filter, json).map(|_| true)
        }
        Commands::Run { name } => commands::run::run(name.as_deref()),
        Commands::Verify => commands::verify::run(),
        Commands::Watch => commands::watch::run(),
        Commands::Hint { name, all, reset } => commands::hint::run(&name, all, reset),
        Commands::Solution { name } => commands::solution::run(&name),
        Commands::Reset { name } => commands::reset::run(&name),
        Commands::Progress { json } => commands::progress::run(json),
        Commands::Next { json } => commands::next::run(json),
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            clap_complete::generate(shell, &mut cmd, name, &mut io::stdout());
            Ok(true)
        }
    }
}
