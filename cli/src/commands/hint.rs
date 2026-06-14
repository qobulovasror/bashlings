//! `bashlings hint <name>` — render the exercise's hint markdown to terminal.
//!
//! Supports a minimal markdown subset commonly used in hint files:
//!   - `# Heading`     — H1 (bold green + dimmed underline)
//!   - `## Heading`    — H2 (▸ prefix, bold)
//!   - `### Heading`   — H3 (italic)
//!   - `- item` / `* item` — bullet
//!   - `1. item`       — numbered
//!   - `` `code` ``    — inline code (cyan)
//!   - `**bold**`      — bold
//!   - ` ```lang `     — fenced code blocks (left-bar prefix, cyan)
//!
//! Anything else passes through verbatim.

use crate::style::Style;
use crate::{i18n, info, state, tr};
use anyhow::{anyhow, Context, Result};
use std::path::{Path, PathBuf};

/// Progressive hint: each invocation reveals the *next* `## ` step. Pass
/// `show_all` to dump everything, or `reset` to forget the revealed level.
pub fn run(name: &str, show_all: bool, reset: bool) -> Result<bool> {
    let root = info::find_workspace_root()?;
    let info_data = info::load(&root)?;

    let ex = info_data.find(name).ok_or_else(|| {
        anyhow!(tr!(
            "'{}' nomli mashq topilmadi. `bashlings list` orqali ro'yxatni ko'ring.",
            "exercise '{}' not found. Run `bashlings list` to see them.",
            name
        ))
    })?;

    let hint_rel = ex.hint.as_deref().ok_or_else(|| {
        anyhow!(tr!(
            "'{}' uchun maslahat e'lon qilinmagan (info.toml da 'hint' yo'q).",
            "no hint declared for '{}' (missing 'hint' in info.toml).",
            name
        ))
    })?;

    // Faza 2: prefer a locale-suffixed hint (e.g. intro1.hint.en.md), fall
    // back to the default (Uzbek) file.
    let base = root.join("exercises").join(hint_rel);
    let hint_path = localized_hint_path(&base, i18n::current());
    if !hint_path.is_file() {
        return Err(anyhow!(tr!(
            "maslahat fayli mavjud emas: {}",
            "hint file does not exist: {}",
            hint_path.display()
        )));
    }

    if reset {
        state::set_hint_level(&root, name, 0)?;
        println!();
        println!(
            "  {} {} — {}",
            "♻".cyan(),
            name.bold(),
            tr!("maslahat bosqichlari qayta tiklandi.", "hint steps reset.")
        );
        println!();
        return Ok(true);
    }

    let content = std::fs::read_to_string(&hint_path)
        .with_context(|| format!("'{}' faylini o'qib bo'lmadi", hint_path.display()))?;

    let total = split_sections(&content).1.len();

    // No `## ` steps — fall back to rendering the whole file.
    if total == 0 {
        render(&content);
        print_path_footer(&root, &hint_path);
        return Ok(true);
    }

    let level = if show_all {
        total
    } else {
        let next = (state::hint_level(&root, name) + 1).min(total);
        state::set_hint_level(&root, name, next)?;
        next
    };

    let (shown, _) = hint_up_to_level(&content, level);
    render(&shown);

    if level < total {
        println!(
            "  {} {} {}/{} — {}  {}",
            "▸".cyan(),
            tr!("bosqich", "step"),
            level,
            total,
            tr!("keyingisi:", "next:"),
            format!("bashlings hint {name}").cyan().bold()
        );
    } else {
        println!(
            "  {} {} {}/{} — {}",
            "✓".green(),
            tr!("bosqich", "step"),
            level,
            total,
            tr!("barcha maslahatlar ko'rsatildi.", "all hints shown.")
        );
    }
    print_path_footer(&root, &hint_path);

    Ok(true)
}

fn print_path_footer(root: &Path, hint_path: &Path) {
    let rel = hint_path.strip_prefix(root).unwrap_or(hint_path);
    println!("  📄 {}", rel.display().to_string().dimmed());
    println!();
}

/// For English, prefer `<stem>.<lang>.md` (e.g. `intro1.hint.en.md`) when it
/// exists; otherwise use the default file. Uzbek always uses the base file.
fn localized_hint_path(base: &Path, lang: i18n::Lang) -> PathBuf {
    if lang == i18n::Lang::Uz {
        return base.to_path_buf();
    }
    // base ends with ".md" → insert ".<code>" before it.
    if let Some(stem) = base.to_str().and_then(|s| s.strip_suffix(".md")) {
        let candidate = PathBuf::from(format!("{stem}.{}.md", lang.code()));
        if candidate.is_file() {
            return candidate;
        }
    }
    base.to_path_buf()
}

/// Split hint markdown into a preamble (everything before the first `## ` line)
/// and a list of `## ` sections, each including its own header line.
fn split_sections(md: &str) -> (String, Vec<String>) {
    let mut preamble: Vec<&str> = Vec::new();
    let mut sections: Vec<Vec<&str>> = Vec::new();
    for line in md.lines() {
        if line.trim_start().starts_with("## ") {
            sections.push(vec![line]);
        } else if let Some(last) = sections.last_mut() {
            last.push(line);
        } else {
            preamble.push(line);
        }
    }
    (
        preamble.join("\n"),
        sections.into_iter().map(|s| s.join("\n")).collect(),
    )
}

/// Build the markdown to display: preamble + first `level` sections.
/// Returns `(markdown, total_sections)`.
fn hint_up_to_level(md: &str, level: usize) -> (String, usize) {
    let (preamble, sections) = split_sections(md);
    let total = sections.len();
    let mut parts: Vec<String> = Vec::new();
    if !preamble.trim().is_empty() {
        parts.push(preamble);
    }
    for s in sections.into_iter().take(level) {
        parts.push(s);
    }
    (parts.join("\n"), total)
}

fn render(md: &str) {
    println!();
    let mut in_code_block = false;

    for raw_line in md.lines() {
        let line = raw_line.trim_end_matches('\r');

        // Code fence — toggle
        if line.trim_start().starts_with("```") {
            in_code_block = !in_code_block;
            println!();
            continue;
        }

        if in_code_block {
            println!("     {} {}", "│".dimmed(), line.cyan());
            continue;
        }

        // H1
        if let Some(text) = line.strip_prefix("# ") {
            let text = text.trim();
            println!("  {}", text.bold().green());
            let underline_len = text.chars().count().clamp(20, 48);
            println!("  {}", "─".repeat(underline_len).dimmed());
            continue;
        }

        // H2
        if let Some(text) = line.strip_prefix("## ") {
            println!();
            println!("  {}  {}", "▸".cyan().bold(), text.trim().bold());
            continue;
        }

        // H3
        if let Some(text) = line.strip_prefix("### ") {
            println!("     {}", text.trim().italic());
            continue;
        }

        // Empty line
        if line.trim().is_empty() {
            println!();
            continue;
        }

        // Unordered list item
        if let Some(text) = line.strip_prefix("- ").or_else(|| line.strip_prefix("* ")) {
            println!("     • {}", render_inline(text));
            continue;
        }

        // Numbered list (digits + ". " prefix)
        if let Some((num, rest)) = line.split_once(". ") {
            if !num.is_empty() && num.chars().all(|c| c.is_ascii_digit()) {
                println!(
                    "     {} {}",
                    format!("{}.", num).cyan(),
                    render_inline(rest)
                );
                continue;
            }
        }

        // Regular paragraph
        println!("     {}", render_inline(line));
    }

    println!();
}

/// Apply inline styles for `code` and **bold**. Unclosed markers pass through verbatim.
fn render_inline(line: &str) -> String {
    let mut out = String::new();
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '`' {
            let mut buf = String::new();
            let mut closed = false;
            while let Some(&nc) = chars.peek() {
                chars.next();
                if nc == '`' {
                    closed = true;
                    break;
                }
                buf.push(nc);
            }
            if closed {
                out.push_str(&format!("{}", buf.cyan()));
            } else {
                out.push('`');
                out.push_str(&buf);
            }
        } else if c == '*' && chars.peek() == Some(&'*') {
            chars.next(); // consume second *
            let mut buf = String::new();
            let mut closed = false;
            while let Some(&nc) = chars.peek() {
                chars.next();
                if nc == '*' {
                    if chars.peek() == Some(&'*') {
                        chars.next();
                        closed = true;
                        break;
                    }
                    buf.push('*');
                } else {
                    buf.push(nc);
                }
            }
            if closed {
                out.push_str(&format!("{}", buf.bold()));
            } else {
                out.push_str("**");
                out.push_str(&buf);
            }
        } else {
            out.push(c);
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "# Sarlavha\nkirish\n\n## 1-bosqich\nbir\n\n## 2-bosqich\nikki\n\n## 3-bosqich\nuch\n";

    #[test]
    fn split_counts_sections() {
        let (preamble, sections) = split_sections(SAMPLE);
        assert!(preamble.contains("Sarlavha"));
        assert_eq!(sections.len(), 3);
        assert!(sections[0].contains("1-bosqich"));
    }

    #[test]
    fn level_one_shows_only_first_step() {
        let (out, total) = hint_up_to_level(SAMPLE, 1);
        assert_eq!(total, 3);
        assert!(out.contains("1-bosqich"));
        assert!(!out.contains("2-bosqich"));
        assert!(out.contains("Sarlavha")); // preamble always shown
    }

    #[test]
    fn full_level_shows_all_steps() {
        let (out, _) = hint_up_to_level(SAMPLE, 3);
        assert!(out.contains("1-bosqich"));
        assert!(out.contains("3-bosqich"));
    }

    #[test]
    fn no_sections_yields_zero_total() {
        let (_, sections) = split_sections("# Faqat sarlavha\nmatn\n");
        assert_eq!(sections.len(), 0);
    }
}
