use std::fmt;

use librespot::discovery::Credentials;

use crate::{commands::AppCommand, widget::theme::UITheme};

#[derive(PartialEq, Debug)]
pub enum AppEvent {
    Ping,
    Painted,
    SetTheme(UITheme),
    InvalidateFontAtlas,
    SetInitialWindowState,
    Command(AppCommand),
}
