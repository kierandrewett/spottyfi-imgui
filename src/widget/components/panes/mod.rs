
use tokio::runtime::Handle;

use super::ComponentContext;

pub mod home;
pub mod preferences;
pub mod search;

#[macro_export]
macro_rules! create_pane {
    ($ui: expr, $widget: expr, $title: expr, $open: expr, $render: block) => {
        use easy_imgui::{vec2, Cond, StyleValue, StyleVar, WindowFlags};

        let font_size = $ui.get_font_size();

        $ui.with_push(
            ((
                StyleVar::WindowPadding,
                StyleValue::Vec2(vec2(font_size * 2.0, font_size * 2.0)),
            )),
            || {
                $ui.set_next_window_dock_id(1, Cond::Appearing);

                $ui.window_config($title)
                    .flags(WindowFlags::None)
                    .open(&mut $open)
                    .with(|| $render);
            },
        );
    };
}

pub fn build(context: &mut ComponentContext) {
    let is_prefs_visible = context.widget.state.lock().unwrap().preferences.visible;
    let is_home_visible = context.widget.state.lock().unwrap().home_visible;
    let is_search_visible = context.widget.state.lock().unwrap().search.visible;

    if is_prefs_visible {
        preferences::build(context);
    }

    if is_home_visible {
        home::build(context);
    }

    if is_search_visible {
        search::build(context);
    }
}
