//! # striptags — strip HTML tags from a string
//!
//! Remove HTML tags from text, optionally keeping a set of allowed tags and/or
//! replacing removed tags with a string. A faithful Rust port of the
//! [`striptags`](https://www.npmjs.com/package/striptags) npm package — the same
//! `<plaintext>`/`<html>`/`<comment>` state machine, including its handling of
//! comments, quoted attributes, and nested `<`. Zero dependencies and `#![no_std]`.
//!
//! ```
//! use striptags::{strip_tags, strip_tags_with};
//!
//! assert_eq!(strip_tags("<p>Hello <b>world</b></p>"), "Hello world");
//! assert_eq!(strip_tags_with("<p>Hi</p><a>x</a>", &["a"], ""), "Hi<a>x</a>");
//! assert_eq!(strip_tags_with("<p>Hi</p>", &[], "\n"), "\nHi\n");
//! ```
//!
//! For chunked input, [`StripTags`] keeps parser state across calls to
//! [`feed`](StripTags::feed).

#![no_std]
#![doc(html_root_url = "https://docs.rs/striptags/0.1.0")]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

// Compile-test the README's examples as part of `cargo test`.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// Strip all HTML tags from `html`.
///
/// ```
/// assert_eq!(striptags::strip_tags("<a href=\"x\">link</a>"), "link");
/// ```
#[must_use]
pub fn strip_tags(html: &str) -> String {
    strip_tags_with(html, &[], "")
}

/// Strip HTML tags from `html`, keeping any tag whose (lower-cased) name is in
/// `allowed`, and replacing each removed tag with `replacement`.
///
/// Tag names in `allowed` are matched case-insensitively against the lower-cased tag
/// name, so they should be given in lower case (matching the reference package).
///
/// ```
/// assert_eq!(striptags::strip_tags_with("<b>x</b><i>y</i>", &["b"], "*"), "<b>x</b>*y*");
/// ```
#[must_use]
pub fn strip_tags_with(html: &str, allowed: &[&str], replacement: &str) -> String {
    let mut ctx = Context::new();
    run(html, allowed, replacement, &mut ctx)
}

/// A streaming HTML-tag stripper that preserves parser state across chunks, matching
/// the npm package's `init_streaming_mode`.
///
/// ```
/// use striptags::StripTags;
/// let mut s = StripTags::new();
/// let mut out = s.feed("<p>chun");
/// out.push_str(&s.feed("ked</p> text"));
/// assert_eq!(out, "chunked text");
/// ```
#[derive(Debug, Clone)]
pub struct StripTags {
    allowed: Vec<String>,
    replacement: String,
    ctx: Context,
}

impl Default for StripTags {
    fn default() -> Self {
        Self::new()
    }
}

impl StripTags {
    /// A stripper that removes all tags.
    #[must_use]
    pub fn new() -> Self {
        Self {
            allowed: Vec::new(),
            replacement: String::new(),
            ctx: Context::new(),
        }
    }

    /// Keep tags whose lower-cased name is in `allowed`.
    #[must_use]
    pub fn with_allowed(allowed: &[&str]) -> Self {
        Self {
            allowed: allowed.iter().map(|s| (*s).into()).collect(),
            replacement: String::new(),
            ctx: Context::new(),
        }
    }

    /// Set the string that replaces each removed tag.
    #[must_use]
    pub fn replacement(mut self, replacement: &str) -> Self {
        self.replacement = replacement.into();
        self
    }

    /// Process the next chunk of HTML, returning its stripped output. Parser state
    /// (an open tag, comment, or quote) carries over to the next call.
    pub fn feed(&mut self, html: &str) -> String {
        let allowed: Vec<&str> = self.allowed.iter().map(String::as_str).collect();
        run(html, &allowed, &self.replacement, &mut self.ctx)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Plaintext,
    Html,
    Comment,
}

#[derive(Debug, Clone)]
struct Context {
    state: State,
    tag_buffer: String,
    depth: u32,
    in_quote: Option<char>,
}

impl Context {
    fn new() -> Self {
        Self {
            state: State::Plaintext,
            tag_buffer: String::new(),
            depth: 0,
            in_quote: None,
        }
    }
}

fn run(html: &str, allowed: &[&str], replacement: &str, ctx: &mut Context) -> String {
    let mut output = String::new();

    for c in html.chars() {
        match ctx.state {
            State::Plaintext => {
                if c == '<' {
                    ctx.state = State::Html;
                    ctx.tag_buffer.push('<');
                } else {
                    output.push(c);
                }
            }
            State::Html => match c {
                '<' => {
                    // A nested '<' (ignored inside a quote).
                    if ctx.in_quote.is_none() {
                        ctx.depth += 1;
                    }
                }
                '>' => {
                    if ctx.in_quote.is_some() {
                        // ignore '>' inside a quote
                    } else if ctx.depth > 0 {
                        ctx.depth -= 1;
                    } else {
                        ctx.in_quote = None;
                        ctx.state = State::Plaintext;
                        ctx.tag_buffer.push('>');
                        match normalize_tag(&ctx.tag_buffer) {
                            Some(tag) if allowed.iter().any(|t| *t == tag) => {
                                output.push_str(&ctx.tag_buffer);
                            }
                            _ => output.push_str(replacement),
                        }
                        ctx.tag_buffer.clear();
                    }
                }
                '"' | '\'' => {
                    if ctx.in_quote == Some(c) {
                        ctx.in_quote = None;
                    } else if ctx.in_quote.is_none() {
                        ctx.in_quote = Some(c);
                    }
                    ctx.tag_buffer.push(c);
                }
                '-' => {
                    if ctx.tag_buffer == "<!-" {
                        ctx.state = State::Comment;
                    }
                    ctx.tag_buffer.push('-');
                }
                ' ' | '\n' => {
                    if ctx.tag_buffer == "<" {
                        ctx.state = State::Plaintext;
                        output.push_str("< ");
                        ctx.tag_buffer.clear();
                    } else {
                        ctx.tag_buffer.push(c);
                    }
                }
                _ => ctx.tag_buffer.push(c),
            },
            State::Comment => {
                if c == '>' {
                    if ctx.tag_buffer.ends_with("--") {
                        ctx.state = State::Plaintext;
                    }
                    ctx.tag_buffer.clear();
                } else {
                    ctx.tag_buffer.push(c);
                }
            }
        }
    }

    output
}

/// Extract a tag's lower-cased name from a tag buffer (`/<\/?([^\s\/>]+)/`), or
/// `None` if there is no name (e.g. `<>`).
fn normalize_tag(tag_buffer: &str) -> Option<String> {
    let chars: Vec<char> = tag_buffer.chars().collect();
    let lt = chars.iter().position(|&c| c == '<')?;
    let mut i = lt + 1;
    if chars.get(i) == Some(&'/') {
        i += 1;
    }
    let start = i;
    while i < chars.len() && !is_js_whitespace(chars[i]) && chars[i] != '/' && chars[i] != '>' {
        i += 1;
    }
    if i == start {
        return None;
    }
    Some(
        chars[start..i]
            .iter()
            .flat_map(|c| c.to_lowercase())
            .collect(),
    )
}

/// Whitespace per JavaScript's regex `\s` (Rust `White_Space` minus NEL `U+0085`,
/// plus the BOM `U+FEFF`).
fn is_js_whitespace(c: char) -> bool {
    (c.is_whitespace() && c != '\u{0085}') || c == '\u{feff}'
}
