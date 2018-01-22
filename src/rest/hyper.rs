use futures::{Future, Stream, future};
use hyper::client::{Client, Connect, Request};
use hyper::header::ContentType;
use hyper::{Body, Method, Uri};
use percent_encoding::{self, DEFAULT_ENCODE_SET};
use serde::de::DeserializeOwned;
use serde_json;
use std::str::FromStr;
use super::LoadedTrack;
use ::{Error, Result};

pub trait LavalinkRestRequester {
    fn load_tracks<S, T, U>(&self, host: S, password: T, identifier: U)
        -> Box<Future<Item = Vec<LoadedTrack>, Error = Error>>
        where S: AsRef<str>, T: AsRef<[u8]>, U: AsRef<str>;

    fn decode_track<S, T, U>(&self, host: S, password: T, track: U)
        -> Box<Future<Item = LoadedTrack, Error = Error>>
        where S: AsRef<str>, T: AsRef<[u8]>, U: Into<String>;

    fn decode_tracks<S, T, U, It>(&self, host: S, password: T, tracks: It)
        -> Box<Future<Item = Vec<LoadedTrack>, Error = Error>>
        where S: AsRef<str>,
              T: AsRef<[u8]>,
              U: Into<Vec<u8>>,
              It: IntoIterator<Item = U>;
}

impl<C: Connect> LavalinkRestRequester for Client<C, Body> {
    fn load_tracks<S, T, U>(&self, host: S, password: T, identifier: U)
        -> Box<Future<Item = Vec<LoadedTrack>, Error = Error>>
        where S: AsRef<str>, T: AsRef<[u8]>, U: AsRef<str> {
        let identifier = identifier.as_ref();

        // url encoding the identifier
        let identifier = percent_encoding::utf8_percent_encode(
            identifier,
            DEFAULT_ENCODE_SET,
        );

        let uri = format!("/loadtracks?identifier={}", identifier);
        let request = create_request(
            Method::Get,
            uri.as_ref(),
            None,
            host.as_ref(),
            password.as_ref(),
        );
        let request = match request {
            Ok(v) => v,
            Err(why) => return Box::new(future::err(why)),
        };

        run_request(&self, request)
    }

    fn decode_track<S, T, U>(&self, host: S, password: T, track: U)
        -> Box<Future<Item = LoadedTrack, Error = Error>>
        where S: AsRef<str>, T: AsRef<[u8]>, U: Into<String> {
        let track = track.into();
        let uri = format!("/decodetrack?track={}", track);
        let request = create_request(
            Method::Get,
            uri.as_ref(),
            None,
            host.as_ref(),
            password.as_ref(),
        );
        let request = match request {
            Ok(v) => v,
            Err(why) => return Box::new(future::err(why)),
        };

        Box::new(run_request(self, request)
            .map(|info| {
                LoadedTrack {
                    info,
                    track,
                }
            })
            .from_err())
    }

    fn decode_tracks<S, T, U, It>(&self, host: S, password: T, tracks: It)
        -> Box<Future<Item = Vec<LoadedTrack>, Error = Error>>
        where S: AsRef<str>,
              T: AsRef<[u8]>,
              U: Into<Vec<u8>>,
              It: IntoIterator<Item = U> {
        let tracks = tracks.into_iter().map(|x| x.into()).collect::<Vec<_>>();
        let tracks = match serde_json::to_vec(&tracks) {
            Ok(tracks) => tracks,
            Err(why) => return Box::new(future::err(Error::Json(why))),
        };
        let body = (tracks, ContentType::json());

        let request = create_request(
            Method::Post,
            "/decodetracks",
            Some(body),
            host.as_ref(),
            password.as_ref(),
        );
        let request = match request {
            Ok(v) => v,
            Err(why) => return Box::new(future::err(why)),
        };

        run_request(&self, request)
    }
}

fn create_request(
    method: Method,
    uri: &str,
    body: Option<(Vec<u8>, ContentType)>,
    host: &str,
    password: &[u8],
) -> Result<Request> {
    let uri = Uri::from_str(&format!("{}{}", host, uri))?;
    let mut req = Request::new(method, uri);

    {
        // cant use hyper::header::Authorization because it requires prefix of
        // Basic or Bearer
        req.headers_mut().set_raw("Authorization", vec![password.to_owned()]);
    }

    if let Some((body, content_type)) = body {
        req.set_body(body);
        req.headers_mut().set(content_type);
    }

    Ok(req)
}

fn run_request<C, T>(client: &Client<C, Body>, request: Request)
    -> Box<Future<Item = T, Error = Error>>
    where C: Connect,
          T: DeserializeOwned + Sized + 'static {
    Box::new(client.request(request)
        .and_then(|res| res.body().concat2())
        .from_err::<Error>()
        .and_then(|body| serde_json::from_slice::<T>(&body).map_err(From::from))
        .from_err())
}