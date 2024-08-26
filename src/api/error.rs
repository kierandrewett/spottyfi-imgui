use std::sync::Arc;

use super::{data::SpotifyAPIDataError, providers::oauth2::SpotifyAPIOAuthError};

#[derive(Debug, Default, Clone)]
pub enum SpotifyAPIError {
    #[default]
    Default,

    Unknown(&'static str),
    OAuth2Error(SpotifyAPIOAuthError),
    DataError(SpotifyAPIDataError),
    RequestError(Arc<reqwest::Error>),
    SerdeError(Arc<serde_json::Error>),
}
