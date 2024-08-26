

use librespot::discovery::Credentials;
use oauth2::RefreshToken;

use crate::{commands::AppCommand, widget::theme::UITheme};

#[derive(PartialEq, Debug, Clone)]
pub enum AppFetchType {
    All,
    Volatile,

    Profile,
    Recommendations
}

#[derive(PartialEq, Debug, Clone)]
pub enum AppEvent {
    Ping,
    Painted,
    SetTheme(UITheme),
    InvalidateFontAtlas,
    SetInitialWindowState,
    Command(AppCommand),
    Focus,
    Login,
    Fetch(AppFetchType),
    StoreToken(Option<String>),
    FirstTimeLogin
}
