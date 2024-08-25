use developer::WidgetStateDeveloper;
use panes::WidgetStatePanes;

use super::theme::UITheme;

pub mod developer;
pub mod panes;
pub mod preferences;

#[derive(Debug, Default)]
pub struct State {
    pub current_theme: UITheme,

    pub panes: WidgetStatePanes,

    #[cfg(debug_assertions)]
    pub developer: WidgetStateDeveloper,
}
