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

use crate::{info, state, tr};
use anyhow::Result;

pub fn run(json: bool) -> Result<bool> {
    let root = info::find_workspace_root()?;
    let info_data = info::load(&root)?;

    let state = state::load(&root);
    let pending = info_data.exercises.iter().find(|e| !state.is_done(&e.name));

    match pending {
        Some(ex) => {
            if json {
                println!("{{\"next\":{}}}", serde_json::to_string(&ex.name)?);
            } else {
                println!("{}", ex.name);
            }
            Ok(true)
        }
        None => {
            if json {
                println!("{{\"next\":null}}");
            } else {
                eprintln!("{}", tr!("Hammasi tugagan", "All done"));
            }
            Ok(false)
        }
    }
}
