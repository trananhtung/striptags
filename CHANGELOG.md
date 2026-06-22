# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-22

### Added

- Initial release.
- `strip_tags` / `strip_tags_with` — remove HTML tags, with optional allowed tags
  and a replacement string.
- `StripTags` — a streaming stripper that preserves parser state across chunks.
- Faithful to the `striptags` npm package's plaintext/html/comment state machine,
  including comments, quoted attributes, nested `<`, and the `< ` rule. Zero
  dependencies; `#![no_std]`.

[0.1.0]: https://github.com/trananhtung/striptags/releases/tag/v0.1.0
