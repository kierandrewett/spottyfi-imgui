use std::ffi::{c_int, CString};

use easy_imgui::{
    easy_imgui_sys::{
        ImGuiCol_, ImGuiDir, ImGui_BeginViewportSideBar, ImGui_End, ImGui_GetMainViewport,
    },
    vec2, Color, ColorId, Cond, ImGuiID, StyleValue, StyleVar, TableColumnFlags, TableFlags,
    TableRowFlags, WindowFlags,
};
use serde::{Deserialize, Serialize};
use stretch::{
    geometry::Size,
    style::{Dimension, FlexDirection, Style},
    Stretch,
};
use tracing::info;

use crate::{
    constants::UI_PLAYER_BAR_HEIGHT,
    imgui_additions::{self, sidebar::ViewportSidebarDirection},
    widget::icons::set::{
        UI_ICON_HEART, UI_ICON_MEDIA_NEXT, UI_ICON_MEDIA_PAUSE, UI_ICON_MEDIA_PREVIOUS,
        UI_ICON_REPEAT, UI_ICON_SHUFFLE,
    },
};

use super::ComponentContext;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum PlayerArea {
    Outside,
    Inside,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum PlayerPosition {
    Top,
    Bottom,
}

fn build_track_info(context: &mut ComponentContext) {
    let centre_y = ((UI_PLAYER_BAR_HEIGHT / 2.0) - 20.0) * context.widget.ui_scale;

    context.ui.table_next_column();
    context.ui.set_cursor_pos_y(centre_y);

    context.ui.with_push(
        (StyleVar::ItemSpacing, StyleValue::Vec2(vec2(0.0, 2.0))),
        || {
            context.ui.with_push(
                (
                    context.widget.font_bold,
                    (ColorId::Text, context.ui.style().color(ColorId::Text)),
                ),
                || {
                    context.ui.text("Making Plans For Nigel");
                },
            );

            context.ui.with_push(
                (
                    context.widget.font_small,
                    (
                        ColorId::Text,
                        context.ui.style().color_alpha(ColorId::Text, 0.7),
                    ),
                ),
                || {
                    context.ui.text("XTC");
                },
            );
        },
    );

    context.ui.table_next_column();

    context.ui.indent(24.0 * context.widget.ui_scale);
    context.ui.set_cursor_pos_y(
        (UI_PLAYER_BAR_HEIGHT * context.widget.ui_scale) / 2.0 - context.ui.get_item_rect_size().y,
    );
    context.widget.create_icon_button(
        context.ui,
        UI_ICON_HEART,
        16.0,
        context.ui.style().color(ColorId::Text),
        Color::TRANSPARENT,
        Color::TRANSPARENT,
        Color::TRANSPARENT,
        0.0,
    );
}

fn build_media_controls(context: &mut ComponentContext) {
    context.ui.table_next_column();

    context
        .ui
        .set_cursor_pos_y((context.ui.get_cursor_pos_y() + 16.0) * context.widget.ui_scale);

    context.widget.create_icon_button(
        context.ui,
        UI_ICON_SHUFFLE,
        16.0,
        context.ui.style().color_alpha(ColorId::Text, 0.75),
        Color::TRANSPARENT,
        Color::TRANSPARENT,
        Color::TRANSPARENT,
        0.0,
    );
    context.ui.same_line();

    context.widget.create_icon_button(
        context.ui,
        UI_ICON_MEDIA_PREVIOUS,
        20.0,
        context.ui.style().color_alpha(ColorId::Text, 0.75),
        Color::TRANSPARENT,
        context.ui.style().color_alpha(ColorId::Text, 0.1),
        context.ui.style().color_alpha(ColorId::Text, 0.15),
        100.0,
    );

    context.ui.same_line();

    context.widget.create_icon_button(
        context.ui,
        UI_ICON_MEDIA_PAUSE,
        20.0,
        context.ui.style().color_alpha(ColorId::Text, 0.75),
        Color::TRANSPARENT,
        context.ui.style().color_alpha(ColorId::Text, 0.1),
        context.ui.style().color_alpha(ColorId::Text, 0.15),
        100.0,
    );
    context.ui.same_line();

    context.widget.create_icon_button(
        context.ui,
        UI_ICON_MEDIA_NEXT,
        20.0,
        context.ui.style().color_alpha(ColorId::Text, 0.75),
        Color::TRANSPARENT,
        context.ui.style().color_alpha(ColorId::Text, 0.1),
        context.ui.style().color_alpha(ColorId::Text, 0.15),
        100.0,
    );
    context.ui.same_line();

    context
        .ui
        .set_cursor_pos_y((context.ui.get_cursor_pos_y() + 2.0) * context.widget.ui_scale);
    context.widget.create_icon_button(
        context.ui,
        UI_ICON_REPEAT,
        16.0,
        context.ui.style().color_alpha(ColorId::Text, 0.75),
        Color::TRANSPARENT,
        Color::TRANSPARENT,
        Color::TRANSPARENT,
        0.0,
    );
    context.ui.same_line();
}

fn build_playback_options(context: &ComponentContext) {
    context.ui.table_next_column();

    context.ui.progress_bar_config(0.75).build();
}

pub fn build(context: &mut ComponentContext) {
    let font_size = context.ui.get_font_size();

    context.ui.with_push(
        (
            (
                StyleVar::WindowPadding,
                StyleValue::Vec2(vec2(
                    16.0 * context.widget.ui_scale,
                    0.0 * context.widget.ui_scale,
                )),
            ),
            (
                StyleVar::ItemSpacing,
                StyleValue::Vec2(vec2(
                    16.0 * context.widget.ui_scale,
                    4.0 * context.widget.ui_scale,
                )),
            ),
        ),
        || {
            let player_position = context
                .widget
                .preferences
                .get()
                .and_then(|p| p.player_bar)
                .and_then(|p| p.position)
                .unwrap();

            imgui_additions::sidebar::begin_main_viewport_sidebar(
                "Player",
                WindowFlags::NoResize
                    | WindowFlags::NoTitleBar
                    | WindowFlags::NoDocking
                    | WindowFlags::NoMove
                    | WindowFlags::NoBringToFrontOnFocus,
                match player_position {
                    PlayerPosition::Top => ViewportSidebarDirection::Up,
                    PlayerPosition::Bottom => ViewportSidebarDirection::Down,
                },
                UI_PLAYER_BAR_HEIGHT * context.widget.ui_scale,
            );

            context.ui.window_draw_list().add_line(
                context.ui.get_cursor_pos(),
                context.ui.display_size(),
                Color::RED,
                2.0,
            );

            let viewport_size = context.ui.get_main_viewport().size();

            context
                .ui
                .table_config("player_layout", 6)
                .flags(TableFlags::None)
                .with(|| {
                    context.ui.table_setup_column(
                        "track_info",
                        TableColumnFlags::WidthFixed,
                        0.0,
                        ImGuiID::default(),
                    );
                    context.ui.table_setup_column(
                        "heart",
                        TableColumnFlags::WidthFixed,
                        0.0,
                        ImGuiID::default(),
                    );
                    context.ui.table_setup_column(
                        "spacer_1",
                        TableColumnFlags::WidthStretch,
                        1.0,
                        ImGuiID::default(),
                    );
                    context.ui.table_setup_column(
                        "media_controls",
                        TableColumnFlags::WidthFixed,
                        0.0,
                        ImGuiID::default(),
                    );
                    context.ui.table_setup_column(
                        "spacer_2",
                        TableColumnFlags::WidthStretch,
                        1.0,
                        ImGuiID::default(),
                    );
                    context.ui.table_setup_column(
                        "playback_options",
                        TableColumnFlags::WidthFixed,
                        0.0,
                        ImGuiID::default(),
                    );

                    // Track Info
                    build_track_info(context);
                    context.ui.table_next_column();

                    // Media Controls
                    build_media_controls(context);
                    context.ui.table_next_column();

                    // Playback Options
                    build_playback_options(context);

                    let column_height = context.ui.get_item_rect_size().y;
                });

            imgui_additions::sidebar::end_main_viewport_sidebar();
        },
    );
}
