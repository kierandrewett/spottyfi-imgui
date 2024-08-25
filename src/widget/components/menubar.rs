
use easy_imgui::{
    vec2, ColorId, ImGuiID, TableColumnFlags,
};
use tokio::runtime::Handle;

use crate::{
    commands::AppCommand,
    constants::UI_ROUTE_PREFERENCES,
    widget::icons::set::UI_ICON_USER,
};

use super::ComponentContext;

pub fn build(context: &mut ComponentContext) {
    context.ui.with_push(
        (
            ColorId::MenuBarBg,
            context.ui.style().color(ColorId::WindowBg),
        ),
        || {
            context.ui.with_main_menu_bar(|| {
                context.ui.table_config("menubar", 2).with(|| {
                    context.ui.table_setup_column(
                        "Main",
                        TableColumnFlags::WidthStretch,
                        1.0,
                        ImGuiID::default(),
                    );
                    context.ui.table_setup_column(
                        "Extra",
                        TableColumnFlags::WidthFixed,
                        0.0,
                        ImGuiID::default(),
                    );

                    context.ui.table_next_column();

                    context.ui.menu_config("File").with(|| {
                        context
                            .ui
                            .menu_item_config("New Playlist")
                            .shortcut("Ctrl+N")
                            .build();
                        context.ui.separator();
                        if context
                            .ui
                            .menu_item_config("Exit")
                            .shortcut("Ctrl+Shift+Q")
                            .build()
                        {
                            context
                                .widget
                                .send_command(context.event_loop, AppCommand::Quit);
                        }
                    });
                    context.ui.menu_config("Edit").with(|| {
                        context
                            .ui
                            .menu_item_config("Undo")
                            .shortcut("Ctrl+Z")
                            .build();
                        context
                            .ui
                            .menu_item_config("Redo")
                            .shortcut("Ctrl+Shift+Z")
                            .build();
                        context.ui.separator();
                        context
                            .ui
                            .menu_item_config("Cut")
                            .shortcut("Ctrl+X")
                            .build();
                        context
                            .ui
                            .menu_item_config("Copy")
                            .shortcut("Ctrl+C")
                            .build();
                        context
                            .ui
                            .menu_item_config("Paste")
                            .shortcut("Ctrl+V")
                            .build();
                        context
                            .ui
                            .menu_item_config("Delete")
                            .shortcut("Delete")
                            .build();
                        context.ui.separator();
                        context
                            .ui
                            .menu_item_config("Select All")
                            .shortcut("Ctrl+A")
                            .build();
                        context.ui.separator();
                        context
                            .ui
                            .menu_item_config("Filter")
                            .shortcut("Ctrl+F")
                            .build();
                        context.ui.separator();
                        if context
                            .ui
                            .menu_item_config("Preferences")
                            .shortcut("Ctrl+P")
                            .build()
                        {
                            context.widget.send_command(
                                context.event_loop,
                                AppCommand::Navigate(UI_ROUTE_PREFERENCES),
                            );
                        }
                    });
                    context.ui.menu_config("View").with(|| {
                        if context
                            .ui
                            .menu_item_config("Zoom In")
                            .shortcut("Ctrl+=")
                            .build()
                        {
                            context
                                .widget
                                .send_command(context.event_loop, AppCommand::ZoomIn);
                        }
                        if context
                            .ui
                            .menu_item_config("Zoom Out")
                            .shortcut("Ctrl+-")
                            .build()
                        {
                            context
                                .widget
                                .send_command(context.event_loop, AppCommand::ZoomOut);
                        }
                        if context
                            .ui
                            .menu_item_config("Reset Zoom")
                            .shortcut("Ctrl+0")
                            .build()
                        {
                            context
                                .widget
                                .send_command(context.event_loop, AppCommand::ZoomReset);
                        }
                    });
                    context.ui.menu_config("Playback").with(|| {
                        context
                            .ui
                            .menu_item_config("Play / Pause")
                            .shortcut("Space")
                            .build();
                        context.ui.separator();
                        context
                            .ui
                            .menu_item_config("Next Track")
                            .shortcut("Ctrl+Right Arrow")
                            .build();
                        context
                            .ui
                            .menu_item_config("Previous Track")
                            .shortcut("Ctrl+Left Arrow")
                            .build();
                        context
                            .ui
                            .menu_item_config("Seek Forward")
                            .shortcut("Shift+Right Arrow")
                            .build();
                        context
                            .ui
                            .menu_item_config("Seek Backward")
                            .shortcut("Shift+Left Arrow")
                            .build();
                        context.ui.separator();
                        context
                            .ui
                            .menu_item_config("Shuffle")
                            .shortcut("Ctrl+S")
                            .build();
                        context
                            .ui
                            .menu_item_config("Repeat")
                            .shortcut("Ctrl+R")
                            .build();
                        context.ui.separator();
                        context
                            .ui
                            .menu_item_config("Volume Up")
                            .shortcut("Ctrl+Up Arrow")
                            .build();
                        context
                            .ui
                            .menu_item_config("Volume Down")
                            .shortcut("Ctrl+Down Arrow")
                            .build();
                    });
                    context.ui.menu_config("Help").with(|| {
                        context.ui.menu_item_config("Third-party licences").build();
                        context.ui.separator();
                        if context.ui.menu_item_config("About").build() {
                            context
                                .widget
                                .send_command(context.event_loop, AppCommand::About);
                        }
                    });

                    let is_authorised = context.api
                        .lock()
                        .unwrap()
                        .is_authorised();

                    context.ui.table_next_column();

                    let menuitem_start_x = context.ui.get_cursor_pos_x();
                    let menuitem_size = context.ui.get_item_rect_size();

                    if is_authorised {
                        context.ui.menu_config("      Kieran".to_string()).with(|| {
                            context
                                .ui
                                .menu_item_config("kieran@dothq.org")
                                .enabled(false)
                                .build();
                            context.ui.separator();

                            if context.ui.menu_item_config("Your Spotify Account").build() {
                                context.widget.send_command(
                                    context.event_loop,
                                    AppCommand::OpenSpotifyAccount,
                                );
                            }

                            context.ui.separator();

                            if context
                                .ui
                                .menu_item_config("Preferences")
                                .shortcut("Ctrl+P")
                                .build()
                            {
                                context.widget.send_command(
                                    context.event_loop,
                                    AppCommand::Navigate(UI_ROUTE_PREFERENCES),
                                );
                            }

                            context
                                .ui
                                .menu_item_config("Log Out")
                                .shortcut("Ctrl+Shift+W")
                                .build();
                        });

                        context.ui.set_cursor_pos(vec2(
                            menuitem_start_x + 2.0,
                            (menuitem_size.y - 16.0 - 2.0) / 2.0,
                        ));
                        context.widget.create_icon(
                            context.ui,
                            UI_ICON_USER,
                            16.0,
                            context.ui.style().color(ColorId::Text),
                        );
                    }
                });
            });
        },
    );
}
