[![ci-badge][]][ci] [![docs-badge][]][docs]

# lavalink.rs

lavalink.rs is a client implemented in Rust used for communicating with
[lavalink] audio nodes.

This library currently targets Lavalink **v3**.

### Library Implementations

Libraries can be built on top of `lavalink.rs` to provide a more easy experience
with the usage of a Discord library.

Here is a list of known libraries:

- [`serenity-lavalink`], used with the [`serenity`] library.

### Installation

If using `lavalink.rs` directly, add the following to your `Cargo.toml`:

```toml
[dependencies]
lavalink = { git = "https://github.com/zeyla/lavalink.rs" }
```

And the following to your project's `main.rs` or `lib.rs`:

```rust
extern crate lavalink;
```

### License

The library is licensed under the [ISC license][license].

[`serenity`]: https://github.com/serenity-rs/serenity
[`serenity-lavalink`]: https://github.com/serenity-rs/serenity-lavalink
[ci]: https://travis-ci.org/zeyla/lavalink.rs
[ci-badge]: https://travis-ci.org/zeyla/lavalink.rs.svg?branch=master
[docs]: https://docs.zeyla.me/lavalink
[docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg
[lavalink]: https://github.com/Frederikam/Lavalink
[license]: https://github.com/zeyla/lavalink.rs/blob/master/LICENSE.md
