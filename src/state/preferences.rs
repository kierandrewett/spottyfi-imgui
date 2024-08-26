use url::Url;

#[derive(Debug, Default)]
pub struct WidgetStatePreferences {
    pub visible: bool,

    pub reveal_email: bool,

    pub credentials_email: Option<String>,
    pub credentials_password: Option<String>,
    pub credentials_auth_url: Option<Url>
}
