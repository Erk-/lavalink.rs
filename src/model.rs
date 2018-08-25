//! A collection of messages to send to and receive from the LavaLink node.

use serde::Serializer;
use super::opcodes::Opcode;
use std::result::Result as StdResult;

/// An incoming message from the node.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum IncomingMessage {
    /// Indicator that this is a PlayerUpdate payload.
    PlayerUpdate(PlayerUpdate),
    /// Indicator that this is a Stats payload.
    Stats(Stats),
}

/// An outgoing message to the node.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum OutgoingMessage {
    /// Indicator that this is a Destroy payload.
    Destroy(Destroy),
    /// Indicator that this is a Pause payload.
    Pause(Pause),
    /// Indicator that this is a Play payload.
    Play(Play),
    /// Indicator that this is a Seek payload.
    Seek(Seek),
    /// Indicator that this is a Stop payload.
    Stop(Stop),
    /// Indicator that this is a VoiceUpdate payload.
    VoiceUpdate(VoiceUpdate),
    /// Indicator that this is a Volume payload.
    Volume(Volume),
}

/// A message sent to a node to destroy a player.
///
/// This is useful if you want to move to a new node for a voice connection.
/// This does not affect the voice state.
///
/// **Note**: This is only sent to a node.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Destroy {
    /// The ID of the guild.
    pub guild_id: String,
    op: Opcode,
}

impl Destroy {
    /// Creates a new `Destroy` message.
    ///
    /// # Examples
    ///
    /// Destroy the player for a guild:
    ///
    /// ```rust,no_run
    /// use lavalink::model::Destroy;
    ///
    /// let _msg = Destroy::new("381880193251409931");
    /// ```
    pub fn new(guild_id: impl Into<String>) -> Self {
        Self::_new(guild_id.into())
    }

    fn _new(guild_id: String) -> Self {
        Self {
            op: Opcode::Destroy,
            guild_id,
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
    #[inline]
    pub fn new(guild_id: impl Into<String>, pause: bool) -> Self {
        Self::_new(guild_id.into(), pause)
    }

    fn _new(guild_id: String, pause: bool) -> Self {
        Self {
            op: Opcode::Pause,
            guild_id,
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
    #[inline]
    pub fn new(
        guild_id: impl Into<String>,
        track: impl Into<String>,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Self {
        Self::_new(guild_id.into(), track.into(), start_time, end_time)
    }

    fn _new(
        guild_id: String,
        track: String,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Self {
        Self {
            op: Opcode::Play,
            guild_id,
            end_time,
            start_time,
            track,
        }
    }
}

/// Position information about a player, including the Unix timestamp.
///
/// **Note**: This is only received from a node.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerUpdate {
    /// The ID of the guild.
    pub guild_id: String,
    op: Opcode,
    /// The new state information.
    pub state: PlayerUpdateState,
}

impl PlayerUpdate {
    /// Creates a new `PlayerUpdate` message.
    ///
    /// # Examples
    ///
    /// Create a new player update message with position 60000 at timestamp
    /// 1535170125:
    ///
    /// ```rust,no_run
    /// use lavalink::model::{PlayerUpdate, PlayerUpdateEvent};
    ///
    /// let _msg = PlayerUpdate::new(
    ///     "381880193251409931",
    ///     1535170125,
    ///     60000,
    /// );
    /// ```
    #[inline]
    pub fn new(guild_id: impl Into<String>, time: u64, position: i64) -> Self {
        Self::_new(guild_id.into(), time, position)
    }

    fn _new(guild_id: String, time: u64, position: i64) -> Self {
        Self {
            op: Opcode::PlayerUpdate,
            state: PlayerUpdateState::new(time, position),
            guild_id,
        }
    }
}

/// State about a player update.
///
/// **Note**: This is only received from a node.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerUpdateState {
    position: i64,
    time: u64,
}

impl PlayerUpdateState {
    /// Creates a new set of `PlayerUpdateState` data.
    ///
    /// # Examples
    ///
    /// Create a new player update state with position 60000 at timestamp
    /// 1535170125:
    ///
    /// ```rust,no_run
    /// use lavalink::model::PlayerUpdateEvent;
    ///
    /// let _msg = PlayerUpdateEvent::new(60000, 1535170125);
    /// ```
    pub fn new(time: u64, position: i64) -> Self {
        Self {
            position,
            time,
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
    #[inline]
    pub fn new(guild_id: impl Into<String>, position: i64) -> Self {
        Self::_new(guild_id.into(), position)
    }

    fn _new(guild_id: String, position: i64) -> Self {
        Self {
            op: Opcode::Seek,
            guild_id,
            position,
        }
    }
}

/// A payload containing statistics about a node.
///
/// **Note**: This is only received from a node.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    /// The CPU usage of the node.
    pub cpu: StatsCpu,
    /// The frame information of the node.
    pub frames: Option<StatsFrames>,
    /// The memory usage of the node.
    pub memory: StatsMemory,
    /// The number of players, both active and inactive.
    pub players: i32,
    /// The number of active players.
    pub playing_players: i32,
    /// The uptime of the node.
    pub uptime: i64,
}

/// The CPU usage of a node.
///
/// **Note**: This is only received from a node.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsCpu {
    /// The number of CPU cores available.
    pub cores: i64,
    /// The system load.
    pub system_load: f64,
    /// The lavalink node load.
    pub lavalink_load: f64,
}

/// The statistics about a node's frames.
///
/// **Note**: This is only received from a node.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsFrames {
    /// The average number of frames sent per minute.
    #[serde(rename = "sent")]
    pub average_sent_per_minute: i64,
    /// The average number of nulled frames per minute.
    #[serde(rename = "nulled")]
    pub average_nulled_per_minute: i64,
    /// The average frame deficit per minute.
    #[serde(rename = "deficit")]
    pub average_deficit_per_minute: i64,
}

/// The memory usage of a node.
///
/// **Note**: This is only received from a node.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsMemory {
    /// The allocated amount of memory.
    pub allocated: i64,
    /// The free amount of memory.
    pub free: i64,
    /// The reservable amount of memory.
    pub reservable: i64,
    /// The used amount of memory.
    pub used: i64,
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
    #[inline]
    pub fn new(guild_id: impl Into<String>) -> Self {
        Self::_new(guild_id.into())
    }

    fn _new(guild_id: String) -> Self {
        Self {
            op: Opcode::Stop,
            guild_id,
        }
    }
}

/// A message sent to a node, relaying a voice state update received from
/// Discord.
#[derive(Clone, Debug, Deserialize, Serialize)]
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
    #[inline]
    pub fn new(
        session_id: impl Into<String>,
        guild_id: impl Into<String>,
        token: impl Into<String>,
        endpoint: impl Into<String>,
    ) -> Self {
        Self::_new(
            session_id.into(),
            guild_id.into(),
            token.into(),
            endpoint.into(),
        )
    }

    fn _new(
        session_id: String,
        guild_id: String,
        token: String,
        endpoint: String,
    ) -> Self {
        Self {
            event: VoiceUpdateEvent::new(endpoint, guild_id.as_ref(), token),
            op: Opcode::VoiceUpdate,
            session_id,
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

impl VoiceUpdateEvent {
    /// Creates a new voice update event.
    #[inline]
    pub fn new(
        endpoint: impl Into<String>,
        guild_id: impl Into<String>,
        token: impl Into<String>,
    ) -> Self {
        Self::_new(endpoint.into(), guild_id.into(), token.into())
    }

    fn _new(endpoint: String, guild_id: String, token: String) -> Self {
        Self {
            endpoint,
            guild_id,
            token,
        }
    }
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
    #[inline]
    pub fn new(guild_id: impl Into<String>, volume: i32) -> Self {
        Self::_new(guild_id.into(), volume)
    }

    fn _new(guild_id: String, volume: i32) -> Self {
        Self {
            op: Opcode::Volume,
            guild_id,
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
