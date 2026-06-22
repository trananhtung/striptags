# striptags

[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=flat-square)](#contributors-)

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

## Contributors ✨

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind are welcome — code, docs, bug reports, ideas, reviews! See the [emoji key](https://allcontributors.org/docs/en/emoji-key) for how each contribution is recognized, and open a PR or issue to get involved.

Thanks goes to these wonderful people:

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/trananhtung"><img src="https://avatars.githubusercontent.com/u/30992229?v=4?s=100" width="100px;" alt="Tung Tran"/><br /><sub><b>Tung Tran</b></sub></a><br /><a href="https://github.com/trananhtung/striptags/commits?author=trananhtung" title="Code">💻</a> <a href="#maintenance-trananhtung" title="Maintenance">🚧</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at
your option.
