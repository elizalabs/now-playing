mod cleanup_timespan;
mod media_session;
mod media_status;
mod player_options;

use std::panic;

use crate::media_session::MediaSession;
use crate::media_status::MediaStatus;
use crate::player_options::Player;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = clap::crate_name!(),
    author = clap::crate_authors!(),
    version = clap::crate_version!(),
    propagate_version = true,
    about = clap::crate_description!()
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Specify a player instead of letting Windows decide
    #[arg(short, long, value_enum)]
    player: Option<Player>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Print the title of the media
    Title,
    /// Print the artist of the media
    Artist,
    /// Print the album
    Album,
    /// Print the current position in the media
    /// (may be delayed for a few seconds due to WinRT restrictions)
    Position,
    /// Print the length of the media
    Duration,
    /// Print the status of the media
    Status,
    /// Play the media if it was previously paused
    Play,
    /// Pause the media if it was previously playing
    Pause,
    /// Toggle the state of the media between playing and paused
    Toggle,
    /// Skip to the next track
    Skip
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    // set a custom panic handler so we can print clean error messages
    panic::set_hook(Box::new(|info| {
        if let Some(s) = info.payload_as_str() {
            println!("Error: {s}");
        } else {
            println!("An unknown error occurred");
        }
    }));

    let cli = Cli::parse();

    if let Ok(media_session) = MediaSession::new(cli.player).await {
        if let Some(command) = cli.command {
            match command {
                // commands that fetch the state
                Commands::Title => println!("{}", media_session.get_title()),
                Commands::Artist => println!("{}", media_session.get_artist()),
                Commands::Album => println!("{}", media_session.get_album()),
                Commands::Position => println!("{}", media_session.get_position()),
                Commands::Duration => println!("{}", media_session.get_duration()),
                Commands::Status => println!("{:#?}", media_session.get_status()),
                // commands that modify the state
                Commands::Play => println!("{}", media_session.play()),
                Commands::Pause => println!("{}", media_session.pause()),
                Commands::Toggle => println!("{}", media_session.toggle()),
                Commands::Skip => println!("{}", media_session.skip()),
            }
        } else {
            println!("{}", media_session);
        }
    } else {
        println!("No media playing");
    }

    Ok(())
}
