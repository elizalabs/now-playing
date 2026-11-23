use clap::{ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum Player {
    Spotify,
    Tidal,
    MediaPlayer,
}

impl Player {
    pub fn id(&self) -> &'static str {
        match self {
            Player::Spotify => "Spotify.exe",
            Player::Tidal => "com.squirrel.TIDAL.TIDAL",
            Player::MediaPlayer => "Microsoft.ZuneMusic_8wekyb3d8bbwe!Microsoft.ZuneMusic",
        }
    }
}
