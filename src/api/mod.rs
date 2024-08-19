
use librespot::{
    core::{
        config::SessionConfig,
        keymaster::{self, Token},
        mercury::MercuryError,
        session::{Session, SessionError},
    },
    discovery::Credentials,
};
use strum_macros::Display;
use tracing::{error, info};

use crate::constants::{SPOTIFY_CLIENT_ID, SPOTIFY_SCOPES};

#[derive(Default)]
pub struct SpotifyAPI {
    pub busy_flags: SpotifyAPIBusyFlags,

    pub session: Option<Result<Token, SpotifyAPIError>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpotifyAPICredentials {
    pub username: String,
    pub password: String,
}

bitflags::bitflags! {
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    pub struct SpotifyAPIBusyFlags: u8 {
        const Busy = 0x01;
        const BusyLoggingIn = 0x02;
    }
}

#[derive(Debug, Display, Default)]
pub enum SpotifyAPIError {
    #[default]
    Default,

    ValidationFailure(&'static str),
    TokenFailure(MercuryError),
    SessionConnectFailure(SessionError),
}

impl SpotifyAPI {
    pub async fn try_login(credentials: SpotifyAPICredentials) -> Result<Token, SpotifyAPIError> {
        match SpotifyAPI::login(credentials).await {
            Ok(token) => Ok(token),
            Err(err) => {
                error!("Failed to authenticate with Spotify: {:?}", err);

                Err(err)
            }
        }
    }

    async fn login(credentials: SpotifyAPICredentials) -> Result<Token, SpotifyAPIError> {
        info!("Attempting to authenticate with Spotify...");

        let session_config = SessionConfig::default();

        let safe_username = credentials.username.trim();
        let safe_password = credentials.password.trim();

        if safe_username.is_empty() || safe_password.is_empty() {
            return Err(SpotifyAPIError::ValidationFailure(
                "Username or password is empty.",
            ));
        }

        let api_credentials = Credentials::with_password(safe_username, safe_password);

        let (session, _) = Session::connect(session_config, api_credentials, None, false)
            .await
            .map_err(SpotifyAPIError::SessionConnectFailure)?;

        keymaster::get_token(&session, SPOTIFY_CLIENT_ID, SPOTIFY_SCOPES)
                .await
                .map_err(SpotifyAPIError::TokenFailure)
    }

    pub fn is_authorised(&self) -> bool {
        self.session
            .as_ref()
            .and_then(|r| r.as_ref().ok())
            .is_some()
    }

    pub fn new() -> Self {
        SpotifyAPI {
            ..Default::default()
        }
    }
}
