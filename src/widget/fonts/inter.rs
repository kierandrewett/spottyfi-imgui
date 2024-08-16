use crate::widget::font::{FontFace, FontFamily, FontWeight};

pub fn build_font_family() -> FontFamily {
    let mut font = FontFamily::new();

    font.add_font_face(FontFace {
        bytes: include_bytes!("Inter-Thin.ttf").to_vec(),
        weight: FontWeight::Thin,
    });

    font.add_font_face(FontFace {
        bytes: include_bytes!("Inter-ThinItalic.ttf").to_vec(),
        weight: FontWeight::ThinItalic,
    });

    font.add_font_face(FontFace {
        bytes: include_bytes!("Inter-ExtraLight.ttf").to_vec(),
        weight: FontWeight::ExtraLight,
    });

    font.add_font_face(FontFace {
        bytes: include_bytes!("Inter-ExtraLightItalic.ttf").to_vec(),
        weight: FontWeight::ExtraLightItalic,
    });

    font.add_font_face(FontFace {
        bytes: include_bytes!("Inter-Light.ttf").to_vec(),
        weight: FontWeight::Light,
    });

    font.add_font_face(FontFace {
        bytes: include_bytes!("Inter-LightItalic.ttf").to_vec(),
        weight: FontWeight::LightItalic,
    });

    font.add_font_face(FontFace {
        bytes: include_bytes!("Inter-Regular.ttf").to_vec(),
        weight: FontWeight::Regular,
    });

    font.add_font_face(FontFace {
        bytes: include_bytes!("Inter-Italic.ttf").to_vec(),
        weight: FontWeight::RegularItalic,
    });

    font.add_font_face(FontFace {
        bytes: include_bytes!("Inter-Medium.ttf").to_vec(),
        weight: FontWeight::Medium,
    });

    font.add_font_face(FontFace {
        bytes: include_bytes!("Inter-MediumItalic.ttf").to_vec(),
        weight: FontWeight::MediumItalic,
    });

    font.add_font_face(FontFace {
        bytes: include_bytes!("Inter-SemiBold.ttf").to_vec(),
        weight: FontWeight::Semibold,
    });

    font.add_font_face(FontFace {
        bytes: include_bytes!("Inter-SemiBoldItalic.ttf").to_vec(),
        weight: FontWeight::SemiboldItalic,
    });

    font.add_font_face(FontFace {
        bytes: include_bytes!("Inter-Bold.ttf").to_vec(),
        weight: FontWeight::Bold,
    });

    font.add_font_face(FontFace {
        bytes: include_bytes!("Inter-BoldItalic.ttf").to_vec(),
        weight: FontWeight::BoldItalic,
    });

    font.add_font_face(FontFace {
        bytes: include_bytes!("Inter-ExtraBold.ttf").to_vec(),
        weight: FontWeight::ExtraBold,
    });

    font.add_font_face(FontFace {
        bytes: include_bytes!("Inter-ExtraBoldItalic.ttf").to_vec(),
        weight: FontWeight::ExtraBoldItalic,
    });

    font.add_font_face(FontFace {
        bytes: include_bytes!("Inter-Black.ttf").to_vec(),
        weight: FontWeight::Black,
    });

    font.add_font_face(FontFace {
        bytes: include_bytes!("Inter-BlackItalic.ttf").to_vec(),
        weight: FontWeight::BlackItalic,
    });

    font
}
