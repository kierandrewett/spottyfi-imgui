use crate::{
    constants::UI_ROUTE_SEARCH,
    create_pane, dummy,
    widget::{
        components::{
            card::{self, CardDetails},
            ComponentContext,
        },
        icons::set::UI_ICON_SEARCH,
    },
};
use chrono::{offset::Local, Timelike};
use easy_imgui::{
    Color, ColorId, FocusedFlags, ImGuiID, InputTextFlags, MouseButton, TableColumnFlags,
    TableFlags,
};
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
                        let field_fill_width =
                            context.ui.get_window_width() < (field_max_width + 80.0);

                        context
                            .ui
                            .table_config("Search Field", if field_fill_width { 1 } else { 2 })
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

                                context
                                    .ui
                                    .input_text_hint_config(
                                        "##SearchField",
                                        "Search artists, songs or albums",
                                        &mut context.widget.state.panes.search.search_value,
                                    )
                                    .flags(InputTextFlags::EscapeClearsAll)
                                    .build();
                                if context.ui.is_window_appearing() {
                                    context.ui.set_keyboard_focus_here(-1);
                                }

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

                dummy!(context);

                if context
                    .widget
                    .state
                    .panes
                    .search
                    .search_value
                    .trim()
                    .is_empty()
                {
                    context.ui.with_push(font_h3, || {
                        context.ui.text("Recent searches");
                        dummy!(context);

                        context
                            .ui
                            .table_config("Card Grid", 8)
                            .flags(TableFlags::Borders)
                            .with(|| {
                                for i in 0..8 {
                                    context.ui.table_setup_column(
                                        format!("Card {i}"),
                                        TableColumnFlags::WidthStretch,
                                        -1.0,
                                        ImGuiID::default(),
                                    );
                                }

                                for _ in 0..8 {
                                    context.ui.table_next_column();
                                    card::build(
                                        context,
                                        CardDetails {
                                            title: "All Out 80s",
                                            subtitle: "By Spotify",
                                            image: context.widget.glyph_album_art,
                                        },
                                    );
                                }
                            });
                    });
                } else {
                    context.ui.with_push(font_h3, || {
                        context.ui.text("Songs");
                        dummy!(context);
                    });

                    context.ui.with_push(font_h3, || {
                        context.ui.text("Artists");
                        dummy!(context);
                    });

                    context.ui.with_push(font_h3, || {
                        context.ui.text("Albums");
                        dummy!(context);
                    });

                    context.ui.with_push(font_h3, || {
                        context.ui.text("Playlists");
                        dummy!(context);
                    });
                }
            },
        )
    });
}
