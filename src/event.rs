use std::fmt;

use crate::{commands::AppCommand, widget::theme::UITheme};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AppEvent {
    Ping,
    Painted,
    SetTheme(UITheme),
    InvalidateFontAtlas,
    Command(AppCommand),
    FocusWindow(&'static str),
}

impl fmt::Display for AppEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
