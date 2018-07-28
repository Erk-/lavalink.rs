//! lavalink.rs is a client implemented in Rust used for communicating with
//! [lavalink] audio nodes.
//!
//! ### Library Implementations
//!
//! Libraries can be built on top of `lavalink.rs` to provide a more easy experience
//! with the usage of a Discord library.
//!
//! Here is a list of known libraries:
//!
//! - [`serenity-lavalink`], used with the [`serenity`] library.
//!
//! ### Installation
//!
//! If using `lavalink.rs` directly, add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! lavalink = { git = "https://github.com/serenity-rs/lavalink.rs" }
//! ```
//!
//! And the following to your project's `main.rs` or `lib.rs`:
//!
//! ```rust
//! extern crate lavalink;
//! ```
//!
//! ### License
//!
//! The library is licensed under the [ISC license][license].
//!
//! [`serenity`]: https://github.com/serenity-rs/serenity
//! [`serenity-lavalink`]: https://github.com/serenity-rs/serenity-lavalink
//! [ci]: https://travis-ci.org/serenity-rs/lavalink.rs
//! [ci-badge]: https://travis-ci.org/serenity-rs/lavalink.rs.svg?branch=master
//! [docs]: https://docs.rs/lavalink
//! [docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg
//! [lavalink]: https://github.com/Frederikam/Lavalink
//! [license]: https://github.com/serenity-rs/lavalink.rs/blob/master/LICENSE.md

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

extern crate parking_lot;
extern crate percent_encoding;
extern crate serde;
extern crate serde_json;
extern crate base64;
extern crate byteorder;

#[cfg(feature = "futures")]
extern crate futures;
#[cfg(feature = "http")]
extern crate http;
#[cfg(feature = "hyper")]
extern crate hyper;
#[cfg(feature = "reqwest")]
extern crate reqwest;

pub mod model;
pub mod opcodes;
pub mod rest;
pub mod stats;
pub mod decoder;

mod error;
mod prelude;

pub use error::{Error, Result};
