//! A collection of opcodes for use between the LavaLink client and server.

use std::str::FromStr;
use std::string::ToString;

/// An opcode used to indicate the type of message received.
///
/// Note that although serde's `Deserialize` and `Serialize` are derived on this
/// type, it may be preferable to use the `FromStr` and `ToString`
/// implementations for performance in some cases.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Opcode {
    /// Makes the server queue a voice connection.
    ///
    /// This is sent by the client to the server.
    Connect,
    /// Makes the server close a voice connection.
    ///
    /// This is sent by the client to the server.
    Disconnect,
    /// Indicates that the server emitted an event.
    ///
    /// This is sent by the server to the client.
    Event,
    /// Request used to check if a shard is connected.
    ///
    /// This is sent by the server to the client.
    IsConnectedReq,
    /// Sent in response to a [`IsConnectedReq`].
    ///
    /// This is sent by the client to the server.
    ///
    /// [`IsConnectedReq`]: #variant.IsConnectedReq
    IsConnectedRes,
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
    /// A WebSocket message payload to forward to the shard's WebSocket for
    /// sending.
    ///
    /// This is sent by the server to the client.
    SendWS,
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
    /// Request to check if the voice channel and/or guild exists and that the
    /// client has access to the voice channel.
    ///
    /// This is sent by the server to the client.
    ValidationReq,
    /// A response to a [`ValidationReq`] containing the received `guild_id`
    /// and potentially `channel_id`, containing a `valid` with a boolean
    /// indicating whether the combination is valid.
    ///
    /// This is sent by the client to the server.
    ValidationRes,
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
            Connect => "connect",
            Disconnect => "disconnect",
            Event => "event",
            IsConnectedReq => "isConnectedReq",
            IsConnectedRes => "isConnectedRes",
            Pause => "pause",
            Play => "play",
            PlayerUpdate => "playerUpdate",
            Seek => "seek",
            SendWS => "sendWS",
            Stats => "stats",
            Stop => "stop",
            Unknown => "unknown",
            ValidationReq => "validationReq",
            ValidationRes => "validationRes",
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
            "connect" => Connect,
            "voiceUpdate" => VoiceUpdate,
            "disconnect" => Disconnect,
            "validationReq" => ValidationReq,
            "validationRes" => ValidationRes,
            "isConnectedReq" => IsConnectedReq,
            "isConnectedRes" => IsConnectedRes,
            "play" => Play,
            "stop" => Stop,
            "pause" => Pause,
            "seek" => Seek,
            "volume" => Volume,
            "sendWS" => SendWS,
            "playerUpdate" => PlayerUpdate,
            "stats" => Stats,
            "event" => Event,
            _ => return Err(Unknown),
        })
    }
}
