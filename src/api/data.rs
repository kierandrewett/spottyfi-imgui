#[derive(Debug)]
pub struct SpotifyAPIData {
    pub profile: SpotifyAPIDataProfile
}

#[derive(Debug)]
pub struct SpotifyAPIDataProfile {
    pub username: String,
    pub display_name: Option<String>,
    pub country: String,
    pub email: String,
}