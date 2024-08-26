use std::fmt::{Display, Debug};

use easy_imgui::ColorId;
use tracing::info;

use crate::{dummy, widget::icons::set::UI_ICON_ERROR};

use super::ComponentContext;

pub fn build<E: Debug>(context: &mut ComponentContext, error: E) {
    let icon_size = context.ui.get_font_size() * 2.0;

    dummy!(context, 10.0);

    context.ui.set_cursor_pos_x((context.ui.get_window_width() / 2.0) - (icon_size / 2.0));
    context.widget.create_icon(
        context.ui,
        UI_ICON_ERROR,
        icon_size,
        context.ui.style().color(ColorId::Text)
    );

    context.ui.with_push(context.widget.font_h2, || {
        context.ui.set_cursor_pos_x((context.ui.get_window_width() / 2.0) - (context.ui.calc_text_size("Something went wrong.").x / 2.0));
        context.ui.text("Something went wrong.");
    });

    dummy!(context);

    let error_text = &format!("{:#?}", error);

    context.ui.set_cursor_pos_x((context.ui.get_window_width() / 2.0) - (context.ui.calc_text_size(error_text).x / 2.0));
    context.ui.text(error_text);
}