use crate::{
    constants::UI_ROUTE_DEFAULT,
    create_pane, dummy,
    widget::components::{
        card::{self, CardDetails},
        ComponentContext,
    },
};
use chrono::{offset::Local, Timelike};
use easy_imgui::{ImGuiID, TableColumnFlags, TableFlags};

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
}
