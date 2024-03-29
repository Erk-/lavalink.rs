//! A trait implementation for Reqwest's Client and an owned client for working
//! with the Lavalink REST API.

use crate::Result;
use percent_encoding::{self, DEFAULT_ENCODE_SET};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use reqwest::{Body, Client as ReqwestClient, Method, Request, RequestBuilder};
use serde_json;
use std::io::Read;
use super::{Load, LoadedTrack};

/// An HTTP client used to communicate with a LavaLink node.
#[derive(Debug)]
pub struct RestClient {
    client: ReqwestClient,
    host: String,
    password: Vec<u8>,
}

impl RestClient {
    /// Creates a new reqwest Client wrapper used to communicate with a LavaLink
    /// node.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lavalink::rest::reqwest::RestClient;
    ///
    /// let client = RestClient::new("127.0.0.1:2333", "test_password");
    /// ```
    #[inline]
    pub fn new(host: impl Into<String>, password: impl Into<Vec<u8>>) -> Self {
        Self::_new(host.into(), password.into())
    }

    fn _new(host: String, password: Vec<u8>) -> Self {
        Self {
            client: ReqwestClient::new(),
            host,
            password,
        }
    }

    /// Loads tracks matching an identifier via a given node.
    #[inline]
    pub fn load_tracks(&self, identifier: impl AsRef<str>)
        -> Result<Load> {
        self._load_tracks(identifier.as_ref())
    }

    fn _load_tracks(&self, identifier: &str) -> Result<Load> {
        self.client.load_tracks(&self.host, &self.password, identifier)
    }

    /// Decodes a track via a given node.
    #[inline]
    pub fn decode_track(
        &self,
        track: impl Into<String>,
    ) -> Result<LoadedTrack> {
        self._decode_track(track.into())
    }

    fn _decode_track(&self, track: String) -> Result<LoadedTrack> {
        self.client.decode_track(&self.host, &self.password, track)
    }

    /// Decodes a vector of tracks via a given node.
    #[inline]
    pub fn decode_tracks<T, It>(
        &self,
        tracks: impl IntoIterator<Item = impl Into<Vec<u8>>>,
    ) -> Result<Vec<LoadedTrack>> {
        self._decode_tracks(tracks.into_iter().map(Into::into).collect())
    }

    fn _decode_tracks(&self, tracks: Vec<Vec<u8>>) -> Result<Vec<LoadedTrack>> {
        self.client.decode_tracks(&self.host, &self.password, tracks)
    }
}

/// Trait to implement for working with the Lavalink REST API over a Reqwest
/// client.
pub trait LavalinkRestRequester {
    /// Loads tracks matching an identifier via a given node.
    fn load_tracks(
        &self,
        host: impl AsRef<str>,
        password: impl AsRef<[u8]>,
        identifier: impl AsRef<str>,
    ) -> Result<Load>;

    /// Decodes a track via a given node.
    fn decode_track(
        &self,
        host: impl AsRef<str>,
        password: impl AsRef<[u8]>,
        track: impl Into<String>,
    ) -> Result<LoadedTrack>;

    /// Decodes a vector of tracks via a given node.
    fn decode_tracks(
        &self,
        host: impl AsRef<str>,
        password: impl AsRef<[u8]>,
        tracks: impl IntoIterator<Item = impl Into<Vec<u8>>>,
    ) -> Result<Vec<LoadedTrack>>;
}

impl LavalinkRestRequester for ReqwestClient {
    #[inline]
    fn load_tracks(
        &self,
        host: impl AsRef<str>,
        password: impl AsRef<[u8]>,
        identifier: impl AsRef<str>,
    ) -> Result<Load> {
        load_tracks(
            self,
            host.as_ref(),
            password.as_ref(),
            identifier.as_ref(),
        )
    }

    #[inline]
    fn decode_track(
        &self,
        host: impl AsRef<str>,
        password: impl AsRef<[u8]>,
        track: impl Into<String>,
    ) -> Result<LoadedTrack> {
        decode_track(
            self,
            host.as_ref(),
            password.as_ref(),
            track.into(),
        )
    }

    #[inline]
    fn decode_tracks(
        &self,
        host: impl AsRef<str>,
        password: impl AsRef<[u8]>,
        tracks: impl IntoIterator<Item = impl Into<Vec<u8>>>,
    ) -> Result<Vec<LoadedTrack>> {
        decode_tracks(
            self,
            host.as_ref(),
            password.as_ref(),
            &tracks.into_iter().map(Into::into).collect::<Vec<_>>(),
        )
    }
}

fn decode_track(
    client: &ReqwestClient,
    host: &str,
    password: &[u8],
    track: String,
) -> Result<LoadedTrack> {
    let uri = format!("/decodetrack?track={}", track);
    let request = create_request(
        client,
        Method::GET,
        uri.as_ref(),
        None,
        host,
        password,
    )?.build()?;

    let response = run_request(client, request)?;

    let info = serde_json::from_slice(&response)?;

    Ok(LoadedTrack {
        info,
        track,
    })
}

fn decode_tracks(
    client: &ReqwestClient,
    host: &str,
    password: &[u8],
    tracks: &[Vec<u8>],
) -> Result<Vec<LoadedTrack>> {
    let tracks = serde_json::to_vec(&tracks)?;

    let request = create_request(
        client,
        Method::POST,
        "/decodetracks",
        Some(tracks),
        host,
        password,
    )?.build()?;

    run_request(client, request)
        .and_then(|resp| serde_json::from_slice(&resp).map_err(From::from))
        .map_err(From::from)
}

fn load_tracks(
    client: &ReqwestClient,
    host: &str,
    password: &[u8],
    identifier: &str,
) -> Result<Load> {
    // url encoding the identifier
    let identifier = percent_encoding::utf8_percent_encode(
        identifier,
        DEFAULT_ENCODE_SET,
    );

    let uri = format!("/loadtracks?identifier={}", identifier);
    let request = create_request(
        client,
        Method::GET,
        uri.as_ref(),
        None,
        host,
        password,
    )?.build()?;

    run_request(client, request)
        .and_then(|body| serde_json::from_slice(&body).map_err(From::from))
        .map_err(From::from)
}

fn create_request<'a>(
    client: &'a ReqwestClient,
    method: Method,
    uri: &str,
    body: Option<Vec<u8>>,
    host: &str,
    password: &[u8],
) -> Result<RequestBuilder> {
    let mut builder = client.request(method, &format!("{}{}", host, uri));

    let mut headers = HeaderMap::new();

    // cant use hyper::header::Authorization because it requires prefix of Basic or Bearer
    headers.insert(AUTHORIZATION, HeaderValue::from_bytes(password)?);

    if let Some(body) = body {
        builder = builder.body(Body::from(body));
        let value = HeaderValue::from_static("application/json");

        headers.insert(CONTENT_TYPE, value);
    }

    builder = builder.headers(headers);

    Ok(builder)
}

fn run_request(client: &ReqwestClient, request: Request) -> Result<Vec<u8>> {
    match client.execute(request) {
        Ok(response) => {
            Ok(response.bytes().fold(Vec::new(), |mut v: Vec<u8>, chunk| {
                match chunk {
                    Ok(b) => v.push(b), // append the byte to the vec
                    Err(e) => {
                        error!("error parsing response body chunk {:?}", e);
                        return v;
                    },
                };

                v // return the vec as the final result
            }))
        },
        Err(e) => Err(From::from(e)),
    }
}
