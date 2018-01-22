#[cfg(feature = "hyper")]
pub mod hyper;
#[cfg(feature = "reqwest")]
pub mod reqwest;

/// Meta information about a loaded track.
#[derive(Clone, Debug, Deserialize)]
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
    #[serde(rename = "isStream")]
    pub is_stream: bool,
    /// Whether the track can be seeked.
    #[serde(rename = "isSeekable")]
    pub is_seekable: bool,
    /// The current position in the track.
    pub position: i64,
}

/// Information about a track.
#[derive(Clone, Debug, Deserialize)]
pub struct LoadedTrack {
    /// Base64 encoded representation of the track.
    pub track: String,
    /// Meta information about the track.
    pub info: LoadedTrackInfo,
}
