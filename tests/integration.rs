//! Behavioral spec for `striptags`, cross-checked against the npm package.

use striptags::{strip_tags, strip_tags_with, StripTags};

#[test]
fn basic() {
    assert_eq!(strip_tags("<p>Hello</p>"), "Hello");
    assert_eq!(strip_tags("a<b>c</b>d"), "acd");
    assert_eq!(strip_tags("<a href=\"x\">link</a>"), "link");
    assert_eq!(strip_tags("plain text"), "plain text");
    assert_eq!(strip_tags(""), "");
}

#[test]
fn allowed_tags() {
    assert_eq!(strip_tags_with("<p>Hi</p>", &["p"], ""), "<p>Hi</p>");
    assert_eq!(
        strip_tags_with("<a href=\"x\">link</a>", &["a"], ""),
        "<a href=\"x\">link</a>"
    );
    assert_eq!(strip_tags_with("<br/>", &["br"], ""), "<br/>");
    assert_eq!(
        strip_tags_with("<x-custom>z</x-custom>", &["x-custom"], ""),
        "<x-custom>z</x-custom>"
    );
    // matching is against the lower-cased tag name
    assert_eq!(
        strip_tags_with("<DIV>x</DIV>", &["div"], ""),
        "<DIV>x</DIV>"
    );
    assert_eq!(strip_tags_with("<DIV>x</DIV>", &["DIV"], ""), "x");
}

#[test]
fn replacement() {
    assert_eq!(strip_tags_with("<p>Hi</p>", &[], "\n"), "\nHi\n");
    assert_eq!(
        strip_tags_with("<p>x</p><a>y</a>", &["a"], "|"),
        "|x|<a>y</a>"
    );
    assert_eq!(
        strip_tags_with("<img src=\"a.png\">", &[], "[img]"),
        "[img]"
    );
    assert_eq!(
        strip_tags_with("<b>x</b><i>y</i>", &["b"], "*"),
        "<b>x</b>*y*"
    );
}

#[test]
fn comments() {
    assert_eq!(strip_tags("a<!-- comment -->b"), "ab");
    assert_eq!(strip_tags("text <!-- <b>not</b> --> more"), "text  more");
}

#[test]
fn loose_angle_brackets() {
    assert_eq!(strip_tags("1 < 2 and 3 > 2"), "1 < 2 and 3 > 2");
    assert_eq!(strip_tags("a < b"), "a < b");
    assert_eq!(strip_tags("<<>>"), "");
    assert_eq!(strip_tags("<a"), ""); // unterminated tag
    assert_eq!(
        strip_tags_with("unclosed <b>bold", &["b"], ""),
        "unclosed <b>bold"
    );
}

#[test]
fn quotes_in_attributes() {
    // a '>' inside a quoted attribute is dropped (matching the reference)
    assert_eq!(
        strip_tags_with("<p class=\"a>b\">hi</p>", &["p"], ""),
        "<p class=\"ab\">hi</p>"
    );
}

#[test]
fn streaming() {
    let mut s = StripTags::new();
    let mut out = s.feed("<p>chun");
    out.push_str(&s.feed("ked</p> text"));
    assert_eq!(out, "chunked text");

    let mut s = StripTags::with_allowed(&["b"]).replacement("|");
    let mut out = s.feed("a<b>x</b>");
    out.push_str(&s.feed("<i>y</i>"));
    assert_eq!(out, "a<b>x</b>|y|");
}
