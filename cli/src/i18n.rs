//! Minimal, dependency-free localization. Default Uzbek; English opt-in via
//! `--lang en` or `BASHLINGS_LANG=en`.
//!
//! Strings are localized at the call site with the [`tr!`] macro, so both
//! languages stay visible together and `format!` argument capture keeps working:
//!
//! ```ignore
//! tr!("Yechim qulflangan", "Solution locked")            // -> &'static str
//! tr!("'{}' topilmadi", "'{}' not found", name)          // -> String
//! ```

use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Lang {
    Uz,
    En,
}

impl Lang {
    /// Short code used for locale-suffixed files, e.g. `intro1.hint.en.md`.
    pub fn code(self) -> &'static str {
        match self {
            Lang::Uz => "uz",
            Lang::En => "en",
        }
    }
}

static LANG: AtomicU8 = AtomicU8::new(0); // 0 = Uz, 1 = En

pub fn set(lang: Lang) {
    LANG.store(lang as u8, Ordering::Relaxed);
}

pub fn current() -> Lang {
    if LANG.load(Ordering::Relaxed) == 1 {
        Lang::En
    } else {
        Lang::Uz
    }
}

/// Parse a language tag. Accepts `uz`/`en` and a few common aliases.
pub fn parse(s: &str) -> Option<Lang> {
    match s.trim().to_lowercase().as_str() {
        "uz" | "uz-uz" | "uzbek" | "o'zbek" => Some(Lang::Uz),
        "en" | "en-us" | "en-gb" | "english" => Some(Lang::En),
        _ => None,
    }
}

/// Resolve the active language: explicit `--lang` flag wins, then
/// `BASHLINGS_LANG`, otherwise Uzbek.
pub fn resolve(flag: Option<&str>) -> Lang {
    if let Some(f) = flag {
        if let Some(l) = parse(f) {
            return l;
        }
    }
    if let Ok(env) = std::env::var("BASHLINGS_LANG") {
        if let Some(l) = parse(&env) {
            return l;
        }
    }
    Lang::Uz
}

/// Pick a localized string at the call site. Two literals return `&'static str`;
/// adding `format!` arguments returns `String`.
#[macro_export]
macro_rules! tr {
    ($uz:literal, $en:literal $(,)?) => {
        match $crate::i18n::current() {
            $crate::i18n::Lang::Uz => $uz,
            $crate::i18n::Lang::En => $en,
        }
    };
    ($uz:literal, $en:literal, $($arg:tt)+) => {
        match $crate::i18n::current() {
            $crate::i18n::Lang::Uz => format!($uz, $($arg)+),
            $crate::i18n::Lang::En => format!($en, $($arg)+),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_known_tags() {
        assert_eq!(parse("uz"), Some(Lang::Uz));
        assert_eq!(parse("EN"), Some(Lang::En));
        assert_eq!(parse("english"), Some(Lang::En));
        assert_eq!(parse("fr"), None);
    }

    #[test]
    fn resolve_precedence() {
        // Flag wins over everything.
        assert_eq!(resolve(Some("en")), Lang::En);
        // Unknown flag falls through to default (env not set in test).
        assert_eq!(resolve(Some("zz")), Lang::Uz);
        assert_eq!(resolve(None), Lang::Uz);
    }

    #[test]
    fn tr_switches_on_lang() {
        set(Lang::Uz);
        assert_eq!(tr!("salom", "hello"), "salom");
        assert_eq!(tr!("son {}", "num {}", 5), "son 5");
        set(Lang::En);
        assert_eq!(tr!("salom", "hello"), "hello");
        assert_eq!(tr!("son {}", "num {}", 5), "num 5");
        set(Lang::Uz); // restore default
    }
}
