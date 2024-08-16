use easy_imgui::FontInfo;

#[derive(Clone, Default)]
pub struct FontFamily {
    font_faces: Vec<FontFace>,
}

impl FontFamily {
    pub fn new() -> Self {
        FontFamily {
            font_faces: Vec::new(),
        }
    }

    pub fn add_font_face(&mut self, font: FontFace) {
        self.font_faces.push(font)
    }

    pub fn get_by_weight(&self, weight: FontWeight) -> Option<&FontFace> {
        self.font_faces.iter().find(|x| x.weight == weight)
    }
}

#[derive(Debug, Clone)]
pub struct FontFace {
    pub bytes: Vec<u8>,
    pub weight: FontWeight,
}

impl FontFace {
    pub fn build_font_info(&self, size: f32) -> FontInfo {
        FontInfo::new(self.bytes.clone(), size)
    }
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum FontWeight {
    Thin,
    ThinItalic,
    ExtraLight,
    ExtraLightItalic,
    Light,
    LightItalic,
    Regular,
    RegularItalic,
    Medium,
    MediumItalic,
    Semibold,
    SemiboldItalic,
    Bold,
    BoldItalic,
    ExtraBold,
    ExtraBoldItalic,
    Black,
    BlackItalic,
}
