mod commands;
mod constants;
mod event;
mod imgui_additions;
mod utils;
mod widget;

use std::ops::{Deref, DerefMut};

use commands::AppCommand;
use constants::{
    SPOTIFY_ACCOUNTS_URL, UI_APP_NAME, UI_DARK_WINDOW_BG_COLOR, UI_DEFAULT_SCALE,
    UI_LIGHT_WINDOW_BG_COLOR, UI_SCALE_STEP,
};
use dark_light::Mode;
use detect_desktop_environment::DesktopEnvironment;
use easy_imgui_window::{
    easy_imgui as imgui,
    winit::{self, dpi::LogicalSize, event_loop::EventLoopProxy, window::Window},
    AppHandler, Application, Args, EventResult,
};
use event::AppEvent;
use tracing::{error, info};
use widget::{
    components::modals::ModalType,
    theme::{self, UITheme},
    Widget,
};
use winit::{event::WindowEvent, event_loop::EventLoop};

fn main() {
    tracing_subscriber::fmt().init();

    let event_loop = EventLoop::with_user_event().build().unwrap();

    let proxy = event_loop.create_proxy();

    let mut main = AppHandler::<App>::new(proxy);
    *main.attributes() = Window::default_attributes()
        .with_title(UI_APP_NAME)
        .with_min_inner_size(LogicalSize::new(256.0, 256.0));

    event_loop.run_app(&mut main).unwrap();
}

struct App {
    widget: Widget,
    event_loop_proxy: EventLoopProxy<AppEvent>,
}

impl Application for App {
    type UserEvent = AppEvent;
    type Data = EventLoopProxy<AppEvent>;

    fn new(args: Args<Self::Data>) -> App {
        let mut widget = Widget::new();
        let event_loop_proxy = args.data.clone();

        widget.init_window_state(&event_loop_proxy);

        App {
            widget,
            event_loop_proxy,
        }
    }

    fn user_event(&mut self, args: Args<Self::Data>, event: Self::UserEvent) {
        // Return early if this is a paint event
        if event == AppEvent::Painted {
            let mut imgui = unsafe { args.window.renderer().imgui().set_current() };

            imgui.set_allow_user_scaling(true);
            imgui.nav_enable_keyboard();

            let app_theme = match self.widget.preferences.get().and_then(|p| p.theme) {
                Some(UITheme::Light) => UITheme::Light,
                Some(UITheme::Dark) => UITheme::Dark,
                // Decects unknown values + system theme
                _ => theme::detect_os_theme(),
            };

            match app_theme {
                UITheme::Dark => {
                    imgui.style().set_colors_dark();

                    args.window
                        .renderer()
                        .set_background_color(Some(UI_DARK_WINDOW_BG_COLOR));
                }
                _ => {
                    imgui.style().set_colors_light();

                    args.window
                        .renderer()
                        .set_background_color(Some(UI_LIGHT_WINDOW_BG_COLOR));
                }
            };

            self.widget.set_theme(app_theme, false);

            return;
        }

        match event {
            AppEvent::InvalidateFontAtlas => {
                args.window.renderer().imgui().invalidate_font_atlas();
            }
            AppEvent::SetTheme(theme) => self.widget.set_theme(theme, true),
            AppEvent::Command(command) => {
                info!("Handling application command: {:?}", event);

                #[allow(unreachable_patterns)]
                match command {
                    AppCommand::About => self.widget.open_modal(ModalType::About),

                    AppCommand::Navigate(route) => self.widget.router(route),

                    AppCommand::ZoomIn => self
                        .widget
                        .set_ui_scale(&self.event_loop_proxy, self.widget.ui_scale + UI_SCALE_STEP),
                    AppCommand::ZoomOut => self
                        .widget
                        .set_ui_scale(&self.event_loop_proxy, self.widget.ui_scale - UI_SCALE_STEP),
                    AppCommand::ZoomReset => self
                        .widget
                        .set_ui_scale(&self.event_loop_proxy, UI_DEFAULT_SCALE),

                    AppCommand::OpenSpotifyAccount => {
                        match self.widget.open_shell_url(SPOTIFY_ACCOUNTS_URL) {
                            Ok(_) => {}
                            Err(err) => error!("Failed to open Spotify accounts URL: {:?}", err),
                        }
                    }

                    AppCommand::Quit => args.event_loop.exit(),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn window_event(&mut self, args: Args<Self::Data>, _event: WindowEvent, res: EventResult) {
        if res.window_closed {
            args.event_loop.exit();
        }
    }
}

impl imgui::UiBuilder for App {
    fn build_custom_atlas(&mut self, atlas: &mut imgui::FontAtlasMut<'_, Self>) {
        self.widget.build_custom_atlas(atlas);
    }

    fn do_ui(&mut self, ui: &imgui::Ui<App>) {
        widget::style::push_style(&self.widget, ui, || {
            self.widget.clone().paint_ui(&self.event_loop_proxy, ui);
        });
    }
}
