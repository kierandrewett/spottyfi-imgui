use std::{
    ffi::CString,
    sync::{Arc},
};

use components::{
    modals::{ModalManager, ModalType},
    player::PlayerArea,
    ComponentContext,
};
use easy_imgui::{
    easy_imgui_sys::ImGui_SetWindowFocus1,
    mint::Vector2,
    vec2, Color, ColorId, CustomRectIndex, DockNodeFlags, FontAtlasMut, FontId,
    ImGuiID, Image, ImageButton, Key, KeyChord, KeyMod, StyleValue, StyleVar, Ui,
};
use easy_imgui_window::{
    easy_imgui as imgui,
    winit::event_loop::EventLoopProxy,
};
use flex::FlexEngine;
use font::FontFamily;
use icons::{IconOffset, IconsManager};
use image::GenericImage;
use image::{load_from_memory, GenericImageView};
use num::clamp;
use preferences::{Preferences, PreferencesManager};
use theme::UITheme;
use tokio::{runtime::Handle, sync::Mutex};
use tracing::{debug, error, info, warn};

use crate::{
    api::SpotifyAPI, commands::AppCommand, constants::{
        self, UI_ALBUM_ART_SIZE, UI_DARK_CHROME_BG_COLOR, UI_DEFAULT_SCALE, UI_ICONS_BASE_SIZE,
        UI_LIGHT_CHROME_BG_COLOR, UI_MAX_SCALE, UI_MIN_SCALE, UI_ROUTE_DEFAULT,
        UI_ROUTE_PREFERENCES, UI_ROUTE_SEARCH,
    }, event::AppEvent, state::State, App, SpotifyAPILock
};

mod flex;
mod font;
mod fonts;

pub mod components;
pub mod icons;
pub mod preferences;
pub mod style;
pub mod theme;

#[derive(Default)]
pub struct Widget {
    pub ui_scale: f32,

    inter: FontFamily,

    font_body: FontId,
    font_bold: FontId,
    font_italic: FontId,

    font_super: FontId,
    font_small: FontId,

    font_h1: FontId,
    font_h2: FontId,
    font_h3: FontId,
    font_h4: FontId,

    glyph_album_art: CustomRectIndex,

    modals: ModalManager,
    icons: IconsManager,
    flex: FlexEngine,

    pub state: Arc<std::sync::Mutex<State>>,

    pub preferences: PreferencesManager,

    viewport_dockspace: ImGuiID,

    pub ready_for_window_events: bool
}

impl Widget {
    pub fn new() -> Widget {
        let inter = fonts::inter::build_font_family();

        let mut preferences = PreferencesManager::new();
        preferences.read_preferences();

        Widget {
            ui_scale: constants::UI_DEFAULT_SCALE,

            inter,

            modals: ModalManager::new(),
            icons: IconsManager::new(),
            flex: FlexEngine::new(),
            preferences,

            viewport_dockspace: ImGuiID::default(),

            ..Default::default()
        }
    }

    pub fn init_window_state(
        &mut self,
        event_loop: &EventLoopProxy<AppEvent>,
        api: &SpotifyAPILock
    ) {
        let zoom_level = self
            .preferences
            .get()
            .and_then(|p| p.zoom_level)
            .unwrap_or(UI_DEFAULT_SCALE);

        self.set_ui_scale(event_loop, zoom_level);

        let is_authorised = api
            .lock()
            .unwrap()
            .is_authorised();

        if is_authorised {
            self.state.lock().unwrap().panes.home_visible = true;
        } else {
            self.state.lock().unwrap().panes.preferences.visible = true;
        }

        event_loop.send_event(AppEvent::SetInitialWindowState).ok();

        event_loop.send_event(AppEvent::InvalidateAPIData).ok();
    }

    pub fn load_icons(&mut self, atlas: &mut FontAtlasMut<'_, App>) {
        let icons_set = self.icons.load_icon_sets(self.ui_scale);

        if let Some(icons_set) = icons_set {
            self.icons.build_icons(icons_set, atlas);
        } else {
            warn!("No suitable icons set found for UI scale {}", self.ui_scale);
        }
    }

    pub fn build_custom_atlas(&mut self, atlas: &mut FontAtlasMut<'_, App>) {
        match self.inter.get_by_weight(font::FontWeight::Regular) {
            Some(font) => {
                self.font_body = atlas.add_font(font.build_font_info(17.0 * self.ui_scale));
                self.font_small = atlas.add_font(font.build_font_info(15.0 * self.ui_scale));
            }
            None => error!("Failed to load regular font!"),
        };

        match self.inter.get_by_weight(font::FontWeight::Medium) {
            Some(font) => {
                self.font_bold = atlas.add_font(font.build_font_info(17.0 * self.ui_scale));
                self.font_h4 = atlas.add_font(font.build_font_info(22.0 * self.ui_scale));
            }
            None => error!("Failed to load medium font!"),
        };

        match self.inter.get_by_weight(font::FontWeight::RegularItalic) {
            Some(font) => {
                self.font_italic = atlas.add_font(font.build_font_info(17.0 * self.ui_scale))
            }
            None => error!("Failed to load italic font!"),
        };

        match self.inter.get_by_weight(font::FontWeight::ExtraBold) {
            Some(font) => {
                self.font_super = atlas.add_font(font.build_font_info(74.0 * self.ui_scale))
            }
            None => error!("Failed to load extrabold font!"),
        };

        match self.inter.get_by_weight(font::FontWeight::Bold) {
            Some(font) => self.font_h1 = atlas.add_font(font.build_font_info(48.0 * self.ui_scale)),
            None => error!("Failed to load bold font!"),
        };

        match self.inter.get_by_weight(font::FontWeight::Semibold) {
            Some(font) => {
                self.font_h2 = atlas.add_font(font.build_font_info(32.0 * self.ui_scale));
                self.font_h3 = atlas.add_font(font.build_font_info(28.0 * self.ui_scale));
            }
            None => error!("Failed to load semibold font!"),
        };

        self.load_icons(atlas);

        let album_art = load_from_memory(include_bytes!("assets/album_art.webp")).expect("failed");

        self.glyph_album_art = atlas.add_custom_rect_font_glyph(
            self.font_body,
            std::char::from_u32(1000).unwrap(),
            Vector2 {
                x: UI_ALBUM_ART_SIZE as u32,
                y: UI_ALBUM_ART_SIZE as u32,
            },
            0.0,
            vec2(0.0, 0.0),
            move |_, img| {
                let binding = album_art.clone();
                let icon = binding.view(0, 0, UI_ALBUM_ART_SIZE as u32, UI_ALBUM_ART_SIZE as u32);

                img.copy_from(&*icon, 0, 0).unwrap();
            },
        );
    }

    pub fn paint_ui(
        &mut self,
        event_loop: &EventLoopProxy<AppEvent>,
        ui: &imgui::Ui<App>,
        api: &SpotifyAPILock
    ) {
        let mut context = ComponentContext {
            widget: self,
            event_loop,
            ui,
            api
        };

        info!("a");

        let is_authorised = context.api
            .lock()
            .unwrap()
            .is_authorised();

        info!("b");

        ui.dock_space_over_viewport(
            1,
            if is_authorised {
                DockNodeFlags::None
            } else {
                DockNodeFlags::NoUndocking | DockNodeFlags::AutoHideTabBar
            },
        );

        ui.with_push((StyleVar::WindowBorderSize, StyleValue::F32(0.0)), || {
            let state_arc = Arc::clone(&context.widget.state);

            let current_theme = &state_arc.lock().unwrap().current_theme;

            ui.with_push(
                (
                    ColorId::WindowBg,
                    match current_theme {
                        UITheme::Dark => UI_DARK_CHROME_BG_COLOR,
                        _ => UI_LIGHT_CHROME_BG_COLOR,
                    },
                ),
                || {
                    let player_area = context
                        .widget
                        .preferences
                        .get()
                        .and_then(|p| p.player_bar)
                        .and_then(|p| p.area)
                        .unwrap();

                    components::menubar::build(&mut context);

                    if player_area == PlayerArea::Outside {
                        components::player::build(&mut context);
                    }

                    components::sidebar::build(&mut context);

                    if player_area == PlayerArea::Inside {
                        components::player::build(&mut context);
                    }
                },
            );
        });

        if context.widget.modals.has(ModalType::About) {
            components::modals::about::build(&mut context);
        }

        components::panes::build(&mut context);

        ui.show_demo_window(Some(&mut true));

        context.widget.handle_keyboard_shortcuts(event_loop, ui);

        event_loop.send_event(AppEvent::Painted).ok();
    }

    fn handle_keyboard_shortcuts(
        &mut self,
        event_loop: &EventLoopProxy<AppEvent>,
        ui: &imgui::Ui<App>,
    ) {
        // Begin critical keyboard shortcuts
        // These shortcuts are essential to general
        // operations of the application.
        // =================================================
        let esc = Key::Escape;

        let exit_chord = KeyChord::new(KeyMod::Ctrl | KeyMod::Shift, Key::Q);

        let zoom_in_chord = KeyChord::new(KeyMod::Ctrl, Key::Equal);
        let zoom_out_chord = KeyChord::new(KeyMod::Ctrl, Key::Minus);
        let zoom_reset_chord = KeyChord::new(KeyMod::Ctrl, Key::Num0);

        // This must be handled first!
        // Give the user an escape route if stuff goes wrong.
        if ui.is_keychord_pressed(exit_chord) {
            self.send_command(event_loop, AppCommand::Quit);
        }

        if ui.is_keychord_pressed(zoom_in_chord) {
            self.send_command(event_loop, AppCommand::ZoomIn);
        }

        if ui.is_keychord_pressed(zoom_out_chord) {
            self.send_command(event_loop, AppCommand::ZoomOut);
        }

        if ui.is_keychord_pressed(zoom_reset_chord) {
            self.send_command(event_loop, AppCommand::ZoomReset);
        }

        if ui.is_key_pressed(esc) {
            // If we have an open modal, close the first one available.
            if let Some(open_modal) = self.modals.first() {
                self.close_modal(open_modal.clone());
            }
        }

        // Begin non-critical keyboard shortcuts
        // =================================================
        if !ui.is_blocking_modal() {
            let preferences_chord = KeyChord::new(KeyMod::Ctrl, Key::P);

            if ui.is_keychord_pressed(preferences_chord) {
                self.send_command(event_loop, AppCommand::Navigate(UI_ROUTE_PREFERENCES));
            }
        }
    }

    fn send_command(&self, event_loop: &EventLoopProxy<AppEvent>, command: AppCommand) {
        event_loop.send_event(AppEvent::Command(command)).ok();
    }

    pub fn create_image(&self, ui: &Ui<App>, texture: CustomRectIndex, scale: f32) -> Image {
        ui.image_with_custom_rect_config(texture, scale)
    }

    pub fn create_image_button(
        &self,
        ui: &Ui<App>,
        texture: CustomRectIndex,
        scale: f32,
    ) -> ImageButton<&str> {
        ui.image_button_with_custom_rect_config("Image", texture, scale)
    }

    pub fn create_icon(&self, ui: &Ui<App>, icon_offset: IconOffset, size: f32, color: Color) {
        let icon_image = self.icons.get_icon(icon_offset.clone());

        let icon_scale =
            (size / UI_ICONS_BASE_SIZE as f32 / self.icons.icons_preferred_scale as f32)
                * self.ui_scale;

        debug!(
            "Rendering icon {},{} at scale {}",
            icon_offset.clone().x,
            icon_offset.clone().y,
            icon_scale
        );

        self
            .create_image(ui, icon_image, icon_scale)
            .tint_col(color)
            .build()
    }

    pub fn create_icon_button(
        &self,
        ui: &Ui<App>,
        icon_offset: IconOffset,
        size: f32,
        color: Color,
        bg_color: Color,
        bg_hover_color: Color,
        bg_active_color: Color,
        roundness: f32,
    ) -> bool {
        let icon_image = self.icons.get_icon(icon_offset.clone());

        let icon_scale =
            (size / UI_ICONS_BASE_SIZE as f32 / self.icons.icons_preferred_scale as f32)
                * self.ui_scale;

        debug!(
            "Rendering icon {},{} at scale {}",
            icon_offset.clone().x,
            icon_offset.clone().y,
            icon_scale
        );

        ui.with_push(
            (
                (ColorId::Button, bg_color),
                (ColorId::ButtonHovered, bg_hover_color),
                (ColorId::ButtonActive, bg_active_color),
                (StyleVar::FrameRounding, StyleValue::F32(roundness)),
            ),
            || {
                self.create_image_button(ui, icon_image, icon_scale)
                    .tint_col(color)
                    .build()
            },
        )
    }

    pub fn set_ui_scale(&mut self, event_loop: &EventLoopProxy<AppEvent>, scale: f32) {
        self.ui_scale = clamp(scale, UI_MIN_SCALE, UI_MAX_SCALE);

        event_loop.send_event(AppEvent::InvalidateFontAtlas).ok();
    }

    pub fn get_theme(&self) -> UITheme {
        self.preferences
            .get()
            .and_then(|p| p.theme)
            .unwrap_or(UITheme::System)
    }

    pub fn set_theme(&mut self, theme: UITheme, store: bool) {
        if store {
            self.preferences.set(Preferences {
                theme: Some(theme),
                ..Default::default()
            });
        }

        self.state.lock().unwrap().current_theme = theme;
    }

    pub fn open_shell_url(&self, url: &str) -> Result<(), std::io::Error> {
        info!("Opening external URL: {:?}", url);

        open::that(url)
    }

    pub fn open_modal(&mut self, modal: ModalType) {
        self.modals.add(modal);
    }

    pub fn close_modal(&mut self, modal: ModalType) {
        self.modals.remove(modal);
    }

    pub fn router(&mut self, route: &'static str) {
        match route {
            UI_ROUTE_DEFAULT => self.state.lock().unwrap().panes.home_visible = true,
            UI_ROUTE_SEARCH => self.state.lock().unwrap().panes.search.visible = true,

            UI_ROUTE_PREFERENCES => self.state.lock().unwrap().panes.preferences.visible = true,
            _ => warn!("No application route matching '{}'", route),
        }

        unsafe {
            let c_str = CString::new(route).expect("Failed to cast route to C string");
            let c_str_ptr = c_str.as_ptr();

            ImGui_SetWindowFocus1(c_str_ptr);
        }
    }
}
