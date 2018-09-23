//! A collection of opcodes for use between the LavaLink client and server.

use std::str::FromStr;
use std::string::ToString;

/// An opcode used to indicate the type of message received.
///
/// Note that although serde's `Deserialize` and `Serialize` are derived on this
/// type, it may be preferable to use the `FromStr` and `ToString`
/// implementations for performance in some cases.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Opcode {
    /// Destroys a player for a guild.
    ///
    /// This is sent by the client to the server.
    Destroy,
    /// Indicates that the server emitted an event.
    ///
    /// This is sent by the server to the client.
    Event,
    /// Sets the pause state of a guild's player.
    ///
    /// This is sent by the client to the server.
    Pause,
    /// Causes the player to play a track.
    ///
    /// This is sent by the client to the server.
    Play,
    /// Includes information about the position of a player.
    ///
    /// This is sent by the server to the client.
    PlayerUpdate,
    /// Makes the player seek to a position of the track.
    ///
    /// This is sent by the client to the server.
    Seek,
    /// A set of statistics collected once a minute.
    ///
    /// This is sent by the server to the client.
    Stats,
    /// Causes a guild's player to stop.
    ///
    /// This is sent by the client to the server.
    Stop,
    /// An unknown opcode.
    Unknown,
    /// A voice state update received from Discord to be forwarded.
    ///
    /// This is sent by the client to the server.
    VoiceUpdate,
    /// Sets the player volume information, on a scale of 1 to 150. The default
    /// is 100.
    ///
    /// This is sent by the client to the server.
    Volume,
}

impl ToString for Opcode {
    fn to_string(&self) -> String {
        use self::Opcode::*;

        match *self {
            Destroy => "destroy",
            Event => "event",
            Pause => "pause",
            Play => "play",
            PlayerUpdate => "playerUpdate",
            Seek => "seek",
            Stats => "stats",
            Stop => "stop",
            Unknown => "unknown",
            VoiceUpdate => "voiceUpdate",
            Volume => "volume",
        }.to_owned()
    }
}

impl FromStr for Opcode {
    type Err = Opcode;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Opcode::*;

        Ok(match s {
            "destroy" => Destroy,
            "voiceUpdate" => VoiceUpdate,
            "play" => Play,
            "stop" => Stop,
            "pause" => Pause,
            "seek" => Seek,
            "volume" => Volume,
            "playerUpdate" => PlayerUpdate,
            "stats" => Stats,
            "event" => Event,
            _ => return Err(Unknown),
        })
    }
}
