pub mod layout;
pub mod menubar;
pub mod modals;
pub mod panes;
pub mod player;

use easy_imgui::Ui;
use easy_imgui_window::winit::event_loop::EventLoopProxy;

use crate::{event::AppEvent, App};

use super::Widget;

pub struct ComponentContext<'a> {
    pub widget: &'a mut Widget,
    pub event_loop: &'a EventLoopProxy<AppEvent>,
    pub ui: &'a Ui<App>,
}
