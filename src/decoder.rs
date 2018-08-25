//! Functions for decoding a track.

use byteorder::{BE, ReadBytesExt};
use std::io::{Cursor, Read};
use Result;

const TRACK_INFO_VERSIONED: i32 = 1;

macro_rules! move_cursor {
    ($c:ident,$s:tt) => {
        let pos = $c.position();
        $c.set_position(pos + $s);
    };
}

fn read_string(cursor: &mut Cursor<Vec<u8>>) -> Result<String> {
    let size = cursor.read_u16::<BE>()?;
    let mut buf = vec![0u8; size as usize];
    cursor.read_exact(&mut buf)?;
    let string = String::from_utf8(buf)?;

    Ok(string)
}

/// Holds decoded track information from a lavaplayer track blob
#[derive(Debug)]
pub struct DecodedTrack {
    /// The version of the track.
    pub version: u8,
    /// The title of the track.
    pub title: String,
    /// The author of the track.
    pub author: String,
    /// The length of the track.
    pub length: u64,
    /// The unique identifier for the track.
    pub identifier: String,
    /// Whether the track is a stream.
    pub stream: bool,
    /// The URL of the track.
    pub url: Option<String>,
    /// The source of the track.
    pub source: String,
}

/// Decodes a binary lavaplayer track blob
#[inline]
pub fn decode_track(input: impl Into<Vec<u8>>) -> Result<DecodedTrack> {
    _decode_track(input.into())
}

fn _decode_track(input: Vec<u8>) -> Result<DecodedTrack> {
    let mut cursor = Cursor::new(input);

    let value = cursor.read_u8()?;
    let flags = ((i64::from(value) & 0xC000_0000) >> 30) as i32;

    // gets the message size (we dont care)
    // let size = value as i32 & 0x3FFFFFFF;

    move_cursor!(cursor, 2);

    let version = match flags & TRACK_INFO_VERSIONED {
        0 => 1,
        _ => cursor.read_u8()?,
    };

    move_cursor!(cursor, 2); // dont care

    let title = read_string(&mut cursor)?;
    let author = read_string(&mut cursor)?;
    let length = cursor.read_u64::<BE>()?;
    let identifier = read_string(&mut cursor)?;

    let stream = cursor.read_u8()? == 1;
    let has_url = cursor.read_u8()? == 1;

    let url = if has_url {
        Some(read_string(&mut cursor)?)
    } else {
        let size = u64::from(cursor.read_u8()?);
        move_cursor!(cursor, size);
        None
    };

    let source = read_string(&mut cursor)?;

    Ok(DecodedTrack {
        author,
        identifier,
        length,
        source,
        stream,
        title,
        url,
        version,
    })
}

/// Decodes a base64 string lavaplayer track blob.
#[inline]
pub fn decode_track_base64(input: impl AsRef<str>) -> Result<DecodedTrack> {
    _decode_track_base64(input.as_ref())
}

#[inline]
fn _decode_track_base64(input: &str) -> Result<DecodedTrack> {
    decode_track(::base64::decode(input)?)
}
