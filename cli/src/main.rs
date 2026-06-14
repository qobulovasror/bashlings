mod commands;
mod i18n;
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
    about = "Interaktiv Bash mashqlar runner'i · Interactive Bash exercises (uz/en)",
    long_about = "Bashlings — rustlings uslubidagi Bash o'rganish CLI'i.\n\
                  Til: default o'zbekcha; inglizcha uchun --lang en yoki BASHLINGS_LANG=en.\n\
                  Language: Uzbek by default; English via --lang en or BASHLINGS_LANG=en."
)]
struct Cli {
    /// Interfeys tili · Interface language: uz | en  [env: BASHLINGS_LANG]
    #[arg(long, global = true, value_name = "uz|en")]
    lang: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Mashqlar va holati · List all exercises and their status
    List {
        /// Faqat tugatilmaganlari · Only pending
        #[arg(long)]
        pending: bool,
        /// Faqat tugatilganlari · Only done
        #[arg(long, conflicts_with = "pending")]
        done: bool,
        /// JSON chiqish · JSON output
        #[arg(long)]
        json: bool,
    },

    /// Mashqni tekshirish (nomsiz — keyingi pending) · Run an exercise
    Run {
        /// Mashq nomi · Exercise name (e.g. intro1)
        name: Option<String>,
    },

    /// Hammasini tartibda, birinchi xatoda to'xtash · Verify all, stop at first failure
    Verify,

    /// Saqlanganda avto-tekshirish · Auto-recheck on save (hotkeys: h/s/r/l/q)
    Watch,

    /// Progressiv maslahat · Progressive hint (next step each call)
    Hint {
        /// Mashq nomi · Exercise name
        name: String,
        /// Barcha bosqichlar · Show all steps at once
        #[arg(long)]
        all: bool,
        /// Bosqichlarni qayta tiklash · Reset revealed steps
        #[arg(long, conflicts_with = "all")]
        reset: bool,
    },

    /// Yechim (faqat test pass'dan keyin) · Show solution (after tests pass)
    Solution {
        /// Mashq nomi · Exercise name
        name: String,
    },

    /// Asl holatga qaytarish · Reset exercise to original state
    Reset {
        /// Mashq nomi · Exercise name
        name: String,
    },

    /// Umumiy + bob bo'yicha progress · Overall and per-chapter progress
    Progress {
        /// JSON chiqish · JSON output
        #[arg(long)]
        json: bool,
    },

    /// Keyingi pending mashq nomi · Print the next pending exercise name
    Next {
        /// JSON chiqish · JSON output
        #[arg(long)]
        json: bool,
    },

    /// Shell completion skripti · Generate shell completions
    Completions {
        /// Shell: bash | zsh | fish | powershell | elvish
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
    i18n::set(i18n::resolve(cli.lang.as_deref()));

    match dispatch(cli) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(err) => {
            eprintln!("{}: {err:#}", tr!("Xato", "Error"));
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
