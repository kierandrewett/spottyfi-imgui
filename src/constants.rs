#![allow(unused)]

use easy_imgui::Color;

pub const UI_APP_NAME: &str = "Spottyfi";
pub const UI_APP_VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

pub const UI_DEFAULT_SCALE: f32 = 1.0;
pub const UI_SCALE_STEP: f32 = 0.1;
pub const UI_MIN_SCALE: f32 = 0.5;
pub const UI_MAX_SCALE: f32 = 2.5;

pub const UI_MODAL_PADDING: f32 = 0.1;
pub const UI_MODAL_AC_MIN_WINDOW_WIDTH: f32 = 700.0;
pub const UI_MODAL_AC_MIN_WINDOW_HEIGHT: f32 = 500.0;

pub const UI_SIDEBAR_WIDTH: f32 = 270.0;
pub const UI_SIDEBAR_MIN_WIDTH: f32 = 100.0;

// If you're changing this, make sure you change icons.png
// so each icon is this exact size.
pub const UI_ICONS_BASE_SIZE: u32 = 24;
pub const UI_ICONS_GAP_SIZE: u32 = 2;

pub const UI_ACCENT_COLOR: Color = Color {
    r: 0.11,
    g: 0.84,
    b: 0.375,
    a: 1.0,
};
pub const UI_LIGHT_WINDOW_BG_COLOR: Color = Color {
    r: 0.96,
    g: 0.96,
    b: 0.96,
    a: 1.0,
};
pub const UI_DARK_WINDOW_BG_COLOR: Color = Color {
    r: 0.15,
    g: 0.15,
    b: 0.15,
    a: 1.0,
};
pub const UI_LIGHT_CHROME_BG_COLOR: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};
pub const UI_DARK_CHROME_BG_COLOR: Color = Color {
    r: 0.1,
    g: 0.1,
    b: 0.1,
    a: 1.0,
};

pub const UI_PLAYER_BAR_HEIGHT: f32 = 90.0;

pub const UI_ROUTE_DEFAULT: &str = "Home";
pub const UI_ROUTE_SEARCH: &str = "Search";
pub const UI_ROUTE_PREFERENCES: &str = "Preferences";

pub const UI_ALBUM_ART_SIZE: f32 = 300.0;

pub const UI_DEFAULT_LOCALE: &str = "en_US";
