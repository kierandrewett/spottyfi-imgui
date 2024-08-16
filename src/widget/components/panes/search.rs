use crate::{
    constants::UI_ROUTE_SEARCH,
    create_pane,
    widget::{components::ComponentContext, icons::set::UI_ICON_SEARCH},
};
use chrono::{offset::Local, Timelike};
use easy_imgui::{Color, ColorId, ImGuiID, InputTextFlags, TableColumnFlags, TableFlags};
use tracing::info;

pub fn build(context: &mut ComponentContext) {
    let mut open = context.widget.state.panes.search.visible;

    let input_icon_size = 16.0 * context.widget.ui_scale;
    let input_padding = 12.0 * context.widget.ui_scale;
    let input_padding_start = input_padding + (input_icon_size * 2.0);

    let font_h3 = context.widget.font_h3.clone();

    create_pane!(context.ui, context.widget, UI_ROUTE_SEARCH, open, {
        context.ui.with_push(
            (
                (
                    StyleVar::FramePadding,
                    StyleValue::Vec2(vec2(input_padding_start, input_padding)),
                ),
                (StyleVar::FrameRounding, StyleValue::F32(100.0)),
            ),
            || {
                context.ui.with_push(
                    (
                        (ColorId::FrameBg, context.ui.style().color(ColorId::Text)),
                        (ColorId::Text, context.ui.style().color(ColorId::WindowBg)),
                    ),
                    || {
                        let field_max_width = 700.0;
                        let field_fill_width = context.ui.get_window_width() < (field_max_width + 80.0);

                        if context.ui.button("set search value") {
                            info!("done");
                            context.widget.state.panes.search.search_value = "hello".to_string();
                            info!("done {}", context.widget.state.panes.search.search_value);
                        }

                        context.ui.table_config("Search Field", if field_fill_width { 1 } else { 2 })
                            .flags(TableFlags::None)
                            .with(|| {
                                context.ui.table_setup_column(
                                    "Field",
                                    if field_fill_width {
                                        TableColumnFlags::WidthStretch
                                    } else {
                                        TableColumnFlags::WidthFixed
                                    },
                                    if field_fill_width {
                                        -1.0
                                    } else {
                                        field_max_width
                                    },
                                    ImGuiID::default(),
                                );
                                if !field_fill_width {
                                    context.ui.table_setup_column(
                                        "Spacer",
                                        TableColumnFlags::WidthStretch,
                                        -1.0,
                                        ImGuiID::default(),
                                    );
                                }

                                context.ui.table_next_column();

                                let input_start_x = context.ui.get_cursor_pos_x();
                                context.ui.input_text_hint_config(
                                    "##SearchField",
                                    "Search artists, songs or albums",
                                    &mut context.widget.state.panes.search.search_value,
                                )
                                .build();
                                let input_height = context.ui.get_item_rect_size().y;
                                context.ui.same_line();
                                context.ui.set_cursor_pos_x(input_start_x + input_icon_size);
                                context.ui.set_cursor_pos_y(
                                    context.ui.get_cursor_pos_y() + (input_height / 2.0)
                                        - (input_icon_size / 2.0),
                                );
                                context.widget.create_icon(
                                    context.ui,
                                    UI_ICON_SEARCH,
                                    16.0,
                                    context.ui.style().color(ColorId::Text),
                                );

                                if !field_fill_width {
                                    context.ui.table_next_column();
                                }
                            });
                    },
                );

                context.ui.dummy(vec2(0.0, context.ui.style().FramePadding.y));

                context.ui.with_push(font_h3, || {
                    context.ui.text("Songs");
                });
                
                context.ui.with_push(font_h3, || {
                    context.ui.text("Artists");
                });
                
                context.ui.with_push(font_h3, || {
                    context.ui.text("Albums");
                });
                
                context.ui.with_push(font_h3, || {
                    context.ui.text("Playlists");
                });
            },
        )
    });
}
