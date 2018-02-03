use ::prelude::Result;
use std::io::*;
use byteorder::*;

const TRACK_INFO_VERSIONED: i32 = 1;

macro_rules! move_cursor {
    ($c:ident,$s:tt) => {
        let pos = $c.position();
        $c.set_position(pos + $s);
    };
}

fn read_string(cursor: &mut Cursor<Vec<u8>>) -> Result<String> {
    let size = cursor.read_u16::<BigEndian>()?;
    let mut buf = vec![0u8; size as usize];
    cursor.read_exact(&mut buf)?;
    let string = String::from_utf8(buf)?;
    Ok(string)
}

/// Holds decoded track information from a lavaplayer track blob
#[derive(Debug)]
pub struct DecodedTrack {
    pub version: u8,
    pub title: String,
    pub author: String,
    pub length: u64,
    pub identifier: String,
    pub stream: bool,
    pub url: Option<String>,
    pub source: String,
}

/// Decodes a binary lavaplayer track blob
pub fn decode_track(input: Vec<u8>) -> Result<DecodedTrack> {
    let mut cursor = Cursor::new(input);

    let value = cursor.read_u8()?;
    let flags = ((value as i64 & 0xC0000000) >> 30) as i32;

    // gets the message size (we dont care)
    // let size = value as i32 & 0x3FFFFFFF;

    move_cursor!(cursor, 2);

    let version = match flags & TRACK_INFO_VERSIONED {
        0 => 1,
        _ => cursor.read_u8()? & 0xFF
    } as u8;

    move_cursor!(cursor, 2); // dont care

    let title = read_string(&mut cursor)?;

    let author = read_string(&mut cursor)?;

    let length = cursor.read_u64::<BigEndian>()?;

    let identifier = read_string(&mut cursor)?;

    let stream = cursor.read_u8()? == 1;
    let has_url = cursor.read_u8()? == 1;

    let url = if has_url {
        Some(read_string(&mut cursor)?)
    } else {
        let size = cursor.read_u8()? as u64;
        move_cursor!(cursor, size);
        None
    };

    let source = read_string(&mut cursor)?;
    println!("source: {}", source);

    let mut buf = Vec::new();
    cursor.read_to_end(&mut buf)?;
    println!("{:?}", buf);

    Ok(DecodedTrack {
        version, title, author, length, identifier, stream, url, source
    })
}

/// Decodes a base64 string lavaplayer track blob
pub fn decode_track_base64(input: &str) -> Result<DecodedTrack> {
    decode_track(::base64::decode(input)?)
}
