//! A collection of messages to send to and receive from the LavaLink node.

use serde::Serializer;
use super::opcodes::Opcode;
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    result::Result as StdResult,
};

/// A representation of an error that occurred while creating a [`Band`].
///
/// [`Band`]: struct.Band.html
#[derive(Debug)]
pub enum BandError {
    /// Indicator that the band value is not within the valid range.
    ///
    /// Refer to [`Band::band`] for more information.
    ///
    /// [`Band::band`]: struct.Band.html#structfield.band
    BandInvalid,
    /// Indicator that the gain value is not within the valid range.
    ///
    /// Refer to [`Gain::gain`] for more information.
    ///
    /// [`Gain::gain`]: struct.Gain.html#structfield.gain
    GainInvalid,
}

impl Display for BandError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for BandError {
    fn description(&self) -> &str {
        use self::BandError::*;

        match self {
            BandInvalid => "The band value is not within range",
            GainInvalid => "The gain value is not within range",
        }
    }
}

/// An incoming message from the node.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IncomingMessage {
    /// Indicator that this is an event from the server.
    Event(Event),
    /// Indicator that this is a PlayerUpdate payload.
    PlayerUpdate(PlayerUpdate),
    /// Indicator that this is a Stats payload.
    Stats(Stats),
}

/// An outgoing message to the node.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
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

/// A band for an equalizer.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Band {
    /// There are 15 bands (0-14) that can be changed.
    pub band: u8,
    /// The multiplier for the band.
    ///
    /// Defaults to `0`.
    ///
    /// Valid values range from `-0.25` to `1.0`, where `-0.25` means the given
    /// band is completely muted, and `0.25` means it is doubled.
    ///
    /// Modifying the gain could also change the volume of the output.
    pub gain: f64,
    #[serde(default, skip_deserializing, skip_serializing)]
    nonexhaustive: (),
}

impl Band {
    /// Creates a new Band instance.
    ///
    /// Refer to the structfields for limits on what these values can be.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lavalink::model::Band;
    ///
    /// // These are values within valid ranges:
    /// assert!(Band::new(0, 0.25).is_ok());
    ///
    /// // While this is not:
    /// assert!(Band::new(23, -6.0).is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`BandError::BandInvalid`] when the band value was not in the
    /// valid range.
    ///
    /// Returns [`BandError::GainInvalid`] when the gain value was not in the
    /// valid range.
    pub fn new(
        band: u8,
        gain: f64,
    ) -> Result<Self, BandError> {
        if band > 14 {
            return Err(BandError::BandInvalid);
        }
        if gain < -0.25 || gain > 1.0 {
            return Err(BandError::GainInvalid);
        }

        Ok(Self {
            band,
            gain,
            nonexhaustive: (),
        })
    }
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

/// Use the equalizer for a guild.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Equalizer {
    /// The list of bands to send for the equalizer.
    pub bands: Vec<Band>,
    /// The ID of the guild.
    pub guild_id: String,
    op: Opcode,
}

impl Equalizer {
    /// Creates a new set of equalizer settings for a guild.
    ///
    /// # Examples
    ///
    /// Equalize a guild:
    ///
    /// ```rust,no_run
    /// # extern crate lavalink;
    /// #
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<Error>> {
    /// #
    /// use lavalink::model::{Band, Equalizer};
    ///
    /// let bands = vec![
    ///     Band::new(0, 0.25)?,
    ///     Band::new(1, 0.0)?,
    /// ];
    /// let _equalizer = Equalizer::new("381880193251409931", bands);
    /// #     Ok(())
    /// # }
    /// ```
    pub fn new(
        guild_id: impl Into<String>,
        bands: impl IntoIterator<Item = Band>,
    ) -> Self {
        Self::_new(guild_id.into(), bands.into_iter().collect::<Vec<_>>())
    }

    fn _new(guild_id: String, bands: Vec<Band>) -> Self {
        Self {
            op: Opcode::Equalizer,
            bands,
            guild_id,
        }
    }
}

/// An event from the server.
///
/// **Note**: This is only sent from a node.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Event {
    /// An indicator that a track ended.
    TrackEnd(EventTrackEnd),
    /// An indicator that an exception occurred while playing a track.
    TrackException(EventTrackException),
    /// An indicator that a track became stuck.
    TrackStuck(EventTrackStuck),
    /// An indicator that a WebSocket connection to Discord closed.
    WebSocketClosed(EventWebSocketClosed),
}

impl Event {
    /// Returns the guild ID of the event.
    pub fn guild_id(&self) -> &str {
        match self {
            Event::TrackEnd(e) => &e.guild_id,
            Event::TrackException(e) => &e.guild_id,
            Event::TrackStuck(e) => &e.guild_id,
            Event::WebSocketClosed(e) => &e.guild_id,
        }
    }
}

/// A track was ended.
///
/// **Note**: This is only sent from a node.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventTrackEnd {
    /// The guild ID of the affected player.
    pub guild_id: String,
    /// The reason for the track ending.
    pub reason: String,
    /// The track that ended.
    pub track: String,
    op: Opcode,
}

impl EventTrackEnd {
    /// Creates a new EventTrackException instance.
    #[inline]
    pub fn new(
        guild_id: impl Into<String>,
        reason: impl Into<String>,
        track: impl Into<String>,
    ) -> Self {
        Self::_new(guild_id.into(), reason.into(), track.into())
    }

    fn _new(guild_id: String, reason: String, track: String) -> Self {
        Self {
            op: Opcode::Event,
            guild_id,
            reason,
            track,
        }
    }
}

/// An exception occurred while playing a track.
///
/// **Note**: This is only sent from a node.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventTrackException {
    /// The guild ID of the affected player.
    pub guild_id: String,
    /// The reason for the exception.
    pub error: String,
    /// The track that ended.
    pub track: String,
    op: Opcode,
}

impl EventTrackException {
    /// Creates a new EventTrackException instance.
    #[inline]
    pub fn new(
        guild_id: impl Into<String>,
        error: impl Into<String>,
        track: impl Into<String>,
    ) -> Self {
        Self::_new(guild_id.into(), error.into(), track.into())
    }

    fn _new(guild_id: String, error: String, track: String) -> Self {
        Self {
            op: Opcode::Event,
            error,
            guild_id,
            track,
        }
    }
}

/// A track became stuck.
///
/// **Note**: This is only sent from a node.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventTrackStuck {
    /// The guild ID of the affected player.
    pub guild_id: String,
    /// The reason for the track ending.
    pub threshold_ms: i64,
    /// The track that ended.
    pub track: String,
    op: Opcode,
}

impl EventTrackStuck {
    /// Creates a new EventTrackStuck instance.
    #[inline]
    pub fn new(
        guild_id: impl Into<String>,
        threshold_ms: i64,
        track: impl Into<String>,
    ) -> Self {
        Self::_new(guild_id.into(), threshold_ms, track.into())
    }

    fn _new(guild_id: String, threshold_ms: i64, track: String) -> Self {
        Self {
            op: Opcode::Event,
            guild_id,
            threshold_ms,
            track,
        }
    }
}

/// A WebSocket connection to Discord closed.
///
/// **Note**: This is only sent from a node.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventWebSocketClosed {
    /// Whether the remote host closed the connection.
    pub by_remote: bool,
    /// The close code from Discord.
    pub code: u16,
    /// The guild ID of the affected player.
    pub guild_id: String,
    /// The reason for the closing.
    pub reason: String,
    op: Opcode,
}

impl EventWebSocketClosed {
    /// Creates a new EventWebSocketClosed instance.
    #[inline]
    pub fn new(
        guild_id: impl Into<String>,
        by_remote: bool,
        code: u16,
        reason: impl Into<String>,
    ) -> Self {
        Self::_new(guild_id.into(), by_remote, code, reason.into())
    }

    fn _new(
        guild_id: String,
        by_remote: bool,
        code: u16,
        reason: String,
    ) -> Self {
        Self {
            op: Opcode::Event,
            by_remote,
            code,
            guild_id,
            reason,
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
    /// Whether to not replace the current playing song.
    ///
    /// Defaults to `false`. This means the default is to replace the current
    /// song. Set to `true` to avoid replacing the current song.
    pub no_replace: bool,
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
            no_replace: false,
            op: Opcode::Play,
            end_time,
            guild_id,
            start_time,
            track,
        }
    }

    /// Creates a new `Play` message with `noReplace` set.
    ///
    ///
    #[inline]
    pub fn with_no_replace(
        guild_id: impl Into<String>,
        track: impl Into<String>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        no_replace: bool,
    ) -> Self {
        Self::_with_no_replace(
            guild_id.into(),
            track.into(),
            start_time,
            end_time,
            no_replace,
        )
    }

    fn _with_no_replace(
        guild_id: String,
        track: String,
        start_time: Option<u64>,
        end_time: Option<u64>,
        no_replace: bool,
    ) -> Self {
        Self {
            op: Opcode::Play,
            end_time,
            guild_id,
            no_replace,
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
    /// use lavalink::model::PlayerUpdate;
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
    /// The current position of the player.
    pub position: Option<i64>,
    /// The Unix timestamp of the update.
    pub time: u64,
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
    /// use lavalink::model::PlayerUpdateState;
    ///
    /// let _msg = PlayerUpdateState::new(60000, 1535170125);
    /// ```
    pub fn new(time: u64, position: i64) -> Self {
        Self {
            position: Some(position),
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
    #[serde(rename = "frameStats")]
    pub frames: Option<StatsFrames>,
    /// The memory usage of the node.
    pub memory: StatsMemory,
    /// The number of players, both active and inactive.
    pub players: i32,
    /// The number of active players.
    pub playing_players: i32,
    /// The uptime of the node.
    pub uptime: i64,
    op: Opcode,
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
    pub average_sent_per_minute: f64,
    /// The average number of nulled frames per minute.
    #[serde(rename = "nulled")]
    pub average_nulled_per_minute: f64,
    /// The average frame deficit per minute.
    #[serde(rename = "deficit")]
    pub average_deficit_per_minute: f64,
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
    Destroy,
    Pause,
    Play,
    PlayerUpdate,
    Seek,
    Stats,
    Stop,
    VoiceUpdate,
    Volume
}

#[cfg(test)]
mod tests {
    use serde_json;
    use super::*;

    static PLAYER_UPDATE: &'static str = r#"{
  "guildId": "1",
  "op": "playerUpdate",
  "state": {
    "position": 45120,
    "time": 1537848743531
  }
}"#;

    static STATS: &'static str = r#"{
  "cpu": {
    "cores": 4,
    "systemLoad": 0.022558908466914995,
    "lavalinkLoad": 0.003833333333333
  },
  "frameStats": {
    "sent": 3000.0,
    "nulled": 0.0,
    "deficit": 0.0
  },
  "memory": {
    "allocated": 187695104,
    "free": 88357736,
    "reservable": 2013265920,
    "used": 99337368
  },
  "players": 1,
  "playingPlayers": 1,
  "uptime": 79943650,
  "op": "stats"
}"#;

    static TRACK_END: &'static str = r#"{
  "op": "event",
  "reason": "FINISHED",
  "type": "TrackEndEvent",
  "track": "foo",
  "guildId": "1"
}"#;

    #[test]
    fn test_track_end_event() {
        serde_json::from_str::<EventTrackEnd>(&TRACK_END).unwrap();
    }

    #[test]
    fn test_incoming_message_deser() {
        serde_json::from_str::<IncomingMessage>(STATS).unwrap();
    }

    #[test]
    fn test_stats_deser() {
        let stats = serde_json::from_str::<Stats>(STATS).unwrap();

        assert_eq!(serde_json::to_string_pretty(&stats).unwrap(), STATS);
    }

    #[test]
    fn test_player_update_deser() {
        let update = serde_json::from_str::<PlayerUpdate>(
            PLAYER_UPDATE,
        ).unwrap();
        assert_eq!(update.guild_id, "1");

        assert_eq!(
            serde_json::to_string_pretty(&update).unwrap(),
            PLAYER_UPDATE,
        );
    }
}
