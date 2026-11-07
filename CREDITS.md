# Credits

## The Rust Programming Language

Rusticoder's course follows *The Rust Programming Language* ("the Rust Book")
by Steve Klabnik, Carol Nichols and Chris Krycho, with contributions from the
Rust Community: <https://doc.rust-lang.org/book/> · source:
<https://github.com/rust-lang/book>.

- The chapter order, topics, examples and many explanations in the lessons,
  questions and problems are adapted from the Book.
- The Book's chapter text is bundled unchanged in `content/rust-basics/book/`
  and shown in the app's offline "Read more" panel.
- The `trpl` crate used in Chapter 17 (`content/rust-basics/vendor/trpl-0.3.0`)
  is the Book's companion crate.

The Rust Book is dual-licensed under the MIT License and the Apache License,
Version 2.0. Its licence texts are kept with the bundled copy:
[`LICENSE-MIT`](content/rust-basics/book/LICENSE-MIT),
[`LICENSE-APACHE`](content/rust-basics/book/LICENSE-APACHE) and
[`COPYRIGHT`](content/rust-basics/book/COPYRIGHT).

Rusticoder is an independent project. It is not affiliated with or endorsed by
the Rust Project or the Rust Foundation. "Rust" is a trademark of the Rust
Foundation.

## Vendored crates

The crates in `content/rust-basics/vendor/` (rand, tokio, futures and their
dependencies) are vendored unchanged so Cargo problems build offline. Each keeps
its licence in its own folder (licence files or its `Cargo.toml`); all are available under MIT and/or Apache-2.0.

## Rusticoder

Everything else in this repository is © 2026 someRandomUser2311, released
under the MIT License (see [`LICENSE`](LICENSE)).
