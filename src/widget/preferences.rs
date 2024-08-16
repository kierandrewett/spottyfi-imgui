use std::{
    collections::HashMap,
    fmt::Error,
    fs::{create_dir_all, exists, File, OpenOptions},
    io::{read_to_string, BufReader, Read, Write},
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use easy_imgui::IntoCStr;
use serde::{Deserialize, Serialize};
use serde_merge::omerge;
use toml::to_string;
use tracing::{error, info, warn};

use crate::constants::UI_APP_NAME;

use super::{
    components::player::{PlayerArea, PlayerPosition},
    theme::UITheme,
};

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub struct Preferences {
    pub zoom_level: Option<f32>,
    pub theme: Option<UITheme>,

    pub player_bar: Option<PreferencesPlayerBar>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Default, Debug)]
pub struct PreferencesPlayerBar {
    pub area: Option<super::components::player::PlayerArea>,
    pub position: Option<super::components::player::PlayerPosition>,
}

#[derive(Clone, Default)]
pub struct PreferencesManager {
    config_dir: Option<PathBuf>,

    user_prefs: Option<Preferences>,
    data: Option<Preferences>,
}

impl PreferencesManager {
    pub fn default_prefs(&self) -> Preferences {
        Preferences {
            zoom_level: Some(1.0),
            theme: Some(UITheme::System),

            player_bar: Some(PreferencesPlayerBar {
                position: Some(PlayerPosition::Bottom),
                area: Some(PlayerArea::Outside),

                ..Default::default()
            }),

            ..Default::default()
        }
    }

    pub fn new() -> PreferencesManager {
        let mut config_dir = None;

        // Linux: ~/.config/spottyfi
        // macOS: /Users/User/Library/Application Support/com.Spottyfi.Spottyfi
        // Windows: C:\Users\User\AppData\Roaming\Spottyfi\Spottyfi\config
        if let Some(project_directory) = ProjectDirs::from("com", UI_APP_NAME, UI_APP_NAME) {
            config_dir = Some(project_directory.config_dir().to_path_buf());
        } else {
            warn!("ATTENTION!");
            warn!("No suitable directory found to store preferences.");
            warn!("Your session state and preferences will NOT be saved!");
        }

        PreferencesManager {
            config_dir,

            user_prefs: None,
            data: None,
        }
    }

    pub fn parse_preferences(&self, toml: &str) -> Result<Preferences, toml::de::Error> {
        toml::from_str::<Preferences>(toml)
    }

    pub fn serialize_preferences(
        &self,
        prefs: Preferences,
    ) -> Result<Option<String>, toml::ser::Error> {
        match toml::to_string_pretty::<Preferences>(&prefs) {
            Ok(serialized) => match self.parse_preferences(&serialized) {
                Ok(_) => Ok(Some(serialized)),
                Err(err) => {
                    warn!(
                        "Failed to parse newly serialized preferences {:#?}: {:#?}",
                        prefs, err
                    );
                    warn!("Serialized: {}", serialized);

                    Ok(None)
                }
            },
            Err(err) => Err(err),
        }
    }

    pub fn get_prefs_path(&self) -> Option<PathBuf> {
        self.config_dir
            .clone()
            .and_then(|d| Some(d.join("preferences.toml")))
    }

    pub fn get_prefs_path_str(&self) -> String {
        let get_prefs_path = self.get_prefs_path();

        if let Some(prefs_path) = get_prefs_path {
            prefs_path.to_str().unwrap_or("no string").to_string()
        } else {
            "no string".to_string()
        }
    }

    pub fn read_preferences(&mut self) {
        self.user_prefs = if let Some(dir) = &self.config_dir {
            if !exists(dir).unwrap_or(false) {
                match create_dir_all(dir) {
                    Ok(_) => {}
                    Err(err) => {
                        error!(
                            "Failed to create config dir {}: {:#?}",
                            dir.to_str().unwrap_or("no string"),
                            err
                        );
                    }
                }
            }

            if let Some(prefs_path) = self.get_prefs_path() {
                let prefs_path_str = &self.get_prefs_path_str();

                if !exists(&prefs_path.clone()).unwrap_or(false) {
                    match File::create(prefs_path_str) {
                        Ok(mut file) => match file.write_all(b"") {
                            Ok(_) => {}
                            Err(err) => error!(
                                "Failed to read preferences file at {}: {:#?}",
                                prefs_path_str, err
                            ),
                        },
                        Err(err) => error!(
                            "Failed to read preferences file at {}: {:#?}",
                            prefs_path_str, err
                        ),
                    };
                }

                let mut prefs_open_options = OpenOptions::new();

                match prefs_open_options.read(true).open(&prefs_path.clone()) {
                    Ok(mut handle) => {
                        let mut contents = String::new();

                        match handle.read_to_string(&mut contents) {
                            Ok(_) => {}
                            Err(err) => {
                                error!(
                                    "Failed to read preferences file at {}: {:#?}",
                                    prefs_path_str, err
                                );
                            }
                        }

                        match self.parse_preferences(&contents) {
                            Ok(parsed) => Some(parsed),
                            Err(err) => {
                                error!(
                                    "Failed to parse preferences file at {}: {:#?}",
                                    prefs_path_str, err
                                );

                                None
                            }
                        }
                    }
                    Err(err) => {
                        error!(
                            "Failed to load preferences file at {}: {:#?}",
                            prefs_path_str, err
                        );

                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        };

        self.data = match &self.user_prefs {
            Some(user_prefs) => match omerge(self.default_prefs(), user_prefs) {
                Ok(prefs) => prefs,
                Err(err) => {
                    error!(
                        "Failed to merge default preferences with user preferences file: {:#?}",
                        err
                    );

                    Some(self.default_prefs())
                }
            },
            None => Some(self.default_prefs()),
        };
    }

    pub fn write_preferences(&mut self, prefs: Preferences) -> Option<Preferences> {
        if let Some(prefs_path) = &self.get_prefs_path() {
            let prefs_path_str = &self.get_prefs_path_str();

            info!(
                "Writing new {:#?} to preferences file at {}",
                prefs, prefs_path_str
            );

            if exists(prefs_path).unwrap_or(false) {
                let mut prefs_write_options = OpenOptions::new();

                match prefs_write_options
                    .write(true)
                    .truncate(true)
                    .open(&prefs_path.clone())
                {
                    Ok(mut handle) => {
                        match self.serialize_preferences(prefs) {
                            Ok(parsed_opt) => {
                                match parsed_opt {
                                    Some(parsed) => {
                                        let bytes = parsed.trim().as_bytes();

                                        match handle.write_all(bytes) {
                                            Ok(_) => {
                                                // Re-read the prefs
                                                self.read_preferences();

                                                Some(prefs)
                                            }
                                            Err(err) => {
                                                error!(
                                                    "Failed to write to preferences file at {}: {:#?}",
                                                    prefs_path_str,
                                                    err
                                                );

                                                None
                                            }
                                        }
                                    }
                                    None => {
                                        error!(
                                            "Failed to serialize preferences file at {}",
                                            prefs_path_str
                                        );

                                        None
                                    }
                                }
                            }
                            Err(err) => {
                                error!(
                                    "Failed to serialize preferences file at {}: {:#?}",
                                    prefs_path_str, err
                                );

                                None
                            }
                        }
                    }
                    Err(err) => {
                        error!(
                            "Failed to load preferences file at {}: {:#?}",
                            prefs_path_str, err
                        );

                        None
                    }
                }
            } else {
                error!("Failed to write to preferences file as it does not exist!");

                None
            }
        } else {
            None
        }
    }

    pub fn get(&self) -> Option<Preferences> {
        self.data
    }

    pub fn set(&mut self, new_prefs: Preferences) -> Option<Preferences> {
        self.read_preferences();

        match omerge(self.data, new_prefs) {
            Ok(merged_prefs) => self.write_preferences(merged_prefs),
            Err(err) => {
                error!(
                    "Failed to merge current preferences with new preferences {:#?}: {:#?}",
                    new_prefs, err
                );

                None
            }
        }
    }
}
