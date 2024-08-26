#![allow(unused)]

mod api;
mod commands;
mod constants;
mod event;
mod imgui_additions;
mod utils;
mod state;
mod widget;

use std::{
    cell::{Ref, RefCell}, rc::Rc, sync::{Arc, Mutex, RwLock}, thread, time::Duration
};

use api::{models::recommendations::{self, BrowseRecommendations}, SpotifyAPI};
use commands::AppCommand;
use constants::{
    UI_APP_NAME, UI_DARK_WINDOW_BG_COLOR, UI_DEFAULT_LOCALE, UI_DEFAULT_SCALE, UI_LIGHT_WINDOW_BG_COLOR, UI_SCALE_STEP
};
use easy_imgui_window::{
    easy_imgui as imgui,
    winit::{self, dpi::{LogicalSize, PhysicalPosition, PhysicalSize}, event_loop::{self, EventLoopProxy}, window::Window},
    AppHandler, Application, Args, EventResult,
};
use event::{AppEvent, AppFetchType};
use semaphore::Semaphore;
use state::{search::WidgetStateSearchResults, State};
use tracing::{error, info};
use widget::{
    components::modals::ModalType,
    preferences::{Preferences, PreferencesCredentials, PreferencesWindowState},
    theme::{self, UITheme},
    Widget,
};
use winit::{event::WindowEvent, event_loop::EventLoop};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let event_loop = EventLoop::with_user_event().build().unwrap();

    let proxy = event_loop.create_proxy();

    let mut main = AppHandler::<App>::new(proxy.clone());
    *main.attributes() = Window::default_attributes()
        .with_title(UI_APP_NAME)
        .with_min_inner_size(LogicalSize::new(256.0, 256.0));

    tokio::task::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        loop {
            proxy.clone().send_event(AppEvent::Fetch(AppFetchType::Volatile)).ok();
            interval.tick().await;
        }
    });

    event_loop.run_app(&mut main).unwrap();
}

pub type WidgetRc = Rc<RefCell<Widget>>;

pub struct App {
    widget: WidgetRc,
    event_loop_proxy: Arc<EventLoopProxy<AppEvent>>,
    api: Arc<SpotifyAPI>,
}

impl Application for App {
    type UserEvent = AppEvent;
    type Data = EventLoopProxy<AppEvent>;

    fn new(args: Args<Self::Data>) -> App {
        let mut widget = Rc::new(RefCell::new(Widget::new()));
        let event_loop_proxy = Arc::new(args.data.clone());

        let refresh_token = widget
            .borrow()
            .preferences
            .get()
            .and_then(|p| p.credentials)
            .and_then(|c| c.secret)
            .and_then(|c| if c.trim().is_empty() { None } else { Some(c) });

        let api = Arc::new(SpotifyAPI::new(
            Arc::clone(&event_loop_proxy),
            refresh_token.clone()
        ));

        if let Some(token) = refresh_token.clone() {
            info!("Attempting to login to Spotify with refresh token...");
        } else {
            // If we don't have a refresh token we can assume we don't have a login session at all
            widget.borrow_mut().state.lock().unwrap().preferences.visible = true;
        }

        widget.borrow_mut()
            .init_window_state(&event_loop_proxy, Arc::clone(&api));

        App {
            widget,
            event_loop_proxy,
            api
        }
    }

    fn user_event(&mut self, args: Args<Self::Data>, event: Self::UserEvent) {
        let mut imgui = unsafe { args.window.renderer().imgui().set_current() };

        // Return early if this is a paint event
        if event == AppEvent::Painted {
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

        match event.clone() {
            AppEvent::InvalidateFontAtlas => {
                args.window.renderer().imgui().invalidate_font_atlas();
            },
            AppEvent::Login => {
                let api_arc = Arc::clone(&self.api);

                tokio::task::spawn(async move {
                    if api_arc.is_not_authenticated() {
                        api_arc.login(None).await;
                    }
                });
            },
            AppEvent::Fetch(r#type) => {
                let can_fetch = self.api.is_authenticated() || self.api.is_logged_in();

                if !can_fetch {
                    return;
                }

                let all = matches!(r#type, AppFetchType::All);
                let volatile = matches!(r#type, AppFetchType::Volatile);

                if all || volatile || matches!(r#type, AppFetchType::Profile) {
                    let api_arc = Arc::clone(&self.api);

                    let locale = self.widget.borrow().locale();

                    tokio::task::spawn(async move {
                        api_arc.fetch_data_wrapper(locale).await;
                    });
                }

                if all || matches!(r#type, AppFetchType::Recommendations) {
                    let api_arc = Arc::clone(&self.api);
                    let state_arc = Arc::clone(&self.widget.borrow_mut().state);

                    let locale = self.widget.borrow().locale();

                    tokio::task::spawn(async move {
                        match api_arc.get_browse_recommendations(locale).await {
                            Ok(recommendations) => {
                                if let Ok(mut state) = state_arc.lock() {
                                    state.recommendations = Some(recommendations);
                                }
                            },
                            Err(err) => {
                                error!("Failed to download recommendations data: {:#?}", err);
                            }
                        }
                    });
                }
            },
            AppEvent::FirstTimeLogin => {
                if self.api.is_logged_in() {
                    self.widget.borrow_mut().state.lock().unwrap().home_visible = true;
                } else {
                    self.widget.borrow_mut().state.lock().unwrap().preferences.visible = true;
                }

                args.window
                    .main_window()
                    .window()
                    .focus_window();
            },
            AppEvent::StoreToken(refresh_token) => {
                let credentials = PreferencesCredentials {
                    secret: Some(refresh_token.unwrap_or("".to_string()))
                };

                self
                    .widget
                    .borrow_mut()
                    .preferences
                    .set(Preferences {
                        credentials: Some(credentials),
                        ..Default::default()
                    });
            },
            AppEvent::SetInitialWindowState => {
                info!("Setting initial window state...");

                if let Some(window_state) = self
                    .widget
                    .borrow()
                    .preferences
                    .get()
                    .and_then(|p| p.window_state)
            {
                    let win = args.window.main_window().window();

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
                info!("Handling application command: {:?}", event.clone());

                #[allow(unreachable_patterns)]
                match command {
                    AppCommand::About => self.widget.borrow_mut().open_modal(ModalType::About),

                    AppCommand::Navigate(route) => self.widget.borrow_mut().router(route),

                    AppCommand::ZoomIn => {
                        let mut widget_mut = self.widget.borrow_mut();

                        let new_scale = widget_mut.ui_scale + UI_SCALE_STEP;

                        widget_mut.set_ui_scale(
                            &self.event_loop_proxy,
                            new_scale,
                        );
                    },
                    AppCommand::ZoomOut => {
                        let mut widget_mut = self.widget.borrow_mut();

                        let new_scale = widget_mut.ui_scale - UI_SCALE_STEP;

                        widget_mut.set_ui_scale(
                            &self.event_loop_proxy,
                            new_scale,
                        );
                    },
                    AppCommand::ZoomReset => self
                        .widget
                        .borrow_mut()
                        .set_ui_scale(&self.event_loop_proxy, UI_DEFAULT_SCALE),

                    AppCommand::OpenSpotifyAccount => {
                        match self.api.open_accounts_page() {
                            Ok(_) => {}
                            Err(err) => error!("Failed to open Spotify accounts URL: {:?}", err),
                        }
                    },

                    AppCommand::DoSearch(value) => {
                        let mut state_arc = Arc::clone(&self.widget.borrow().state);

                        if let Some(handle) = &state_arc.lock().unwrap().search.search_task {
                            handle.abort();
                        }

                        if !value.is_empty() {
                            let mut api_arc = Arc::clone(&self.api);

                            state_arc.lock().unwrap().search.search_results = WidgetStateSearchResults::Fetching;

                            let task = tokio::task::spawn(async move {
                                state_arc.lock().unwrap().search.search_results = WidgetStateSearchResults::Fetched(api_arc.search(
                                    value,
                                    None,
                                    Some(10)
                                )
                                    .await
                                );
                            });

                            let mut state_arc = Arc::clone(&self.widget.borrow().state);

                            state_arc.lock().unwrap().search.search_task = Some(task.abort_handle());
                        } else {
                            state_arc.lock().unwrap().search.search_results = WidgetStateSearchResults::None;
                        }
                    },

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
                    self.widget.borrow_mut().preferences.set(Preferences {
                        window_state: Some(PreferencesWindowState {
                            x: Some(new_pos.x as u32),
                            y: Some(new_pos.y as u32),

                            maximized: Some(args.window.main_window().window().is_maximized()),

                            ..Default::default()
                        }),
                        ..Default::default()
                    });
                },
                WindowEvent::Resized(new_size) => {
                    let is_maximized = args.window.main_window().window().is_maximized();

                    let window_state = if is_maximized {
                        PreferencesWindowState {
                            maximized: Some(is_maximized),

                            ..Default::default()
                        }
                    } else {
                        PreferencesWindowState {
                            width: Some(new_size.width),
                            height: Some(new_size.height),

                            maximized: Some(is_maximized),

                            ..Default::default()
                        }
                    };

                    self.widget.borrow_mut().preferences.set(Preferences {
                        window_state: Some(window_state),
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
