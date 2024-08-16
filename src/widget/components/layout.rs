use easy_imgui::{
    easy_imgui_sys::{
        ImGuiDir, ImGui_BeginDisabled, ImGui_DockBuilderAddNode, ImGui_DockBuilderSplitNode,
        ImGui_EndDisabled,
    },
    vec2, Color, ColorId, Cond, DockNodeFlags, ImGuiID, MouseButton, StyleValue, StyleVar,
    TableColumnFlags, TableFlags, TableRowFlags, TreeNodeFlags, Ui, WindowFlags,
};
use tracing::info;

use crate::{
    commands::AppCommand,
    constants::{
        UI_ALBUM_ART_SIZE, UI_APP_NAME, UI_ROUTE_DEFAULT, UI_ROUTE_SEARCH, UI_SIDEBAR_MIN_WIDTH,
        UI_SIDEBAR_WIDTH,
    },
    imgui_additions::{self, sidebar::ViewportSidebarDirection},
    widget::{
        icons::{self, set::UI_ICON_MUSICAL_NOTE, IconOffset},
        Widget,
    },
    App,
};

use super::ComponentContext;

macro_rules! build_sidebar_item {
    ($ui: expr, $widget: expr, $label: expr, $icon: expr, $on_click: block) => {
        let font_size = $ui.get_font_size();

        let icon_size = 16.0;
        let icon_padding_left = font_size / 6.0;

        $ui.indent(icon_padding_left);

        $widget.create_icon_button(
            $ui,
            $icon,
            icon_size,
            $ui.style().color(ColorId::Text),
            $ui.style().color_alpha(ColorId::Text, 0.075),
            $ui.style().color_alpha(ColorId::Text, 0.075),
            $ui.style().color_alpha(ColorId::Text, 0.075),
            font_size,
        );

        $ui.same_line_ex(
            icon_size + icon_padding_left * 1.1,
            icon_size + icon_padding_left * 1.1,
        );

        $ui.tree_node_config($label)
            .flags(TreeNodeFlags::Leaf | TreeNodeFlags::SpanFullWidth | TreeNodeFlags::FramePadding)
            .with(|| {
                if $ui.is_item_clicked(MouseButton::Left) {
                    $on_click
                }
            });

        $ui.unindent(icon_padding_left);
    };
}

pub fn build(context: &ComponentContext) {
    let viewport = context.ui.get_main_viewport();
    let viewport_outer_size = viewport.size();
    let viewport_size = viewport.work_size();
    let viewport_pos = viewport.work_pos();

    let font_size = context.ui.get_font_size();

    let sidebar_width = UI_SIDEBAR_WIDTH * context.widget.ui_scale;
    let album_art_height = UI_SIDEBAR_WIDTH * context.widget.ui_scale;
    let album_art_bottom_padding = 1.0;

    context.ui.set_next_window_dock_id(context.widget.viewport_dockspace + 1, Cond::Appearing);

    context.ui.with_push(
        (
            (
                StyleVar::ItemSpacing,
                StyleValue::Vec2(vec2(0.0, font_size / 4.0)),
            ),
            (StyleVar::WindowPadding, StyleValue::Vec2(vec2(0.0, 0.0))),
            (StyleVar::DockingSeparatorSize, StyleValue::F32(0.0)),
        ),
        || {
            imgui_additions::sidebar::begin_main_viewport_sidebar(
                "Sidebar VP",
                WindowFlags::NoCollapse
                    | WindowFlags::NoResize
                    | WindowFlags::NoTitleBar
                    | WindowFlags::NoNav
                    | WindowFlags::NoBringToFrontOnFocus
                    | WindowFlags::NoScrollbar,
                ViewportSidebarDirection::Left,
                sidebar_width,
            );

            context.ui.dock_space(
                10,
                vec2(
                    viewport_size.x,
                    viewport_size.y - album_art_height - album_art_bottom_padding,
                ),
                DockNodeFlags::AutoHideTabBar | DockNodeFlags::NoDockingSplit,
            );

            context.ui.set_next_window_dock_id(10, Cond::Once);
            context.ui.window_config("Sidebar")
                .flags(WindowFlags::AlwaysVerticalScrollbar)
                .with(|| {
                    context.ui.tree_node_config("Main")
                        .flags(
                            TreeNodeFlags::DefaultOpen
                                | TreeNodeFlags::SpanFullWidth
                                | TreeNodeFlags::FramePadding
                                | TreeNodeFlags::NoAutoOpenOnLog,
                        )
                        .with(|| {
                            build_sidebar_item!(context.ui, context.widget, "Home", icons::set::UI_ICON_HOME, {
                                context.widget.send_command(
                                    context.event_loop,
                                    AppCommand::Navigate(UI_ROUTE_DEFAULT),
                                );
                            });
                            build_sidebar_item!(
                                context.ui,
                                context.widget,
                                "Search",
                                icons::set::UI_ICON_SEARCH,
                                {
                                    context.widget.send_command(
                                        context.event_loop,
                                        AppCommand::Navigate(UI_ROUTE_SEARCH),
                                    );
                                }
                            );
                            build_sidebar_item!(
                                context.ui,
                                context.widget,
                                "Browse",
                                icons::set::UI_ICON_BROWSE,
                                {}
                            );
                            build_sidebar_item!(
                                context.ui,
                                context.widget,
                                "Charts",
                                icons::set::UI_ICON_CHARTS,
                                {}
                            );
                            build_sidebar_item!(
                                context.ui,
                                context.widget,
                                "Releases",
                                icons::set::UI_ICON_CALENDAR,
                                {}
                            );
                        });

                    context.ui.tree_node_config("Your Library")
                        .flags(
                            TreeNodeFlags::DefaultOpen
                                | TreeNodeFlags::SpanFullWidth
                                | TreeNodeFlags::FramePadding,
                        )
                        .with(|| {
                            build_sidebar_item!(
                                context.ui,
                                context.widget,
                                "Liked Songs",
                                icons::set::UI_ICON_HEART,
                                {}
                            );
                            build_sidebar_item!(
                                context.ui,
                                context.widget,
                                "Recently Played",
                                icons::set::UI_ICON_CLOCK,
                                {}
                            );
                            build_sidebar_item!(
                                context.ui,
                                context.widget,
                                "Your Artists",
                                icons::set::UI_ICON_USER,
                                {}
                            );
                            build_sidebar_item!(
                                context.ui,
                                context.widget,
                                "Your Albums",
                                icons::set::UI_ICON_DISC,
                                {}
                            );
                            build_sidebar_item!(
                                context.ui,
                                context.widget,
                                "Your Files",
                                icons::set::UI_ICON_DOWNLOAD,
                                {}
                            );
                        });

                    context.ui.tree_node_config("Playlists")
                        .flags(
                            TreeNodeFlags::DefaultOpen
                                | TreeNodeFlags::SpanFullWidth
                                | TreeNodeFlags::FramePadding,
                        )
                        .with(|| {
                            for _ in 0..50 {
                                build_sidebar_item!(
                                    context.ui,
                                    context.widget,
                                    "Rediscover - Aug 9th",
                                    icons::set::UI_ICON_MUSICAL_NOTE,
                                    {}
                                );
                            }
                        });
                });

            context.ui.with_push((StyleVar::WindowBorderSize, StyleValue::F32(0.0)), || {
                context.ui.dock_space(
                    11,
                    vec2(
                        album_art_height,
                        album_art_height - album_art_bottom_padding,
                    ),
                    DockNodeFlags::AutoHideTabBar | DockNodeFlags::NoDockingSplit,
                );

                context.ui.set_next_window_dock_id(11, Cond::Once);
                context.ui.set_next_window_size_constraints_callback(
                    vec2(
                        album_art_height,
                        album_art_height + album_art_bottom_padding,
                    ),
                    vec2(10000.0, 10000.0),
                    |mut data| {
                        let min = data.desired_size().x.min(data.desired_size().y);

                        data.set_desired_size(vec2(min, min));
                    },
                );

                context.ui.window_config("Album Art")
                    .flags(
                        WindowFlags::NoCollapse
                            | WindowFlags::NoScrollbar
                            | WindowFlags::NoScrollWithMouse
                            | WindowFlags::NoTitleBar,
                    )
                    .with(|| {
                        let display_size = context.ui.get_content_region_avail();
                        let album_art_scale =
                            display_size.x.min(display_size.y) / UI_ALBUM_ART_SIZE;

                        context.ui.image_with_custom_rect_config(context.widget.glyph_album_art, album_art_scale)
                            .build();
                    });
            });

            imgui_additions::sidebar::end_main_viewport_sidebar();
        },
    );
}
