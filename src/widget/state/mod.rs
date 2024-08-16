use developer::WidgetStateDeveloper;
use panes::WidgetStatePanes;

use super::theme::UITheme;

pub mod developer;
pub mod panes;

#[derive(Debug, Clone, Default)]
pub struct WidgetState {
    pub current_theme: UITheme,

    pub panes: WidgetStatePanes,

    #[cfg(debug_assertions)]
    pub developer: WidgetStateDeveloper,
}
