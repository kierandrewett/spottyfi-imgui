use developer::WidgetStateDeveloper;
use preferences::WidgetStatePreferences;
use search::WidgetStateSearch;

use crate::api::{data::SpotifyAPIData, error::SpotifyAPIError, models::recommendations::{BrowseRecommendationSections, BrowseRecommendations}};

use super::theme::UITheme;

pub mod developer;
pub mod search;
pub mod preferences;

#[derive(Debug, Default)]
pub struct State {
    pub current_theme: UITheme,

    pub home_visible: bool,

    pub preferences: WidgetStatePreferences,
    pub search: WidgetStateSearch,

    pub recommendations: Option<BrowseRecommendations>,

    #[cfg(debug_assertions)]
    pub developer: WidgetStateDeveloper,
}
