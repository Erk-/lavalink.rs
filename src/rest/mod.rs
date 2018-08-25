//! Hyper and reqwest implementations for working with the Lavalink REST API.
//!
//! To enable each implementation, add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies.lavalink]
//! default-features = false
//! features = [
//!     "hyper-support", // and/or
//!     "reqwest-support",
//! ]
//! ```

#[cfg(feature = "hyper")]
pub mod hyper;
#[cfg(feature = "reqwest")]
pub mod reqwest;

/// Information about loaded tracks.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename = "camelCase")]
pub struct Load {
    /// The type of track load.
    pub load_type: LoadType,
    /// The playlist information.
    pub playlist_info: Option<PlaylistInfo>,
    /// The list of tracks.
    pub tracks: Vec<LoadedTrack>,
}

/// The type of a track load.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename = "SCREAMING_SNAKE_CASE")]
pub enum LoadType {
    /// Indicator that loading the track failed.
    LoadFailed,
    /// Indicator that no matches were found.
    NoMatches,
    /// Indicator that a playlist item was loaded.
    PlaylistLoaded,
    /// Indicator that a search was made.
    SearchResult,
    /// Indicator that loading a track succeeded.
    TrackLoaded,
}

/// Meta information about a loaded track.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename = "camelCase")]
pub struct LoadedTrackInfo {
    /// The title of the track.
    pub title: String,
    /// The name of the author of the track.
    pub author: String,
    /// The length of the track in frames.
    pub length: i64,
    /// The ID of the track.
    pub identifier: String,
    /// The URI to the track.
    pub uri: String,
    /// Whether the track is a stream.
    pub is_stream: bool,
    /// Whether the track can be seeked.
    pub is_seekable: bool,
    /// The current position in the track.
    pub position: i64,
}

/// Information about a track.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoadedTrack {
    /// Base64 encoded representation of the track.
    pub track: String,
    /// Meta information about the track.
    pub info: LoadedTrackInfo,
}

/// Information about a playlist, if any.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename = "camelCase")]
pub struct PlaylistInfo {
    /// The name of the playlist.
    pub name: String,
    /// The item that was selected.
    pub selected_track: u64,
}
