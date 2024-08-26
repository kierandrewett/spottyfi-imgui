// Credit to https://github.com/jpochyla/psst/blob/a492c33e5d314b40ef7c6ed6634e4fbe4857a8ad/psst-core/src/session/access_token.rs#L10
pub const SPOTIFY_SCOPES: &str = "streaming,user-read-email,user-read-private,playlist-read-private,playlist-read-collaborative,playlist-modify-public,playlist-modify-private,user-follow-modify,user-follow-read,user-library-read,user-library-modify,user-top-read,user-read-recently-played";
pub const SPOTIFY_CLIENT_ID: &str = "65b708073fc0480ea92a077233ca87bd";
pub const SPOTIFY_DEVICE_NAME: &str = "Spottyfi";

pub const SPOTIFY_API_URL: &str = "https://api.spotify.com/v1/";

pub const SPOTIFY_OAUTH_AUTHORISE_URL: &str = "https://accounts.spotify.com/authorize";
pub const SPOTIFY_OAUTH_TOKEN_URL: &str = "https://accounts.spotify.com/api/token";

pub const SPOTIFY_ACCOUNTS_URL: &str = "https://www.spotify.com/account/overview/";

pub const SPOTIFY_CATEGORY_ID_MADE_FOR_YOU: &str = "0JQ5DAt0tbjZptfcdMSKl3";

pub const SPOTIFY_CATEGORIES_INTERNAL: [&str; 0] = [
    
];
