use crate::api::SpotifyAPIError;

#[derive(Debug, Default)]
pub struct WidgetStatePreferences {
    pub visible: bool,

    pub credentials_email: String,
    pub credentials_password: String,
}
