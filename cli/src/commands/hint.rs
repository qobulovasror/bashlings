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

use crate::info;
use anyhow::{anyhow, Context, Result};
use owo_colors::OwoColorize;

pub fn run(name: &str) -> Result<bool> {
    let root = info::find_workspace_root()?;
    let info_data = info::load(&root)?;

    let ex = info_data.find(name).ok_or_else(|| {
        anyhow!(
            "'{name}' nomli mashq topilmadi. `bashlings list` orqali ro'yxatni ko'ring."
        )
    })?;

    let hint_rel = ex.hint.as_deref().ok_or_else(|| {
        anyhow!("'{name}' uchun maslahat e'lon qilinmagan (info.toml da 'hint' yo'q).")
    })?;

    let hint_path = root.join("exercises").join(hint_rel);
    if !hint_path.is_file() {
        return Err(anyhow!(
            "maslahat fayli mavjud emas: {}",
            hint_path.display()
        ));
    }

    let content = std::fs::read_to_string(&hint_path)
        .with_context(|| format!("'{}' faylini o'qib bo'lmadi", hint_path.display()))?;

    render(&content);

    let rel = hint_path.strip_prefix(&root).unwrap_or(&hint_path);
    println!(
        "  {} {}",
        "📄".to_string(),
        rel.display().to_string().dimmed()
    );
    println!();

    Ok(true)
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
