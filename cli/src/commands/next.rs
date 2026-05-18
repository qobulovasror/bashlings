//! `bashlings next` — print just the next pending exercise name.
//!
//! Useful for CI scripts and shell loops:
//!
//! ```bash
//! while name=$(bashlings next); do
//!     bashlings run "$name"
//! done
//! ```
//!
//! Exit codes:
//!   - 0 : a pending exercise exists, its name is on stdout
//!   - 1 : everything is done (no output)

use crate::info;
use anyhow::Result;

pub fn run() -> Result<bool> {
    let root = info::find_workspace_root()?;
    let info_data = info::load(&root)?;

    let pending = info_data
        .exercises
        .iter()
        .find(|e| !e.is_done(&root).unwrap_or(false));

    match pending {
        Some(ex) => {
            println!("{}", ex.name);
            Ok(true)
        }
        None => {
            eprintln!("Hammasi tugagan");
            Ok(false)
        }
    }
}
