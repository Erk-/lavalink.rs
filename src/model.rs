//! A collection of messages to send to and receive from the LavaLink node.

use serde::Serializer;
use super::opcodes::Opcode;
use std::result::Result as StdResult;

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
    #[serde(serialize_with = "serialize_option_u64")]
    pub end_time: Option<u64>,
    /// The ID of the guild whose player is having a stream added.
    pub guild_id: String,
    op: Opcode,
    /// The time at which to start the stream.
    ///
    /// If set to `None`, this will play starting at the start of a stream.
    #[serde(serialize_with = "serialize_option_u64")]
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

/// A message sent to a node, relaying a voice state update received from
/// Discord.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceUpdate {
    /// The event data for the voice update.
    pub event: VoiceUpdateEvent,
    /// The guild which an update was dispatched for.
    pub guild_id: String,
    op: Opcode,
    /// The session ID for the voice session.
    pub session_id: String,
}

impl VoiceUpdate {
    /// Creates a new voice update message.
    pub fn new<S>(session_id: S, guild_id: S, token: S, endpoint: S) -> Self
        where S: Into<String> {
        let endpoint = endpoint.into();
        let guild_id = guild_id.into();
        let token = token.into();

        Self {
            event: VoiceUpdateEvent {
                endpoint: endpoint,
                guild_id: guild_id.clone(),
                token: token,
            },
            op: Opcode::VoiceUpdate,
            session_id: session_id.into(),
            guild_id,
        }
    }
}

/// Additional event data for a [`VoiceUpdate`].
///
/// [`VoiceUpdate`]: struct.VoiceUpdate.html
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
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

/// Utility function to serialize Option<u64> with no present value as 0 instead of null
fn serialize_option_u64<S: Serializer>(option: &Option<u64>, s: S) -> StdResult<S::Ok, S::Error> {
    let value = match *option {
        Some(value) => value,
        None => 0,
    };

    s.serialize_u64(value)
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
        )*
    };
}

impl_stuff_for_model! {
    Pause,
    Play,
    Seek,
    Stop,
    VoiceUpdate,
    Volume
}
