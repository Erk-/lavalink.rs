use serde_json::Error as JsonError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::result::Result as StdResult;
use std::sync::mpsc::SendError;
use websocket::client::ParseError;
use websocket::WebSocketError;

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
    /// An error occurred while parsing a URI.
    UriParse(ParseError),
    /// An error from the `websocket` crate.
    WebSocket(WebSocketError),
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
            Error::UriParse(ref inner) => inner.description(),
            Error::WebSocket(ref inner) => inner.description(),
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

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::UriParse(err)
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

impl From<WebSocketError> for Error {
    fn from(err: WebSocketError) -> Self {
        Error::WebSocket(err)
    }
}
