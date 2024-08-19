pub mod set;

use std::cell::RefMut;
use std::collections::HashMap;
use std::hash::Hash;

use easy_imgui::CustomRectIndex;
use easy_imgui::FontAtlasMut;
use easy_imgui::Ui;

use image::DynamicImage;
use image::GenericImage;
use image::GenericImageView;
use image::ImageError;

use tracing::error;
use tracing::info;
use tracing::warn;

use crate::constants::UI_ICONS_GAP_SIZE;
use crate::{constants::UI_ICONS_BASE_SIZE, App};

use super::Widget;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct IconOffset {
    pub x: u32,
    pub y: u32,
}

impl IconOffset {
    pub fn new(x: u32, y: u32) -> IconOffset {
        IconOffset { x, y }
    }
}

#[derive(Clone)]
pub struct IconSet {
    image: DynamicImage,
    scale: u32,
}

impl IconSet {
    fn get_icon_size(&self) -> u32 {
        UI_ICONS_BASE_SIZE * self.scale
    }

    fn get_icon_gap(&self) -> u32 {
        {
            UI_ICONS_GAP_SIZE * self.scale
        }
    }

    fn get_num_total_icons(&self) -> u32 {
        let size = self.image.width() + self.image.height();

        size / (self.get_icon_size() + self.get_icon_gap())
    }

    pub fn load_icons(
        self,
        icons: &mut HashMap<IconOffset, CustomRectIndex>,
        atlas: &mut FontAtlasMut<'_, App>,
    ) {
        let icon_size = self.get_icon_size();
        let icon_gap = self.get_icon_gap();
        let total_icons = self.get_num_total_icons();

        for icon_index in 0..total_icons {
            let icon_x = (icon_size + icon_gap) * icon_index;
            let icon_y = 0;

            let icon_offset = IconOffset {
                x: icon_x / self.scale,
                y: icon_y / self.scale,
            };

            let image_clone = self.image.clone();

            icons.insert(
                icon_offset,
                atlas.add_custom_rect_regular([icon_size, icon_size], move |_, img| {
                    let icons_set = image_clone.view(icon_x, icon_y, icon_size, icon_size);

                    img.copy_from(&*icons_set, 0, 0).unwrap();
                }),
            );
        }
    }
}

#[derive(Clone, Default)]
pub struct IconsManager {
    sets: HashMap<u32, IconSet>,
    icons: HashMap<IconOffset, CustomRectIndex>,
    pub icons_preferred_scale: u32,
}

impl IconsManager {
    pub fn new() -> IconsManager {
        IconsManager {
            sets: HashMap::new(),
            icons: HashMap::new(),
            icons_preferred_scale: 1,
        }
    }

    fn load_icon_set(&mut self, scale: u32, bytes: &'static [u8]) -> Result<IconSet, ImageError> {
        match image::load_from_memory(bytes) {
            Ok(image) => {
                let icons_set = IconSet {
                    image: image.clone(),
                    scale,
                };

                self.sets.insert(scale, icons_set.clone());

                Ok(icons_set)
            }
            Err(err) => {
                error!("Failed to load icons set at {}x: {:?}", scale, err);

                Err(err)
            }
        }
    }

    fn get_preferred_icons(&self, ui_scale: f32) -> Option<IconSet> {
        let mut icon_sets_keys = self.sets.keys().collect::<Vec<_>>();
        icon_sets_keys.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let preferred_scale = icon_sets_keys
            .iter()
            .rposition(|&&icon_scale| ui_scale >= icon_scale as f32)
            .map_or(1, |index| index + 1) as u32;

        if let Some(icon_set) = self.sets.get(&preferred_scale) {
            return Some(icon_set.clone());
        }

        None
    }

    pub fn get_icon(&self, offset: IconOffset) -> CustomRectIndex {
        match self.icons.get(&offset) {
            Some(icon) => *icon,
            None => {
                warn!("No icon character at offset {},{}", offset.x, offset.y);

                CustomRectIndex::default()
            }
        }
    }

    pub fn load_icon_sets(&mut self, ui_scale: f32) -> Option<IconSet> {
        self.sets = HashMap::new();
        self.icons = HashMap::new();
        self.icons_preferred_scale = 1;

        self.load_icon_set(1, include_bytes!("icons.png")).ok();
        self.load_icon_set(2, include_bytes!("icons@2x.png")).ok();
        self.load_icon_set(3, include_bytes!("icons@3x.png")).ok();

        self.get_preferred_icons(ui_scale)
    }

    pub fn build_icons(&mut self, icons_set: IconSet, atlas: &mut FontAtlasMut<'_, App>) {
        self.icons_preferred_scale = icons_set.scale;
        icons_set.clone().load_icons(&mut self.icons, atlas);

        info!(
            "Loaded {} icons from set at {}x",
            icons_set.get_num_total_icons(),
            icons_set.scale
        );
    }
}
