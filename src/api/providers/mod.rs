use librespot::core::SessionConfig;
use spotify_rs::{auth::UnAuthenticated, client::Client};

pub mod oauth2;

pub struct SpotifyAPIAuthProvider<T> {
    client: T
}