use std::sync::Arc;

use rspotify_model::{Category, Page, PrivateUser, SimplifiedPlaylist};
use super::models::{recommendations::BrowseRecommendations, user::UserImpl};

#[derive(Debug, Clone)]
pub enum SpotifyAPIDataError {
    NotAuthenticated,
    RequestError(),
}

#[derive(Debug, Clone, Default)]
pub struct SpotifyAPIData {
    pub profile: Option<PrivateUser>
}