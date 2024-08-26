use rspotify_model::{FullArtist, FullTrack, Page, SearchResult, SimplifiedAlbum, SimplifiedPlaylist, SimplifiedTrack};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResults {
    pub albums: Option<Page<SimplifiedAlbum>>,
    pub artists: Option<Page<FullArtist>>,
    pub playlists: Option<Page<SimplifiedPlaylist>>,
    pub tracks: Option<Page<FullTrack>>,
}

impl SearchResults {
    pub fn is_empty(&self) -> bool {
        (
            self.albums.is_none() &&
            self.artists.is_none() &&
            self.playlists.is_none() &&
            self.tracks.is_none()
        )
    }
}