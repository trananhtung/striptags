# striptags

[![Crates.io](https://img.shields.io/crates/v/striptags.svg)](https://crates.io/crates/striptags)
[![Documentation](https://docs.rs/striptags/badge.svg)](https://docs.rs/striptags)
[![CI](https://github.com/trananhtung/striptags/actions/workflows/ci.yml/badge.svg)](https://github.com/trananhtung/striptags/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/striptags.svg)](#license)

**Strip HTML tags from a string** — with optional allowed tags and a replacement for
removed tags. A faithful Rust port of the
[`striptags`](https://www.npmjs.com/package/striptags) npm package: the same
plaintext/html/comment state machine, including its handling of comments, quoted
attributes, and stray `<`/`>`. Zero dependencies and `#![no_std]`.

```rust
use striptags::{strip_tags, strip_tags_with};

assert_eq!(strip_tags("<p>Hello <b>world</b></p>"), "Hello world");
assert_eq!(strip_tags_with("<p>Hi</p><a>x</a>", &["a"], ""), "Hi<a>x</a>");
assert_eq!(strip_tags_with("<p>Hi</p>", &[], "\n"), "\nHi\n");
```

## Why striptags?

Pulling readable text out of HTML — for search indexing, previews, plain-text
emails, or light sanitization — is a common need, and the canonical JS implementation
handles the tricky bits (comments, `>` inside quoted attributes, unterminated tags,
`a < b`). This ports it faithfully.

```toml
[dependencies]
striptags = "0.1"
```

> Note: like the original, this is a fast tag remover, **not** a security sanitizer.
> For untrusted HTML where XSS matters, use a dedicated sanitizer such as `ammonia`.

## API

| Item | Purpose |
| --- | --- |
| `strip_tags(html)` | Remove all tags |
| `strip_tags_with(html, allowed, replacement)` | Keep `allowed` tags; replace removed ones |
| `StripTags` | Streaming stripper that keeps state across `feed` calls |

## Behavior

- Allowed tag names are matched against the **lower-cased** tag name, so pass them in
  lower case (`&["p", "a"]`).
- HTML comments (`<!-- … -->`) are removed entirely (no replacement emitted).
- A `>` inside a quoted attribute is ignored; a stray `< ` (with a space) is kept as
  text; an unterminated tag at end of input produces no output.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at
your option.
