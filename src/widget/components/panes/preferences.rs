use easy_imgui::{
    vec2, Color, ColorId, Cond, ImGuiID, StyleValue, StyleVar, TableColumnFlags, TableFlags, Ui,
    Window, WindowFlags,
};
use tracing::info;

use crate::{
    constants::UI_ROUTE_PREFERENCES,
    create_pane,
    event::AppEvent,
    utils::{color_darken, color_light_dark},
    widget::{theme::UITheme, Widget},
    App,
};

use super::ComponentContext;

macro_rules! gen_pref_section {
    ($ui:expr, $widget:expr, $title:expr, $subtitle:expr, $render:block) => {
        $ui.with_push($widget.font_h4, || {
            $ui.text($title);
        });

        if let Some(text) = $subtitle {
            $ui.with_push((
                ColorId::Text,
                $ui.style().color_alpha(ColorId::Text, 0.85)
            ), || {
                $ui.text(text);
            });
        }

        $ui.spacing();

        $render

        $ui.dummy(vec2(0.0, 10.0));
    };
}

pub fn build(mut context: &mut ComponentContext) {
    let mut open = context.widget.state.panes.preferences_visible;

    create_pane!(context.ui, context.widget, UI_ROUTE_PREFERENCES, open, {
        let window_width = context.ui.get_window_width();

        let content_min_width = 800.0;
        let imgui_gutter_width = 50.0;

        let include_gutters = if window_width < content_min_width + (imgui_gutter_width * 2.0) {
            false
        } else {
            true
        };

        context.ui.table_config("Preferences", if include_gutters { 3 } else { 1 })
            .flags(TableFlags::None)
            .with(|| {
                if include_gutters {
                    context.ui.table_setup_column(
                        "spacer_1",
                        TableColumnFlags::WidthStretch,
                        -1.0,
                        ImGuiID::default(),
                    );
                }

                context.ui.table_setup_column(
                    "content",
                    TableColumnFlags::WidthFixed,
                    content_min_width,
                    ImGuiID::default(),
                );

                if include_gutters {
                    context.ui.table_setup_column(
                        "spacer_2",
                        TableColumnFlags::WidthStretch,
                        -1.0,
                        ImGuiID::default(),
                    );
                }

                if include_gutters {
                    context.ui.table_next_column();
                }

                // Begin rendering
                context.ui.table_next_column();
                context.ui.with_push(context.widget.font_h3, || {
                    context.ui.text("Preferences");
                });

                context.ui.dummy(vec2(0.0, 10.0));

                gen_pref_section!(
                    context.ui,
                    context.widget,
                    "Theme",
                    Some("Change the look and feel of the app."),
                    {
                        let theme = context.widget.get_theme();

                        if context.ui
                            .radio_button_config("System", theme == UITheme::System)
                            .build()
                        {
                            context.event_loop
                                .send_event(AppEvent::SetTheme(UITheme::System))
                                .ok();
                        }
                        if context.ui
                            .radio_button_config("Light", theme == UITheme::Light)
                            .build()
                        {
                            context.event_loop
                                .send_event(AppEvent::SetTheme(UITheme::Light))
                                .ok();
                        }
                        if context.ui
                            .radio_button_config("Dark", theme == UITheme::Dark)
                            .build()
                        {
                            context.event_loop
                                .send_event(AppEvent::SetTheme(UITheme::Dark))
                                .ok();
                        }
                    }
                );

                gen_pref_section!(
                    context.ui,
                    context.widget,
                    "Language",
                    Some("Language to use throughout the app."),
                    {
                        let mut value = "en_GB";

                        context.ui.combo(
                            "",
                            ["en_GB"],
                            |v| match v {
                                "en_GB" => "English (United Kingdom)",
                                _ => "Unknown",
                            },
                            &mut value,
                        );
                    }
                );

                gen_pref_section!(context.ui, context.widget, "Audio quality", None, {
                    #[derive(Clone, Copy, PartialEq)]
                    enum StreamingQuality {
                        Low,
                        Normal,
                        High,
                    }

                    let mut value = StreamingQuality::Normal;

                    context.ui.combo(
                        "Streaming quality",
                        [
                            StreamingQuality::Low,
                            StreamingQuality::Normal,
                            StreamingQuality::High,
                        ],
                        |v| match v {
                            StreamingQuality::Low => "Low - 96 kbps",
                            StreamingQuality::Normal => "Normal - 160 kbps",
                            StreamingQuality::High => "High - 320 kbps",
                        },
                        &mut value,
                    );
                });

                // End rendering

                if include_gutters {
                    context.ui.table_next_column();
                }
            });
    });
}
