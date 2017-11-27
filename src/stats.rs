//! A collection of statistics about a LavaLink node.


/// Information about audio frame averages on the LavaLink audio node.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct FrameStats {
    /// Average number of frames sent per minute.
    pub sent: i32,
    /// Average number of frames nulled per minute.
    pub nulled: i32,
    /// Average number of frames deficit per minute.
    pub deficit: i32,
}

/// A set of information about the memory usage of a device.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct MemoryStats {
    /// The amount of free memory that is not allocated.
    pub free: i64,
    /// The amount of memory that is used (i.e. without cache and buffer).
    ///
    /// This is most likely to be more useful to you than [`allocated`].
    ///
    /// [`allocated`]: #structfield.allocated
    pub used: i64,
    /// The amount of memory that is allocated (e.g. used, cached, buffered).
    pub allocated: i64,
    /// The amount of memory that is reservable.
    pub reservable: i64,
}

/// A set of information about the CPU and its usage of a device.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuStats {
    /// The number of cores available to the device.
    pub cores: i32,
    /// The device's system load.
    pub system_load: f64,
    /// The amount of load used by LavaLink on the device.
    pub lavalink_load: f64,
}

/// Information about the statistics of a node.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStats {
    /// The number of players managed by the node.
    pub players: i32,
    /// The number of players that are currently streaming audio.
    pub playing_players: i32,
    /// The uptime of the node in seconds.
    pub uptime: i64,
    /// The memory usage of the node.
    pub memory: MemoryStats,
    /// The CPU usage of the node.
    pub cpu: CpuStats,
    /// The frame statistic averages of the node.
    pub frame_stats: Option<FrameStats>,
}
