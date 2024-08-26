use rspotify_model::{Category, FullArtist, FullTrack, Page, SimplifiedAlbum, SimplifiedPlaylist};
use tracing::{error, warn};

use crate::api::{constants::SPOTIFY_CATEGORY_ID_MADE_FOR_YOU, error::SpotifyAPIError, SpotifyAPI};

#[derive(Debug, Clone)]
pub enum BrowseRecommendationSections {
    Fetching,
    Sections(Result<Vec<BrowseRecommendationSection>, SpotifyAPIError>)
} 

#[derive(Debug, Clone)]
pub struct BrowseRecommendations {
    pub categories: Vec<Category>,
    pub featured: Page<SimplifiedPlaylist>,

    pub sections: Option<BrowseRecommendationSections>
}

#[derive(Debug, Clone)]
pub enum BrowseRecommendationItem {
    Album(SimplifiedAlbum),
    Artist(FullArtist),
    Playlist(SimplifiedPlaylist),
    Track(FullTrack)
}

#[derive(Debug, Clone)]
pub struct BrowseRecommendationSection {
    pub title: String,
    pub description: Option<String>,
    pub items: Vec<BrowseRecommendationItem>
}

impl BrowseRecommendations {
    pub async fn generate(&mut self, api: &SpotifyAPI, locale: String, playlists_limit: Option<u32>) {
        self.sections = Some(BrowseRecommendationSections::Sections({
            let mut sections: Vec<BrowseRecommendationSection> = Vec::new();

            for category in self.categories.clone() {
                match api.get_category_playlists(category.id.clone(), Some(playlists_limit.unwrap_or(5))).await {
                    Ok(playlists) => {
                        let section = BrowseRecommendationSection {
                            title: category.name,
                            description: None,
                            items: playlists.items
                                .into_iter()
                                .map(BrowseRecommendationItem::Playlist)
                                .collect()
                        };

                        sections.push(section);
                    },
                    Err(err) => {
                        error!("Failed to get playlists for category '{}': {:#?}", category.id.clone(), err);
                    }
                }
            }

            Ok(sections)
        }));
    }
}
