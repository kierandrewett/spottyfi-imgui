use easy_imgui::{
    vec2, ColorId, CustomRectIndex, ImGuiID, StyleValue, StyleVar, TableColumnFlags, TableFlags,
    TableRowFlags,
};

use crate::constants::UI_ALBUM_ART_SIZE;

use super::ComponentContext;

pub struct CardDetails {
    pub image: CustomRectIndex,
    pub title: &'static str,
    pub subtitle: &'static str,
}

pub fn build(context: &mut ComponentContext, details: CardDetails) {
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

            context
                .widget
                .create_image(context.ui, details.image, 180.0 / UI_ALBUM_ART_SIZE)
                .build();

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
