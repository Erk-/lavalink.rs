[![ci-badge][]][ci] [![license-badge][]][license] [![docs-badge][]][docs] [![rust badge]][rust link]

# lavalink.rs

lavalink.rs is a client implemented in Rust used for communicating with
[lavalink] audio nodes.

This library currently targets Lavalink **v3.3**.

### Library Implementations

Libraries can be built on top of `lavalink.rs` to provide a more easy experience
with the usage of a Discord library.

Here is a list of known libraries:

- [`lavalink-futures`], an async implementation of a Lavalink client.
- [`serenity-lavalink`], used with the [`serenity`] library.

### Installation

This library requires at least Rust 1.31.0

If using `lavalink.rs` directly, add the following to your `Cargo.toml`:

```toml
[dependencies]
lavalink = { git = "https://github.com/dabbotorg/lavalink.rs" }
```

And the following to your project's `main.rs` or `lib.rs`:

```rust
extern crate lavalink;
```

### License

The library is licensed under the [ISC license][license].

[`lavalink-futures`]: https://github.com/zeyla/lavalink-futures
[`serenity`]: https://github.com/serenity-rs/serenity
[`serenity-lavalink`]: https://github.com/serenity-rs/serenity-lavalink
[ci]: https://travis-ci.org/dabbotorg/lavalink.rs
[ci-badge]: https://img.shields.io/travis/dabbotorg/lavalink.rs.svg?style=flat-square
[docs]: https://docs.rs/crate/lavalink
[docs-badge]: https://img.shields.io/badge/docs-online-2020ff.svg?style=flat-square
[lavalink]: https://github.com/Frederikam/Lavalink
[license]: https://github.com/serenity-rs/lavalink.rs/blob/master/LICENSE.md
[license]: https://opensource.org/licenses/ISC
[license-badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
[rust badge]: https://img.shields.io/badge/rust-1.31.0+-93450a.svg?style=flat-square
[rust link]: hhttps://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
