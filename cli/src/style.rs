//! Minimal ANSI styling with a global on/off switch.
//!
//! `owo_colors`' direct methods (`.green()`) always emit color and can't be
//! globally disabled, so colored output leaks into pipes (`bashlings list | cat`).
//! This module provides the same chainable API but honors a global flag set
//! once at startup from TTY / `NO_COLOR` / `CLICOLOR_FORCE` detection.

use std::fmt::{self, Display};
use std::sync::atomic::{AtomicBool, Ordering};

static ENABLED: AtomicBool = AtomicBool::new(true);

/// Enable or disable all ANSI styling globally.
pub fn set_enabled(enabled: bool) {
    ENABLED.store(enabled, Ordering::Relaxed);
}

fn enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

/// A borrowed value plus one SGR (on, off) code pair. Methods take `&self`
/// (like `owo_colors`) so they work on borrowed values without cloning;
/// chaining nests `Painted` wrappers, rendered with correctly ordered codes.
pub struct Painted<'a, T: ?Sized> {
    inner: &'a T,
    on: &'static str,
    off: &'static str,
}

impl<T: Display + ?Sized> Display for Painted<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !enabled() {
            return write!(f, "{}", self.inner);
        }
        write!(f, "\x1b[{}m{}\x1b[{}m", self.on, self.inner, self.off)
    }
}

/// Chainable styling for anything that can be displayed. Mirrors the subset of
/// `owo_colors` the CLI actually uses.
pub trait Style: Display {
    fn paint(&self, on: &'static str, off: &'static str) -> Painted<'_, Self> {
        Painted {
            inner: self,
            on,
            off,
        }
    }
    fn green(&self) -> Painted<'_, Self> {
        self.paint("32", "39")
    }
    fn red(&self) -> Painted<'_, Self> {
        self.paint("31", "39")
    }
    fn yellow(&self) -> Painted<'_, Self> {
        self.paint("33", "39")
    }
    fn cyan(&self) -> Painted<'_, Self> {
        self.paint("36", "39")
    }
    fn bold(&self) -> Painted<'_, Self> {
        self.paint("1", "22")
    }
    fn dimmed(&self) -> Painted<'_, Self> {
        self.paint("2", "22")
    }
    fn italic(&self) -> Painted<'_, Self> {
        self.paint("3", "23")
    }
}

impl<T: Display + ?Sized> Style for T {}

#[cfg(test)]
mod tests {
    use super::*;

    // A single test: the ENABLED flag is global, so toggling it across several
    // parallel tests would race. Keep all assertions sequential here.
    #[test]
    fn styling_respects_global_flag() {
        set_enabled(true);
        assert_eq!("hi".green().to_string(), "\x1b[32mhi\x1b[39m");
        // bold then cyan: cyan outer, bold inner — correctly ordered codes.
        assert_eq!("x".bold().cyan().to_string(), "\x1b[36m\x1b[1mx\x1b[22m\x1b[39m");

        set_enabled(false);
        assert_eq!("hi".green().bold().to_string(), "hi");

        set_enabled(true); // restore default for any other tests
    }
}
