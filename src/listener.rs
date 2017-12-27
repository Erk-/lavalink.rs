use ::prelude::*;
use ::player::AudioPlayer;

pub trait AudioPlayerListener: Send + Sync {
    fn player_pause(&self, player: &AudioPlayer);
    fn player_resume(&self, player: &AudioPlayer);
    fn track_start(&self, player: &AudioPlayer, track: &str);
    fn track_end(&self, player: &AudioPlayer, track: &str, reason: &str);
    fn track_exception(&self, player: &AudioPlayer, track: &str, exception: &str);
    fn track_stuck(&self, player: &AudioPlayer, track: &str, threshold: i64);
}
