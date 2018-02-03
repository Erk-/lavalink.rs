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
    let size = cursor.read_u8()?;
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
}

/// Decodes a binary lavaplayer track blob
pub fn decode_track(input: Vec<u8>) -> Result<DecodedTrack> {
    let mut cursor = Cursor::new(input);

    let value = cursor.read_u8()?;
    let flags = ((value as i64 & 0xC0000000) >> 30) as i32;

    // gets the message size (we dont care)
    // let size = value as i32 & 0x3FFFFFFF;

    move_cursor!(cursor, 2); // padding

    let version = match flags & TRACK_INFO_VERSIONED {
        0 => 1,
        _ => cursor.read_u8()? & 0xFF
    } as u8;

    move_cursor!(cursor, 3); // dont care

    let title = read_string(&mut cursor)?;

    move_cursor!(cursor, 1); // padding

    let author = read_string(&mut cursor)?;

    let length = cursor.read_u64::<BigEndian>()?;

    move_cursor!(cursor, 1); // padding

    let identifier = read_string(&mut cursor)?;

    Ok(DecodedTrack {
        version, title, author, length, identifier

    })
}

/// Decodes a base64 string lavaplayer track blob
pub fn decode_track_base64(input: &str) -> Result<DecodedTrack> {
    decode_track(::base64::decode(input)?)
}
