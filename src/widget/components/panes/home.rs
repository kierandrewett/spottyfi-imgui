use crate::{constants::UI_ROUTE_DEFAULT, create_pane, widget::components::ComponentContext};
use chrono::{offset::Local, Timelike};

pub fn build(context: &mut ComponentContext) {
    let mut open = context.widget.state.panes.home_visible;

    let time = Local::now();

    create_pane!(context.ui, context.widget, UI_ROUTE_DEFAULT, open, {
        context.ui.with_push(context.widget.font_h2, || {
            context.ui.text(match time.hour() {
                5..12 => "Good morning",
                12..16 => "Good afternoon",
                _ => "Good evening",
            })
        });
    });
}
