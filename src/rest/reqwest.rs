use percent_encoding::{self, DEFAULT_ENCODE_SET};
use reqwest::header::{ContentType, Headers};
use reqwest::{Body, Client as ReqwestClient, Method, Request, RequestBuilder};
use serde_json;
use std::io::Read;
use super::LoadedTrack;
use ::Result;

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
    pub fn new<S, V>(host: S, password: V) -> Self
        where S: Into<String>, V: Into<Vec<u8>> {
        Self {
            client: ReqwestClient::new(),
            host: host.into(),
            password: password.into(),
        }
    }

    pub fn load_tracks<T: AsRef<str>>(&self, identifier: T)
        -> Result<Vec<LoadedTrack>> {
        self.client.load_tracks(&self.host, &self.password, identifier)
    }

    pub fn decode_track<T: Into<String>>(&self, track: T)
        -> Result<LoadedTrack> {
        self.client.decode_track(&self.host, &self.password, track)
    }

    pub fn decode_tracks<T, It>(&self, tracks: It) -> Result<Vec<LoadedTrack>>
        where T: Into<Vec<u8>>, It: IntoIterator<Item = T> {
        self.client.decode_tracks(&self.host, &self.password, tracks)
    }
}

pub trait LavalinkRestRequester {
    fn load_tracks<S, T, U>(&self, host: S, password: T, identifier: U)
        -> Result<Vec<LoadedTrack>>
        where S: AsRef<str>, T: AsRef<[u8]>, U: AsRef<str>;

    fn decode_track<S, T, U>(&self, host: S, password: T, track: U)
        -> Result<LoadedTrack>
        where S: AsRef<str>, T: AsRef<[u8]>, U: Into<String>;

    fn decode_tracks<S, T, U, It>(&self, host: S, password: T, tracks: It)
        -> Result<Vec<LoadedTrack>>
        where S: AsRef<str>,
              T: AsRef<[u8]>,
              U: Into<Vec<u8>>,
              It: IntoIterator<Item = U>;
}

impl LavalinkRestRequester for ReqwestClient {
    fn load_tracks<S, T, U>(&self, host: S, password: T, identifier: U)
        -> Result<Vec<LoadedTrack>>
        where S: AsRef<str>, T: AsRef<[u8]>, U: AsRef<str> {
        let identifier = identifier.as_ref();

        // url encoding the identifier
        let identifier = percent_encoding::utf8_percent_encode(
            identifier,
            DEFAULT_ENCODE_SET,
        );

        let uri = format!("/loadtracks?identifier={}", identifier);
        let request = create_request(
            &self,
            Method::Get,
            uri.as_ref(),
            None,
            host.as_ref(),
            password.as_ref(),
        ).build()?;

        run_request(&self, request)
            .and_then(|body| serde_json::from_slice(&body).map_err(From::from))
            .map_err(From::from)
    }

    fn decode_track<S, T, U>(&self, host: S, password: T, track: U)
        -> Result<LoadedTrack>
        where S: AsRef<str>, T: AsRef<[u8]>, U: Into<String> {
        let track = track.into();
        let uri = format!("/decodetrack?track={}", track);
        let request = create_request(
            &self,
            Method::Get,
            uri.as_ref(),
            None,
            host.as_ref(),
            password.as_ref(),
        ).build()?;

        let response = run_request(&self, request)?;

        let info = serde_json::from_slice(&response)?;

        Ok(LoadedTrack {
            track: track,
            info,
        })
    }

    fn decode_tracks<S, T, U, It>(&self, host: S, password: T, tracks: It)
        -> Result<Vec<LoadedTrack>>
        where S: AsRef<str>,
              T: AsRef<[u8]>,
              U: Into<Vec<u8>>,
              It: IntoIterator<Item = U> {
        let tracks = tracks.into_iter().map(|x| x.into()).collect::<Vec<_>>();
        let tracks = serde_json::to_vec(&tracks)?;
        let body = (tracks, ContentType::json());

        let request = create_request(
            &self,
            Method::Post,
            "/decodetracks",
            Some(body),
            host.as_ref(),
            password.as_ref(),
        ).build()?;

        run_request(&self, request)
            .and_then(|resp| serde_json::from_slice(&resp).map_err(From::from))
            .map_err(From::from)
    }
}


fn create_request<'a>(
    client: &'a ReqwestClient,
    method: Method,
    uri: &str,
    body: Option<(Vec<u8>, ContentType)>,
    host: &str,
    password: &[u8],
) -> RequestBuilder {
    let mut builder = client.request(method, &format!("{}{}", host, uri));

    let mut headers = Headers::new();

    // cant use hyper::header::Authorization because it requires prefix of Basic or Bearer
    headers.set_raw("Authorization", vec![password.to_owned()]);

    if let Some((body, content_type)) = body {
        builder.body(Body::from(body));
        headers.set(content_type);
    }

    builder.headers(headers);

    builder
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
