use std::{borrow::BorrowMut, sync::Arc};

use crate::{
    api::models::user::UserImpl, commands::AppCommand, constants::UI_ROUTE_SEARCH, create_pane, dummy, state::search::WidgetStateSearchResults, widget::{
        components::{
            self, card::{self, CardDetails}, ComponentContext
        },
        icons::set::UI_ICON_SEARCH,
    }
};
use easy_imgui::{
    ColorId, ImGuiID, InputTextFlags, TableColumnFlags,
    TableFlags,
};
use rspotify_model::SearchResult;

pub fn build(context: &mut ComponentContext) {
    let state_arc = Arc::clone(&context.widget.state);

    let mut open = state_arc.lock().unwrap().search.visible;

    let search_value = &state_arc
        .lock()
        .unwrap()
        .search
        .search_value
        .clone();

    let input_icon_size = 16.0 * context.widget.ui_scale;
    let input_padding = 12.0 * context.widget.ui_scale;
    let input_padding_start = input_padding + (input_icon_size * 2.0);

    let font_h3 = context.widget.font_h3;

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

                                if context
                                    .ui
                                    .input_text_hint_config(
                                        "##SearchField",
                                        "Search artists, songs or albums",
                                        &mut state_arc
                                            .lock()
                                            .unwrap()
                                            .search
                                            .search_value,
                                    )
                                    .flags(InputTextFlags::EscapeClearsAll)
                                    .build()
                                {
                                    let evt = AppCommand::DoSearch(state_arc
                                        .lock()
                                        .unwrap()
                                        .search
                                        .search_value.clone()
                                    );

                                    context.widget.send_command(context.event_loop, evt);
                                }
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

                match &state_arc
                    .lock()
                    .unwrap()
                    .search
                    .search_results
                {
                    WidgetStateSearchResults::Fetched(Ok(results)) => {
                        if results.is_empty() {
                            context.ui.with_push(font_h3, || {
                                context.ui.text("No results found.");
                            });
                        } else {
                            if let Some(tracks) = &results.tracks {
                                context.ui.with_push(font_h3, || {
                                    context.ui.text("Songs");
                                    dummy!(context);
                                });

                                for track in &tracks.items {
                                    context.ui.text(&format!(
                                        "{} by {} on {}",
                                        track.name,
                                        track.artists.iter()
                                            .map(|a| a.name.clone())
                                            .collect::<Vec<String>>()
                                            .join(", "),
                                        track.album.name
                                    ));
                                }
                            }
    
                            if let Some(artists) = &results.artists {
                                context.ui.with_push(font_h3, || {
                                    context.ui.text("Artists");
                                    dummy!(context);
                                });

                                for artist in &artists.items {
                                    context.ui.text(&artist.name);
                                }
                            }
    
                            if let Some(albums) = &results.albums {
                                context.ui.with_push(font_h3, || {
                                    context.ui.text("Albums");
                                    dummy!(context);
                                });

                                for album in &albums.items {
                                    context.ui.text(&format!(
                                        "{} created by {}",
                                        album.name,
                                        album.artists.iter()
                                            .map(|a| a.name.clone())
                                            .collect::<Vec<String>>()
                                            .join(", "),
                                    ));
                                }
                            }
    
                            if let Some(playlists) = &results.playlists {
                                context.ui.with_push(font_h3, || {
                                    context.ui.text("Playlists");
                                    dummy!(context);
                                });

                                for playlist in &playlists.items {
                                    context.ui.text(&format!(
                                        "{} created by {}",
                                        playlist.name,
                                        playlist.owner.name()
                                    ));
                                }
                            }
                        }
                    },
                    WidgetStateSearchResults::Fetched(Err(err)) => {
                        components::error::build(context, Box::new(err.clone()));
                    },
                    WidgetStateSearchResults::None => {
                        context.ui.with_push(font_h3, || {
                            context.ui.text("Recent searches");
                        });
                    },
                    _ => {}
                }
            },
        )
    });
}
