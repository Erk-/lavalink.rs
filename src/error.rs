use serde_json::Error as JsonError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::result::Result as StdResult;
use std::sync::mpsc::SendError;
use std::string::FromUtf8Error;
use base64::DecodeError;

#[cfg(feature = "hyper")]
use hyper::error::{Error as HyperError, UriError};
#[cfg(feature = "reqwest")]
use reqwest::Error as ReqwestError;

/// Common result type returned by library functions.
///
/// The Err type is always [`Error`].
///
/// [`Error`]: enum.Error.html
pub type Result<T> = StdResult<T, Error>;

/// Common error type used throughout the library's return types.
#[derive(Debug)]
pub enum Error {
    /// An error from the `hyper` crate.
    #[cfg(feature = "hyper")]
    Hyper(HyperError),
    /// An error from the `std::io` module.
    Io(IoError),
    /// An error from the `serde_json` crate.
    Json(JsonError),
    /// A player already exists for the guild.
    PlayerAlreadyExists,
    /// An error from the `reqwest` crate.
    #[cfg(feature = "reqwest")]
    Reqwest(ReqwestError),
    /// An error occurred sending a WebSocket message to an mpsc Receiver.
    ///
    /// This is the `Display` implementation of the error.
    Send(String),
    /// An error from the `hyper` crate while parsing a URI.
    #[cfg(feature = "hyper")]
    Uri(UriError),
    /// An error parsing a UTF-8 String with `String::from_utf8`.
    ParseUtf8(FromUtf8Error),
    /// An error from the `base64` crate while decoding.
    Base64Error(DecodeError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            #[cfg(feature = "hyper")]
            Error::Hyper(ref inner) => inner.description(),
            Error::Io(ref inner) => inner.description(),
            Error::Json(ref inner) => inner.description(),
            Error::PlayerAlreadyExists => "Player already exists for the guild",
            #[cfg(feature = "reqwest")]
            Error::Reqwest(ref inner) => inner.description(),
            Error::Send(ref inner) => inner,
            #[cfg(feature = "hyper")]
            Error::Uri(ref inner) => inner.description(),
            Error::ParseUtf8(ref inner) => inner.description(),
            Error::Base64Error(ref inner) => inner.description(),
        }
    }
}

#[cfg(feature = "hyper")]
impl From<HyperError> for Error {
    fn from(err: HyperError) -> Self {
        Error::Hyper(err)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Io(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::Json(err)
    }
}

#[cfg(feature = "reqwest")]
impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Self {
        Error::Reqwest(err)
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(err: SendError<T>) -> Self {
        Error::Send(format!("{}", err))
    }
}

#[cfg(feature = "hyper")]
impl From<UriError> for Error {
    fn from(err: UriError) -> Self {
        Error::Uri(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Error::ParseUtf8(error)
    }
}

impl From<DecodeError> for Error {
    fn from(error: DecodeError) -> Self {
        Error::Base64Error(error)
    }
}
