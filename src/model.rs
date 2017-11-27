//! A collection of messages to send to and receive from the LavaLink node.

use serde_json;
use super::opcodes::Opcode;
use websocket::OwnedMessage;
use ::prelude::*;

/// Message used to connect a new player.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Connect {
    /// The ID of the channel being connected to.
    pub channel_id: String,
    /// The ID of the guild being connected to.
    pub guild_id: String,
    op: Opcode,
}

impl Connect {
    /// Creates a new `Connect` message.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lavalink::model::Connect;
    ///
    /// let _msg = Connect::new("381880193700069380", "381880193251409931");
    /// ```
    pub fn new<S: Into<String>>(channel_id: S, guild_id: S) -> Self {
        Self {
            channel_id: channel_id.into(),
            guild_id: guild_id.into(),
            op: Opcode::Connect,
        }
    }
}

/// Message used to disconnect a player.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Disconnect {
    /// The ID of the guild being disconnected from.
    pub guild_id: String,
    op: Opcode,
}

impl Disconnect {
    /// Creates a new `Disconnect` message.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lavalink::model::Disconnect;
    ///
    /// let _msg = Disconnect::new("381880193251409931");
    /// ```
    pub fn new<S: Into<String>>(guild_id: S) -> Self {
        Self {
            guild_id: guild_id.into(),
            op: Opcode::Disconnect,
        }
    }
}

/// A response from the client to the node about whether a shard is connected.
///
/// This should be sent to the node in reply to a [`IsConnectedRequest`].
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IsConnectedResponse {
    /// Whether the shard is connected.
    pub connected: bool,
    op: Opcode,
    /// The ID of the shard whose connection status is being confirmed.
    pub shard_id: u64,
}

impl IsConnectedResponse {
    /// Creates a new `IsConnectedResponse` message.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lavalink::model::IsConnectedResponse;
    ///
    /// let _msg = IsConnectedResponse::new(5, true);
    /// ```
    pub fn new(shard_id: u64, connected: bool) -> Self {
        Self {
            op: Opcode::IsConnectedRes,
            connected,
            shard_id,
        }
    }
}

/// A message sent to a node to modify the pause state a guild's player.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pause {
    /// The ID of the guild whose player's pause state is being modified.
    pub guild_id: String,
    op: Opcode,
    /// Whether to pause the player or not.
    pub pause: bool,
}

impl Pause {
    /// Creates a new `Pause` message.
    ///
    /// # Examples
    ///
    /// Pause a guild's player:
    ///
    /// ```rust,no_run
    /// use lavalink::model::Pause;
    ///
    /// let _msg = Pause::new("381880193251409931", true);
    /// ```
    pub fn new<S: Into<String>>(guild_id: S, pause: bool) -> Self {
        Self {
            guild_id: guild_id.into(),
            op: Opcode::Pause,
            pause,
        }
    }
}

/// A message sent to a node to play a new audio stream via a guild's player.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Play {
    /// The time at which to end the stream.
    ///
    /// If set to `None`, this will play until the stream ends.
    pub end_time: Option<u64>,
    /// The ID of the guild whose player is having a stream added.
    pub guild_id: String,
    op: Opcode,
    /// The time at which to start the stream.
    ///
    /// If set to `None`, this will play starting at the start of a stream.
    pub start_time: Option<u64>,
    /// The base64 encoded track information.
    pub track: String,
}

impl Play {
    /// Creates a new `Play` message.
    ///
    /// Note that the `track` information is not a URL or name of a song; it is
    /// a base64 encoded string containing track information.
    ///
    /// # Examples
    ///
    /// Play a song on a guild's audio player, with no custom start or end time:
    ///
    /// ```rust,no_run
    /// use lavalink::model::Play;
    ///
    /// let _msg = Play::new("381880193251409931", "info here", None, None);
    /// ```
    pub fn new<S: Into<String>>(
        guild_id: S,
        track: S,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Self {
        Self {
            guild_id: guild_id.into(),
            op: Opcode::Play,
            track: track.into(),
            end_time,
            start_time,
        }
    }
}

/// A message sent to a node to seek a guild's audio player to a specific time.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Seek {
    /// The ID of the guild whose player's position is being seeked.
    pub guild_id: String,
    op: Opcode,
    /// The time position to seek to.
    pub position: i64,
}

impl Seek {
    /// Creates a new `Seek` message.
    ///
    /// # Examples
    ///
    /// Create a new message to seek a guild's position to 30000:
    ///
    /// ```rust,no_run
    /// use lavalink::model::Seek;
    ///
    /// let _msg = Seek::new("381880193251409931", 30_000);
    /// ```
    pub fn new<S: Into<String>>(guild_id: S, position: i64) -> Self {
        Self {
            guild_id: guild_id.into(),
            op: Opcode::Seek,
            position,
        }
    }
}

/// A message sent to a node to stop a guild's audio player.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    /// The ID of the guild whose audio player is to stop.
    pub guild_id: String,
    op: Opcode,
}

impl Stop {
    /// Creates a new `Stop` message.
    ///
    /// # Examples
    ///
    /// Create a new message to stop a guild's audio player:
    ///
    /// ```rust,no_run
    /// use lavalink::model::Stop;
    ///
    /// let _msg = Stop::new("381880193251409931");
    /// ```
    pub fn new<S: Into<String>>(guild_id: S) -> Self {
        Self {
            guild_id: guild_id.into(),
            op: Opcode::Stop,
        }
    }
}

/// A message sent to a node in response to a received [`ValidationRequest`]
/// message.
///
/// **Note**: If the received [`ValidationRequest::channel_id`] is `None`, then
/// the response's [`channel_id`] must be `None` as well. Otherwise, the same
/// ID received must be sent in reply.
///
/// # Examples
///
/// If a `ValidationRequest` with a `channel_id` like so is received (in JSON
/// form):
///
/// ```json
/// {
///   "channel_id": "381880193700069380",
///   "guild_id": "381880193251409931",
///   // ...
/// }
/// ```
///
/// Then the following message must be constructed, noting the `channel_id`
/// parameter:
///
/// ```rust,no_run
/// use lavalink::model::ValidationResponse;
///
/// let guild_id = "381880193251409931";
/// let channel_id = Some("381880193700069380");
///
/// // Note that `valid`'s boolean value is variant on your program's state.
/// let _msg = ValidationResponse::new(guild_id, channel_id, true);
/// ```
///
/// [`ValidationRequest`]: struct.ValidationRequest.html
/// [`ValidationRequest::channel_id`]: struct.ValidationRequest.html#structfield.channel_id
/// [`channel_id`]: #structfield.channel_id
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResponse {
    /// The ID of the channel being validated.
    pub channel_id: Option<String>,
    /// The ID of the guild being validated.
    pub guild_id: String,
    op: Opcode,
    /// Whether the guild ID, and potentially channel ID, combination is valid.
    pub valid: bool,
}

impl ValidationResponse {
    /// Creates a new `ValidationResponse` message.
    ///
    /// # Examples
    ///
    /// Create a new message with no channel ID present, describing the guild as
    /// being invalid:
    ///
    /// ```rust,no_run
    /// use lavalink::model::ValidationResponse;
    ///
    /// let _msg = ValidationResponse::new("381880193251409931", None, false);
    /// ```
    pub fn new<S>(guild_id: S, channel_id: Option<S>, valid: bool) -> Self
        where S: Into<String> {
        Self {
            channel_id: channel_id.map(Into::into),
            guild_id: guild_id.into(),
            op: Opcode::ValidationRes,
            valid,
        }
    }
}

/// A message sent to a node, relaying a voice state update received from
/// Discord.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceUpdate {
    /// The endpoint of the voice state.
    pub endpoint: String,
    /// The event data for the voice update.
    pub event: VoiceUpdateEvent,
    /// The guild which an update was dispatched for.
    pub guild_id: String,
    op: Opcode,
    /// The session ID for the voice session.
    pub session_id: String,
    /// The token.
    pub token: String,
}

impl VoiceUpdate {
    /// Creates a new voice update message.
    pub fn new<S>(session_id: S, guild_id: S, token: S, endpoint: S) -> Self
        where S: Into<String> {
        let endpoint = endpoint.into();
        let guild_id = guild_id.into();
        let token = token.into();

        Self {
            endpoint: endpoint.clone(),
            event: VoiceUpdateEvent {
                endpoint: endpoint,
                guild_id: guild_id.clone(),
                token: token.clone(),
            },
            op: Opcode::VoiceUpdate,
            session_id: session_id.into(),
            guild_id,
            token,
        }
    }
}

/// Additional event data for a [`VoiceUpdate`].
///
/// [`VoiceUpdate`]: struct.VoiceUpdate.html
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceUpdateEvent {
    /// The endpoint of the voice state.
    pub endpoint: String,
    /// The guild which an update was dispatched for.
    pub guild_id: String,
    /// The token.
    pub token: String,
}

/// A message sent to an audio node to update the volume of a player.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    /// The ID of the guild whose player is having their volume modified.
    pub guild_id: String,
    op: Opcode,
    /// The new volume setting to use.
    pub volume: i32,
}

impl Volume {
    /// Creates a new message to modify a guild's volume setting.
    ///
    /// The given volume does not increment or decrement the existing volume
    /// setting, but instead sets it in-place.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lavalink::model::Volume;
    ///
    /// let _msg = Volume::new("381880193251409931", 110);
    /// ```
    pub fn new<S: Into<String>>(guild_id: S, volume: i32) -> Self {
        Self {
            guild_id: guild_id.into(),
            op: Opcode::Volume,
            volume,
        }
    }
}

/// Used to convert a message into a JSON serialized WebSocket message.
pub trait IntoWebSocketMessage {
    /// Converted a message into a JSON serialized WebSocket message.
    fn into_ws_message(self) -> Result<OwnedMessage>;
}

macro_rules! impl_stuff_for_model {
    ($($model: ident),*) => {
        $(
            /// Implementation for retrieving the opcode of a model.
            impl $model {
                /// Retrieves the opcode for the model.
                pub fn opcode(&self) -> Opcode {
                    self.op.clone()
                }
            }

            impl IntoWebSocketMessage for $model {
                /// Serializes the model into a JSON string, wrapping it in an
                /// owned WebSocket message.
                fn into_ws_message(self) -> Result<OwnedMessage> {
                    serde_json::to_string(&self)
                        .map(OwnedMessage::Text)
                        .map_err(From::from)
                }
            }
        )*
    };
}

impl_stuff_for_model! {
    Connect,
    Disconnect,
    IsConnectedResponse,
    Pause,
    Play,
    Seek,
    Stop,
    ValidationResponse,
    VoiceUpdate,
    Volume
}
