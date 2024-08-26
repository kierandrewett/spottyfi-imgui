use std::{default, sync::Arc};

use rspotify_model::SearchResult;
use tokio::task::{AbortHandle, JoinHandle};

use crate::api::{error::SpotifyAPIError, models::search::SearchResults};

#[derive(Debug, Default)]
pub enum WidgetStateSearchResults {
    #[default]
    None,
    Fetching,
    Fetched(Result<SearchResults, SpotifyAPIError>)
}

#[derive(Debug, Default)]
pub struct WidgetStateSearch {
    pub visible: bool,

    pub search_value: String,
    pub last_search_value: String,

    pub search_task: Option<AbortHandle>,
    pub search_results: WidgetStateSearchResults,
}