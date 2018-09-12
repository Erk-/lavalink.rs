//! A trait implementation for Hyper's Client for working with the Lavalink
//! REST API.

use futures::{Future, Stream, future};
use hyper::client::connect::Connect;
use hyper::header::{CONTENT_TYPE, HeaderValue};
use hyper::{Body, Client, Method, Request, Uri};
use percent_encoding::{self, DEFAULT_ENCODE_SET};
use serde::de::DeserializeOwned;
use serde_json;
use std::str::FromStr;
use super::{Load, LoadedTrack};
use ::{Error, Result};

/// Trait to implement for working with the Lavalink REST API over a Hyper
/// client.
pub trait LavalinkRestRequester {
    /// Loads tracks matching an identifier via a given node.
    fn load_tracks(
        &self,
        host: impl AsRef<str>,
        password: impl AsRef<[u8]>,
        identifier: impl AsRef<str>,
    ) -> Box<Future<Item = Load, Error = Error> + Send>;

    /// Decodes a track via a given node.
    fn decode_track(
        &self,
        host: impl AsRef<str>,
        password: impl AsRef<[u8]>,
        track: impl Into<String>,
    ) -> Box<Future<Item = LoadedTrack, Error = Error> + Send>;

    /// Decodes a vector of tracks via a given node.
    fn decode_tracks(
        &self,
        host: impl AsRef<str>,
        password: impl AsRef<[u8]>,
        tracks: impl IntoIterator<Item = impl Into<Vec<u8>>>,
    ) -> Box<Future<Item = Vec<LoadedTrack>, Error = Error> + Send>;
}

impl<C: Connect + 'static> LavalinkRestRequester for Client<C, Body> {
    fn load_tracks(
        &self,
        host: impl AsRef<str>,
        password: impl AsRef<[u8]>,
        identifier: impl AsRef<str>,
    ) -> Box<Future<Item = Load, Error = Error> + Send> {
        load_tracks(
            &self,
            host.as_ref(),
            password.as_ref(),
            identifier.as_ref(),
        )
    }

    fn decode_track(
        &self,
        host: impl AsRef<str>,
        password: impl AsRef<[u8]>,
        track: impl Into<String>,
    ) -> Box<Future<Item = LoadedTrack, Error = Error> + Send> {
        decode_track(
            &self,
            host.as_ref(),
            password.as_ref(),
            track.into(),
        )
    }

    fn decode_tracks(
        &self,
        host: impl AsRef<str>,
        password: impl AsRef<[u8]>,
        tracks: impl IntoIterator<Item = impl Into<Vec<u8>>>,
    ) -> Box<Future<Item = Vec<LoadedTrack>, Error = Error> + Send> {
        decode_tracks(
            self,
            host.as_ref(),
            password.as_ref(),
            &tracks.into_iter().map(Into::into).collect::<Vec<_>>(),
        )
    }
}

fn decode_track<C: Connect + 'static>(
    client: &Client<C, Body>,
    host: &str,
    password: &[u8],
    track: String,
) -> Box<Future<Item = LoadedTrack, Error = Error> + Send> {
    let uri = format!("/decodetrack?track={}", track);
    let request = create_request(
        Method::GET,
        uri.as_ref(),
        None,
        host,
        password,
    );
    let request = match request {
        Ok(v) => v,
        Err(why) => return Box::new(future::err(why)),
    };

    Box::new(run_request(client, request)
        .map(|info| {
            LoadedTrack {
                info,
                track,
            }
        })
        .from_err())
}

fn decode_tracks<C: Connect + 'static>(
    client: &Client<C, Body>,
    host: &str,
    password: &[u8],
    tracks: &[Vec<u8>],
) -> Box<Future<Item = Vec<LoadedTrack>, Error = Error> + Send> {
    let tracks = match serde_json::to_vec(&tracks) {
        Ok(tracks) => tracks,
        Err(why) => return Box::new(future::err(Error::Json(why))),
    };
    let body = (tracks, HeaderValue::from_static("Application/json"));

    let request = create_request(
        Method::POST,
        "/decodetracks",
        Some(body),
        host,
        password,
    );
    let request = match request {
        Ok(v) => v,
        Err(why) => return Box::new(future::err(why)),
    };

    run_request(client, request)
}

fn load_tracks<C: Connect + 'static>(
    client: &Client<C, Body>,
    host: &str,
    password: &[u8],
    identifier: &str,
) -> Box<Future<Item = Load, Error = Error> + Send> {
    // url encoding the identifier
    let identifier = percent_encoding::utf8_percent_encode(
        identifier,
        DEFAULT_ENCODE_SET,
    );

    let uri = format!("/loadtracks?identifier={}", identifier);
    let request = create_request(
        Method::GET,
        uri.as_ref(),
        None,
        host,
        password,
    );
    let request = match request {
        Ok(v) => v,
        Err(why) => return Box::new(future::err(why)),
    };

    run_request(client, request)
}

fn create_request(
    method: Method,
    uri: &str,
    body: Option<(Vec<u8>, HeaderValue)>,
    host: &str,
    password: &[u8],
) -> Result<Request<Body>> {
    let uri = Uri::from_str(&format!("{}{}", host, uri))?;
    debug!("uri: {:?}", uri);
    let mut req = Request::builder();
    req.method(method);
    req.uri(uri);

    req.header("Authorization", password);

    let req = if let Some((body, content_type)) = body {
        req.header(CONTENT_TYPE, content_type);
        req.body(Body::from(body))?
    } else {
        req.body(Body::empty())?
    };

    Ok(req)
}

fn run_request<C, T>(client: &Client<C, Body>, request: Request<Body>)
    -> Box<Future<Item = T, Error = Error> + Send>
    where C: Connect + 'static,
          T: DeserializeOwned + Send + Sized + 'static {
    Box::new(client.request(request)
        .and_then(|res| res.into_body().concat2())
        .from_err::<Error>()
        .map(|body| {
            debug!("Body: {}", String::from_utf8_lossy(&body));

            body
        })
        .and_then(|body| serde_json::from_slice::<T>(&body).map_err(From::from))
        .from_err())
}
