use easy_imgui::{
    vec2, ColorId, CustomRectIndex, ImGuiID, StyleValue, StyleVar, TableColumnFlags, TableFlags,
    TableRowFlags,
};
use rspotify_model::{FullTrack, SimplifiedTrack};

use crate::constants::UI_ALBUM_ART_SIZE;

use super::ComponentContext;

pub enum CardType {
    FullTrack(FullTrack),
    SimplifiedTrack(SimplifiedTrack)
}

#[derive(Debug, Default)]
pub struct CardDetails {
    title: String,
    subtitle: Option<String>,
    image: Option<String>,
    href: Option<String>
}

pub fn build(context: &mut ComponentContext, data: CardType) {
    let details = match data {
        CardType::FullTrack(track) => CardDetails {
            image: track.album.images
                .first().map(|i| i.url.clone()),
            ..Default::default()
        },
        CardType::SimplifiedTrack(track) => CardDetails {
            image: track.album
                .and_then(|a|
                    a.images
                        .first()
                        .map(|i| i.url.clone())
                ),
            ..Default::default()
        }
    };

    context
        .ui
        .table_config("Card", 1)
        .flags(TableFlags::Borders)
        .with(|| {
            context.ui.table_setup_column(
                "Card Details",
                TableColumnFlags::WidthStretch,
                -1.0,
                ImGuiID::default(),
            );

            // context.ui.table_next_row(TableRowFlags::None, 180.0);
            context.ui.table_next_column();

            // let image = image::load_from_memory(reqwest::)

            // context
            //     .widget
            //     .create_image(context.ui, data.image, 180.0 / UI_ALBUM_ART_SIZE)
            //     .build();

            context.ui.table_next_row(TableRowFlags::None, 200.0);

            context.ui.with_push(
                (StyleVar::ItemSpacing, StyleValue::Vec2(vec2(0.0, 2.0))),
                || {
                    context.ui.with_push(
                        (
                            context.widget.font_bold,
                            (ColorId::Text, context.ui.style().color(ColorId::Text)),
                        ),
                        || {
                            context.ui.text("All Out 80s");
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
                            context.ui.text("By Spotify");
                        },
                    );
                },
            );
        });
}
