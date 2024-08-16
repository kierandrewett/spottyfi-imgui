use dark_light::Mode;
use detect_desktop_environment::DesktopEnvironment;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Default, PartialEq)]
pub enum UITheme {
    #[default]
    #[serde(rename = "system")]
    System,
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
}

pub fn detect_os_theme() -> UITheme {
    fn detect_wrapper() -> UITheme {
        match dark_light::detect() {
            Mode::Dark => UITheme::Dark,
            _ => UITheme::Light,
        }
    }

    if let Some(desktop_env) = DesktopEnvironment::detect() {
        // Check color-scheme on Gnome DEs over the legacy GTK theme
        if desktop_env == DesktopEnvironment::Gnome {
            match dconf_rs::get_string("/org/gnome/desktop/interface/color-scheme") {
                Ok(theme) => {
                    if theme.to_lowercase().contains("dark") {
                        UITheme::Dark
                    } else {
                        UITheme::Light
                    }
                }
                Err(_) => UITheme::Light,
            }
        } else {
            detect_wrapper()
        }
    } else {
        detect_wrapper()
    }
}
