use easy_imgui::Color;

use crate::widget::theme::UITheme;

pub fn color_alpha(color: Color, alpha_mul: f32) -> Color {
    let mut cloned_color = color;

    cloned_color.a *= alpha_mul;
    cloned_color
}

pub fn color_darken(color: Color, darken_mul: f32) -> Color {
    let mut cloned_color = color;

    cloned_color.r *= (1.0 - darken_mul);
    cloned_color.g *= (1.0 - darken_mul);
    cloned_color.b *= (1.0 - darken_mul);

    cloned_color
}

pub fn color_lighten(color: Color, lighten_mul: f32) -> Color {
    let mut cloned_color = color;

    cloned_color.r += (1.0 - cloned_color.r) * lighten_mul;
    cloned_color.g += (1.0 - cloned_color.g) * lighten_mul;
    cloned_color.b += (1.0 - cloned_color.b) * lighten_mul;

    cloned_color
}

pub fn color_lighten_darken(theme: UITheme, color: Color, mul: f32) -> Color {
    match theme {
        UITheme::Dark => color_lighten(color, 1.0 - mul),
        _ => color_darken(color, mul),
    }
}

pub fn color_light_dark(theme: UITheme, light_color: Color, dark_color: Color) -> Color {
    match theme {
        UITheme::Dark => dark_color,
        _ => light_color,
    }
}

