//! lint-prose - strip mechanical "signs of AI writing" from repo content.
//!
//! Two classes of tell:
//!   1. Typographic (auto-fixable): em-dash, en-dash, curly quotes, ellipsis
//!      glyph. Replaced with plain ASCII. Checked across all shipped text:
//!      Markdown posts, static HTML, and the prose baked into src/main.rs.
//!      No language here uses these glyphs as syntax, so replacing is safe.
//!   2. Lexical (flagged only): overused AI filler. Checked in prose files
//!      (Markdown + HTML) only, to avoid false positives on code identifiers.
//!      Rewriting these needs judgement, so the tool never edits them.
//!
//! Usage:
//!   cargo lint-prose            # check: report tells, exit 1 if typographic
//!   cargo lint-prose -- --fix   # fix typographic tells in place
//!
//! Ref: https://en.wikipedia.org/wiki/Wikipedia:Signs_of_AI_writing

use std::path::{Path, PathBuf};
use std::process::exit;

use walkdir::WalkDir;

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

// Curly quotes and ellipsis, mapped to ASCII. Dashes are handled separately
// because they also absorb surrounding whitespace.
const SIMPLE: &[(char, &str)] = &[
    ('\u{2018}', "'"),  // '
    ('\u{2019}', "'"),  // '
    ('\u{201C}', "\""), // "
    ('\u{201D}', "\""), // "
    ('\u{2026}', "..."), // ...
];
const DASHES: &[char] = &['\u{2014}', '\u{2013}']; // em, en

fn is_typo_glyph(c: char) -> bool {
    DASHES.contains(&c) || SIMPLE.iter().any(|(g, _)| *g == c)
}

/// em/en-dash with optional surrounding spaces/tabs -> " - " (spaced hyphen).
/// Newlines are never consumed, so lines are not joined.
fn fix_dashes(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if DASHES.contains(&c) {
            while matches!(out.chars().last(), Some(' ') | Some('\t')) {
                out.pop();
            }
            out.push_str(" - ");
            while matches!(chars.peek(), Some(' ') | Some('\t')) {
                chars.next();
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn fix_text(s: &str) -> String {
    let mut t = fix_dashes(s);
    for (glyph, repl) in SIMPLE {
        if t.contains(*glyph) {
            t = t.replace(*glyph, repl);
        }
    }
    t
}

// Lexical filler tells. Matched case-insensitively on whole-word boundaries.
const LEXICAL: &[&str] = &[
    "delve", "tapestry", "testament to", "realm of", "boasts", "nestled",
    "dive into", "game changer", "game-changer", "gamechanger", "moreover",
    "furthermore", "leverage", "seamless", "meticulous", "underscore",
    "foster", "myriad", "plethora", "pivotal", "holistic", "paradigm",
    "embark", "multifaceted", "in conclusion", "it is worth noting",
    "in today's world", "in todays world",
];

fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Case-insensitive whole-word search of `needle` in `hay`.
fn word_match(hay_lower: &str, needle: &str) -> bool {
    let (h, n) = (hay_lower.as_bytes(), needle.as_bytes());
    if n.is_empty() || n.len() > h.len() {
        return false;
    }
    for i in 0..=h.len() - n.len() {
        if &h[i..i + n.len()] == n {
            let before_ok = i == 0 || !is_word_byte(h[i - 1]);
            let after = i + n.len();
            let after_ok = after == h.len() || !is_word_byte(h[after]);
            if before_ok && after_ok {
                return true;
            }
        }
    }
    false
}

fn collect(dir: &str, ext: &str) -> Vec<PathBuf> {
    let base = Path::new(ROOT).join(dir);
    if !base.exists() {
        return Vec::new();
    }
    WalkDir::new(base)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some(ext))
        .collect()
}

fn rel(p: &Path) -> String {
    p.strip_prefix(ROOT).unwrap_or(p).display().to_string()
}

fn main() {
    let fix = std::env::args().any(|a| a == "--fix");

    // Typographic pass: all shipped text. src is limited to main.rs so the tool
    // does not flag the glyphs in its own source.
    let mut typo_targets = collect("content", "md");
    typo_targets.extend(collect("static", "html"));
    typo_targets.push(Path::new(ROOT).join("src/main.rs"));

    let mut typo_hits = 0;
    for p in &typo_targets {
        let Ok(text) = std::fs::read_to_string(p) else { continue };
        if fix {
            let fixed = fix_text(&text);
            if fixed != text {
                std::fs::write(p, fixed).expect("write fixed file");
            }
        } else if text.chars().any(is_typo_glyph) {
            println!("typo: {}", rel(p));
            for (i, line) in text.lines().enumerate() {
                if line.chars().any(is_typo_glyph) {
                    println!("  {}: {}", i + 1, line);
                }
            }
            typo_hits += 1;
        }
    }

    // Lexical pass: prose files only, always report, never edit.
    let mut lex_targets = collect("content", "md");
    lex_targets.extend(collect("static", "html"));
    let mut lex_hits = 0;
    for p in &lex_targets {
        let Ok(text) = std::fs::read_to_string(p) else { continue };
        let mut file_lines = Vec::new();
        for (i, line) in text.lines().enumerate() {
            let lower = line.to_lowercase();
            let hit: Vec<&str> = LEXICAL.iter().copied().filter(|n| word_match(&lower, n)).collect();
            if !hit.is_empty() {
                file_lines.push((i + 1, line.to_string(), hit.join(", ")));
            }
        }
        if !file_lines.is_empty() {
            println!("warn (lexical, review by hand): {}", rel(p));
            for (n, line, tells) in file_lines {
                println!("  {} [{}]: {}", n, tells, line);
            }
            lex_hits += 1;
        }
    }

    if fix {
        println!("fixed typographic tells across content/, static/, src/main.rs.");
        if lex_hits > 0 {
            println!("note: {lex_hits} file(s) still have lexical tells to review.");
        }
        exit(0);
    }

    println!();
    println!("{typo_hits} file(s) with auto-fixable typographic tells (run: cargo lint-prose -- --fix)");
    println!("{lex_hits} file(s) with lexical tells (rewrite by hand)");
    if typo_hits > 0 {
        exit(1);
    }
    println!("clean: no typographic tells.");
}
