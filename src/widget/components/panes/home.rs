use std::sync::Arc;

use crate::{
    api::{error::SpotifyAPIError, models::{recommendations::{BrowseRecommendationItem, BrowseRecommendationSections}, user::UserImpl as _}}, constants::UI_ROUTE_DEFAULT, create_pane, dummy, widget::components::{
        self, card::{self, CardDetails}, ComponentContext
    }
};
use chrono::{offset::Local, Timelike};
use easy_imgui::{easy_imgui_sys::ImGui_ClearDragDrop, ImGuiID, TableColumnFlags, TableFlags};
use tokio::runtime::Handle;
use tracing::error;

pub fn build(context: &mut ComponentContext) {
    let mut open = context.widget.state.lock().unwrap().home_visible;

    let time = Local::now();

    let last_auth_error = context.api.get_state_error();

    let recommendations_sections = &context.widget.state
        .lock()
        .unwrap()
        .recommendations
        .clone()
        .and_then(|r| r.sections);

    create_pane!(context.ui, context.widget, UI_ROUTE_DEFAULT, open, {
        if let Some(profile) = context.api.state().and_then(|s| s.profile) {
            context.ui.with_push(context.widget.font_h2, || {
                context.ui.text(&format!("{}, {}", match time.hour() {
                    5..12 => "Good morning",
                    12..16 => "Good afternoon",
                    _ => "Good evening",
                }, profile.name()))
            });

            dummy!(context);

            match recommendations_sections {
                Some(BrowseRecommendationSections::Sections(Ok(sections))) => {
                    for section in sections {
                        context.ui.with_push(context.widget.font_h3, || {
                            context.ui.text(&section.title);
                        });

                        if let Some(desc) = &section.description {
                            context.ui.text(desc);
                        }

                        for item in &section.items {
                            match item {
                                BrowseRecommendationItem::Playlist(playlist) => {
                                    context.ui.with_push(context.widget.font_h4, || {
                                        context.ui.text(playlist.name.as_str());
                                    });

                                    if let Some(desc) = &playlist.description {
                                        context.ui.text(desc);
                                    }
                                },
                                r => {
                                    error!("Unhandled recommendation item {:?}", r);
                                    panic!("Panic here.");
                                }
                            }
                        }
                    }
                },
                Some(BrowseRecommendationSections::Sections(Err(err))) => {
                    components::error::build(
                        context,
                        Box::new(err.clone())
                    );
                },
                _ => {
                    context.ui.text("Loading...");
                }
            }
        } else {
            components::error::build(
                context,
                Box::new(last_auth_error.unwrap_or(
                    SpotifyAPIError::Unknown("Failed to establish connection to Spotify.")
                ))
            );
        }
    });
}
