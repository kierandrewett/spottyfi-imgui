pub mod card;
pub mod menubar;
pub mod modals;
pub mod panes;
pub mod player;
pub mod sidebar;
pub mod error;
pub mod async_image;

use std::sync::{Arc, Mutex};

use easy_imgui::Ui;
use easy_imgui_window::winit::event_loop::EventLoopProxy;

use crate::{api::SpotifyAPI, event::AppEvent, state::State, App};

use super::Widget;

#[macro_export]
macro_rules! dummy {
    ($ctx: expr) => {
        {
            use easy_imgui::vec2;

            $ctx.ui.dummy(vec2(0.0, $ctx.ui.style().FramePadding.y));
        }
    };
    ($ctx: expr, $mult: expr) => {
        {
            use easy_imgui::vec2;

            $ctx.ui.dummy(vec2(0.0, $ctx.ui.style().FramePadding.y * $mult));
        }
    };
}

pub struct ComponentContext<'a> {
    pub widget: &'a mut Widget,
    pub event_loop: &'a EventLoopProxy<AppEvent>,
    pub ui: &'a Ui<App>,
    pub api: Arc<SpotifyAPI>
}
