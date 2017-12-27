use parking_lot::Mutex;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::rc::Rc;
use super::model::{IntoWebSocketMessage, Pause, Play, Stop, Volume};
use websocket::OwnedMessage;
use ::prelude::*;
use ::listener::AudioPlayerListener;

type AudioPlayerMap = HashMap<u64, Arc<Mutex<AudioPlayer>>>;

// todo potentially split state into child struct to avoid mutable reference of AudioPlayer
// where mutablity should not be nessesary for non state fields
#[derive(Clone)]
pub struct AudioPlayer {
    pub sender: Arc<Mutex<Sender<OwnedMessage>>>,
    pub guild_id: u64,
    pub track: Option<String>,
    pub time: i64,
    pub position: i64,
    pub paused: bool,
    pub volume: i32,
    listener: Arc<Box<AudioPlayerListener>>,
}

impl AudioPlayer {
    fn new(sender: Arc<Mutex<Sender<OwnedMessage>>>, guild_id: u64, listener: Arc<Box<AudioPlayerListener>>) -> Self {
        Self {
            sender,
            guild_id,
            track: None,
            time: 0,
            position: 0,
            paused: false,
            volume: 100,
            listener,
        }
    }

    #[inline]
    fn send(&self, message: OwnedMessage) -> Result<()> {
        self.sender.lock().send(message).map_err(From::from)
    }

    pub fn play(
        &mut self,
        track: &str,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Result<()> {
        let result = self.send(Play::new(
            &self.guild_id.to_string()[..],
            track,
            start_time,
            end_time,
        ).into_ws_message()?);

        match result {
            Ok(_) => {
                self.track = Some(track.to_string());

                self.listener.track_start(self, track);
            },
            Err(e) => {
                error!("play websocket send error {:?}", e);
            },
        }

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        let result = self.send(Stop::new(
            &self.guild_id.to_string()[..],
        ).into_ws_message()?);

        match result {
            Ok(_) => {
                let track = self.track.clone().unwrap_or_else(|| "no track in state".to_string());
                self.track = None;

                self.listener.track_end(self, &track, "no reason");

                debug!("stopped playing track {:?}", track);
            },
            Err(e) => {
                error!("stop websocket send error {:?}", e);
            },
        }

        Ok(())
    }

    pub fn pause(&mut self, pause: bool) -> Result<()> {
        let result = self.send(Pause::new(
            &self.guild_id.to_string()[..],
            pause,
        ).into_ws_message()?);

        match result {
            Ok(_) => {
                self.paused = pause;

                if pause {
                    self.listener.player_pause(self);
                } else {
                    self.listener.player_resume(self);
                }

                debug!("pause audio player: {}", pause);
            },
            Err(e) => {
                error!("pause websocket send error {:?}", e);
            },
        }

        Ok(())
    }

    #[allow(unused)]
    pub fn seek(&mut self, position: i64) {
        unimplemented!()
    }

    pub fn volume(&mut self, volume: i32) -> Result<()> {
        let result = self.send(Volume::new(
            &self.guild_id.to_string()[..],
            volume,
        ).into_ws_message()?);

        match result {
            Ok(_) => {
                self.volume = volume;

                debug!("set volume {:?}", self.volume);
            },
            Err(e) => {
                error!("play websocket send error {:?}", e);
            },
        }

        Ok(())
    }
}

impl Debug for AudioPlayer {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_struct("AudioPlayer")
            .field("sender", &self.sender)
            .field("guild_id", &self.guild_id)
            .field("track", &self.track)
            .field("time", &self.time)
            .field("position", &self.position)
            .field("paused", &self.paused)
            .field("volume", &self.volume)
            .finish()
    }
}

#[derive(Clone)]
pub struct AudioPlayerManager {
    players: AudioPlayerMap,
    pub listener: Arc<AudioPlayerListener>,
}

impl AudioPlayerManager {
    pub fn new(listener: Arc<AudioPlayerListener>) -> Self {
        Self {
            players: HashMap::default(),
            listener,
        }
    }

    // utility assosiated function for creating AudioPlayer instances wrapped in Arc & Mutex
    fn new_player(&self, sender: Arc<Mutex<Sender<OwnedMessage>>>, guild_id: u64) -> Arc<Mutex<AudioPlayer>> {
        Arc::new(Mutex::new(AudioPlayer::new(sender, guild_id, self.listener.clone())))
    }

    pub fn has_player(&self, guild_id: &u64) -> bool {
        self.players.contains_key(guild_id)
    }

    pub fn get_player(&self, guild_id: &u64) -> Option<Arc<Mutex<AudioPlayer>>> {
        let player = match self.players.get(guild_id) {
            Some(player) => player,
            None => return None,
        };

        Some(Arc::clone(player))
    }

    pub fn create_player(&mut self, sender: Arc<Mutex<Sender<OwnedMessage>>>, guild_id: u64) -> Result<Arc<Mutex<AudioPlayer>>> {
        // we dont use #has_key yet because it would get its own players clone & mutex lock
        if self.players.contains_key(&guild_id) {
            return Err(Error::PlayerAlreadyExists);
        }

        let player = self.new_player(sender, guild_id);
        let _ = self.players.insert(guild_id, player);

        // unwrap because we can assert it exists after insertion
        let player = &self.players[&guild_id];
        Ok(Arc::clone(player))
    }
}

impl Debug for AudioPlayerManager {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_struct("AudioPlayerManager")
            .field("players", &self.players)
            .finish()
    }
}
