use std::cell::{RefCell, RefMut};

use easy_imgui::{vec2, Color, ColorId, Pushable, Style, StyleValue, StyleVar, Ui};

use crate::{
    constants::{
        UI_ACCENT_COLOR, UI_DARK_CHROME_BG_COLOR, UI_DARK_WINDOW_BG_COLOR,
        UI_LIGHT_CHROME_BG_COLOR, UI_LIGHT_WINDOW_BG_COLOR,
    },
    utils::{color_alpha, color_darken, color_light_dark, color_lighten},
    App, WidgetRc,
};

use super::{theme::UITheme, Widget};

pub fn push_style<R>(widget: &WidgetRc, ui: &Ui<App>, cb: impl FnOnce() -> R) {
    let font_size = ui.get_font_size();

    let style = (
        (
            StyleVar::ItemSpacing,
            StyleValue::Vec2(vec2(font_size / 2.0, font_size / 2.0)),
        ),
        (
            StyleVar::FramePadding,
            StyleValue::Vec2(vec2(font_size / 4.0, font_size / 4.0)),
        ),
        (StyleVar::WindowBorderSize, StyleValue::F32(1.0)),
        (
            (StyleVar::DockingSeparatorSize, StyleValue::F32(2.0)),
            (StyleVar::WindowRounding, StyleValue::F32(6.0)),
            (StyleVar::FrameRounding, StyleValue::F32(4.0)),
        ),
    );

    let chrome_bg_color = match widget.borrow().state.current_theme {
        UITheme::Dark => UI_DARK_CHROME_BG_COLOR,
        _ => UI_LIGHT_CHROME_BG_COLOR,
    };

    let window_bg_color = match widget.borrow().state.current_theme {
        UITheme::Dark => UI_DARK_WINDOW_BG_COLOR,
        _ => UI_LIGHT_WINDOW_BG_COLOR,
    };

    let docking_empty_bg =
        color_light_dark(widget.borrow().state.current_theme, window_bg_color, 0.25);

    let separator = ui.style().color_alpha(ColorId::Text, 0.125);
    let separator_hover = ui.style().color_alpha(ColorId::Text, 0.25);
    let separator_active = UI_ACCENT_COLOR;

    let colors = (
        (
            ColorId::HeaderHovered,
            ui.style().color_alpha(ColorId::Text, 0.125),
        ),
        (
            ColorId::TitleBg,
            color_light_dark(widget.borrow().state.current_theme, window_bg_color, 0.9),
        ),
        (
            ColorId::TitleBgActive,
            color_light_dark(widget.borrow().state.current_theme, window_bg_color, 0.2),
        ),
        (
            (ColorId::HeaderActive, color_alpha(UI_ACCENT_COLOR, 0.5)),
            (ColorId::DockingEmptyBg, docking_empty_bg),
            (ColorId::Tab, Color::TRANSPARENT),
            (
                (ColorId::TabDimmed, Color::TRANSPARENT),
                (
                    ColorId::TabHovered,
                    ui.style().color_alpha(ColorId::Text, 0.1),
                ),
                (ColorId::TabSelected, window_bg_color),
                (
                    (
                        ColorId::TabDimmedSelected,
                        color_light_dark(
                            widget.borrow().state.current_theme,
                            window_bg_color,
                            0.25,
                        ),
                    ),
                    (ColorId::TabSelectedOverline, UI_ACCENT_COLOR),
                    (
                        ColorId::Button,
                        Color {
                            r: 0.5,
                            g: 0.5,
                            b: 0.5,
                            a: 0.2,
                        },
                    ),
                    (
                        (
                            ColorId::ButtonHovered,
                            Color {
                                r: 0.5,
                                g: 0.5,
                                b: 0.5,
                                a: 0.4,
                            },
                        ),
                        (
                            ColorId::ButtonActive,
                            Color {
                                r: 0.5,
                                g: 0.5,
                                b: 0.5,
                                a: 0.6,
                            },
                        ),
                        (ColorId::SeparatorHovered, separator_hover),
                        (
                            (ColorId::SeparatorActive, separator_active),
                            (ColorId::ResizeGrip, separator),
                            (ColorId::ResizeGripHovered, separator_hover),
                            (
                                (ColorId::ResizeGripActive, separator_active),
                                (ColorId::ScrollbarBg, Color::TRANSPARENT),
                                (ColorId::WindowBg, window_bg_color),
                                (
                                    (
                                        ColorId::FrameBg,
                                        Color {
                                            r: 0.5,
                                            g: 0.5,
                                            b: 0.5,
                                            a: 0.2,
                                        },
                                    ),
                                    (
                                        ColorId::FrameBgHovered,
                                        Color {
                                            r: 0.5,
                                            g: 0.5,
                                            b: 0.5,
                                            a: 0.4,
                                        },
                                    ),
                                    (
                                        ColorId::FrameBgActive,
                                        Color {
                                            r: 0.5,
                                            g: 0.5,
                                            b: 0.5,
                                            a: 0.75,
                                        },
                                    ),
                                    (
                                        (ColorId::CheckMark, UI_ACCENT_COLOR),
                                        (ColorId::SliderGrab, UI_ACCENT_COLOR),
                                        (
                                            ColorId::SliderGrabActive,
                                            color_alpha(UI_ACCENT_COLOR, 0.75),
                                        ),
                                        (
                                            ColorId::TextSelectedBg,
                                            color_alpha(UI_ACCENT_COLOR, 0.25),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );

    ui.with_push(style, || {
        ui.with_push(colors, || {
            cb();
        });
    })
}
