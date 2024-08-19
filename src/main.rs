mod api;
mod commands;
mod constants;
mod event;
mod imgui_additions;
mod utils;
mod widget;

use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

use api::SpotifyAPI;
use commands::AppCommand;
use constants::{
    SPOTIFY_ACCOUNTS_URL, UI_APP_NAME, UI_DARK_WINDOW_BG_COLOR, UI_DEFAULT_SCALE,
    UI_LIGHT_WINDOW_BG_COLOR, UI_SCALE_STEP,
};
use dark_light::Mode;
use detect_desktop_environment::DesktopEnvironment;
use easy_imgui::vec2;
use easy_imgui_window::{
    easy_imgui as imgui,
    winit::{self, dpi::{LogicalSize, PhysicalPosition, PhysicalSize}, event_loop::EventLoopProxy, window::Window},
    AppHandler, Application, Args, EventResult, MainWindowRef,
};
use event::AppEvent;
use tracing::{error, info, warn};
use widget::{
    components::modals::ModalType,
    preferences::{Preferences, PreferencesWindowState},
    theme::{self, UITheme},
    Widget,
};
use winit::{event::WindowEvent, event_loop::EventLoop};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let event_loop = EventLoop::with_user_event().build().unwrap();

    let proxy = event_loop.create_proxy();

    let mut main = AppHandler::<App>::new(proxy);
    *main.attributes() = Window::default_attributes()
        .with_title(UI_APP_NAME)
        .with_min_inner_size(LogicalSize::new(256.0, 256.0));

    event_loop.run_app(&mut main).unwrap();
}

pub type WidgetRc = Rc<RefCell<Widget>>;

struct App {
    widget: WidgetRc,
    event_loop_proxy: EventLoopProxy<AppEvent>,
    api: Arc<Mutex<SpotifyAPI>>,
}

impl Application for App {
    type UserEvent = AppEvent;
    type Data = EventLoopProxy<AppEvent>;

    fn new(args: Args<Self::Data>) -> App {
        let mut widget = Widget::new();
        let event_loop_proxy = args.data.clone();
        let api = Arc::new(Mutex::new(SpotifyAPI::new()));

        widget.init_window_state(&event_loop_proxy, Arc::clone(&api));

        App {
            widget: Rc::new(RefCell::new(widget)),
            event_loop_proxy,
            api,
        }
    }

    fn user_event(&mut self, args: Args<Self::Data>, event: Self::UserEvent) {
        let mut imgui = unsafe { args.window.renderer().imgui().set_current() };

        // Return early if this is a paint event
        if event == AppEvent::Painted {
            imgui.set_allow_user_scaling(true);
            imgui.nav_enable_keyboard();

            let app_theme = match self.widget.borrow().preferences.get().and_then(|p| p.theme) {
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

            self.widget.borrow_mut().set_theme(app_theme, false);

            return;
        }

        match event {
            AppEvent::InvalidateFontAtlas => {
                args.window.renderer().imgui().invalidate_font_atlas();
            }
            AppEvent::SetInitialWindowState => {
                info!("Setting initial window state...");

                if let Some(window_state) = self
                    .widget
                    .borrow()
                    .preferences
                    .get()
                    .and_then(|p| p.window_state)
                {
                    let mut win = args.window.main_window().window();

                    if let (Some(width), Some(height)) = (window_state.width, window_state.height) {
                        let _ = win.request_inner_size(PhysicalSize::new(width, height));
                    }

                    let is_maximized = window_state.maximized.unwrap_or(false);

                    if let (Some(x), Some(y)) = (window_state.x, window_state.y) {
                        win.set_outer_position(PhysicalPosition::new(x, y));
                    }

                    win.set_maximized(is_maximized);
                }

                self.widget.borrow_mut().ready_for_window_events = true;
            }
            AppEvent::SetTheme(theme) => self.widget.borrow_mut().set_theme(theme, true),
            AppEvent::Command(command) => {
                info!("Handling application command: {:?}", event);

                #[allow(unreachable_patterns)]
                match command {
                    AppCommand::About => self.widget.borrow_mut().open_modal(ModalType::About),

                    AppCommand::Navigate(route) => self.widget.borrow_mut().router(route),

                    AppCommand::ZoomIn => self.widget.borrow_mut().set_ui_scale(
                        &self.event_loop_proxy,
                        self.widget.borrow().ui_scale + UI_SCALE_STEP,
                    ),
                    AppCommand::ZoomOut => self.widget.borrow_mut().set_ui_scale(
                        &self.event_loop_proxy,
                        self.widget.borrow().ui_scale - UI_SCALE_STEP,
                    ),
                    AppCommand::ZoomReset => self
                        .widget
                        .borrow_mut()
                        .set_ui_scale(&self.event_loop_proxy, UI_DEFAULT_SCALE),

                    AppCommand::OpenSpotifyAccount => {
                        match self.widget.borrow().open_shell_url(SPOTIFY_ACCOUNTS_URL) {
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

    fn window_event(&mut self, args: Args<Self::Data>, event: WindowEvent, res: EventResult) {
        if self.widget.borrow().ready_for_window_events {
            match event {
                WindowEvent::Moved(new_pos) => {
                    info!("{:#?}", Preferences {
                        window_state: Some(PreferencesWindowState {
                            x: Some(new_pos.x as u32),
                            y: Some(new_pos.y as u32),

                            ..Default::default()
                        }),
                        ..Default::default()
                    });

                    self.widget.borrow_mut().preferences.set(Preferences {
                        window_state: Some(PreferencesWindowState {
                            x: Some(new_pos.x as u32),
                            y: Some(new_pos.y as u32),

                            ..Default::default()
                        }),
                        ..Default::default()
                    });
                },
                WindowEvent::Resized(new_size) => {
                    self.widget.borrow_mut().preferences.set(Preferences {
                        window_state: Some(PreferencesWindowState {
                            width: Some(new_size.width),
                            height: Some(new_size.height),

                            ..Default::default()
                        }),
                        ..Default::default()
                    });
                },
                _ => {}
            }
        }

        if res.window_closed {
            args.event_loop.exit();
        }
    }
}

impl imgui::UiBuilder for App {
    fn build_custom_atlas(&mut self, atlas: &mut imgui::FontAtlasMut<'_, Self>) {
        self.widget.borrow_mut().build_custom_atlas(atlas);
    }

    fn do_ui(&mut self, ui: &imgui::Ui<App>) {
        widget::style::push_style(&self.widget, ui, || {
            self.widget
                .borrow_mut()
                .paint_ui(&self.event_loop_proxy, ui, Arc::clone(&self.api));
        });
    }
}
