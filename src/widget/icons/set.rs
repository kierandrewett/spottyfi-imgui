use super::IconOffset;
use crate::constants::{UI_ICONS_BASE_SIZE, UI_ICONS_GAP_SIZE};

macro_rules! get_icon_offset {
    ($index_x:expr, $index_y:expr) => {
        IconOffset {
            x: (UI_ICONS_BASE_SIZE + UI_ICONS_GAP_SIZE) * $index_x,
            y: (UI_ICONS_BASE_SIZE + UI_ICONS_GAP_SIZE) * $index_y,
        }
    };
}

pub const UI_ICON_HOME: IconOffset = get_icon_offset!(0, 0);
pub const UI_ICON_SEARCH: IconOffset = get_icon_offset!(1, 0);
pub const UI_ICON_BROWSE: IconOffset = get_icon_offset!(2, 0);
pub const UI_ICON_CHARTS: IconOffset = get_icon_offset!(3, 0);
pub const UI_ICON_HEART: IconOffset = get_icon_offset!(4, 0);
pub const UI_ICON_CALENDAR: IconOffset = get_icon_offset!(5, 0);
pub const UI_ICON_CLOCK: IconOffset = get_icon_offset!(6, 0);
pub const UI_ICON_MUSICAL_NOTE: IconOffset = get_icon_offset!(7, 0);
pub const UI_ICON_DISC: IconOffset = get_icon_offset!(8, 0);

// Media
pub const UI_ICON_MEDIA_PREVIOUS: IconOffset = get_icon_offset!(9, 0);
pub const UI_ICON_MEDIA_PAUSE: IconOffset = get_icon_offset!(10, 0);
pub const UI_ICON_MEDIA_PLAY: IconOffset = get_icon_offset!(11, 0);
pub const UI_ICON_MEDIA_NEXT: IconOffset = get_icon_offset!(12, 0);
pub const UI_ICON_MEDIA_EXPLICIT: IconOffset = get_icon_offset!(13, 0);

pub const UI_ICON_REPEAT: IconOffset = get_icon_offset!(14, 0);
pub const UI_ICON_REPEAT_SONG: IconOffset = get_icon_offset!(15, 0);
pub const UI_ICON_SHUFFLE: IconOffset = get_icon_offset!(16, 0);
pub const UI_ICON_VOLUME: IconOffset = get_icon_offset!(17, 0);
pub const UI_ICON_CHECK: IconOffset = get_icon_offset!(18, 0);
pub const UI_ICON_QUEUE: IconOffset = get_icon_offset!(19, 0);
pub const UI_ICON_USER: IconOffset = get_icon_offset!(20, 0);
pub const UI_ICON_ADD: IconOffset = get_icon_offset!(21, 0);
pub const UI_ICON_MINUS: IconOffset = get_icon_offset!(22, 0);
pub const UI_ICON_BACK: IconOffset = get_icon_offset!(23, 0);
pub const UI_ICON_FORWARD: IconOffset = get_icon_offset!(24, 0);
pub const UI_ICON_DOWNLOAD: IconOffset = get_icon_offset!(25, 0);
