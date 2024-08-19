use std::collections::{btree_set::Iter, BTreeSet};

use easy_imgui::{vec2, Cond, Ui};

use crate::{
    constants::{UI_MODAL_AC_MIN_WINDOW_HEIGHT, UI_MODAL_AC_MIN_WINDOW_WIDTH},
    App,
};

pub mod about;

#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd)]
pub enum ModalType {
    About,
}

#[derive(Debug, Clone, Default)]
pub struct ModalManager {
    open_modals: BTreeSet<ModalType>,
}

impl ModalManager {
    pub fn new() -> Self {
        ModalManager {
            open_modals: BTreeSet::new(),
        }
    }

    pub fn has(&self, modal: ModalType) -> bool {
        self.open_modals.contains(&modal)
    }

    pub fn add(&mut self, modal: ModalType) {
        self.open_modals.insert(modal);
    }

    pub fn remove(&mut self, modal: ModalType) {
        self.open_modals.remove(&modal);
    }

    pub fn iter(&self) -> Iter<ModalType> {
        self.open_modals.iter()
    }

    pub fn first(&self) -> Option<&ModalType> {
        self.open_modals.first()
    }
}

pub enum CentreModalAxis {
    Horizontal,
    Vertical,
    Both,
    None,
}

fn safe_divide(a: f32, b: f32) -> Option<f32> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

pub fn do_centre_modal(ui: &Ui<App>, padding: &[f32; 2], mut axis: CentreModalAxis) {
    let dsp_size = ui.display_size();

    let x_padding = dsp_size[0] * padding[0];
    let y_padding = dsp_size[1] * padding[1];

    if dsp_size[0] < UI_MODAL_AC_MIN_WINDOW_WIDTH || dsp_size[1] < UI_MODAL_AC_MIN_WINDOW_HEIGHT {
        axis = CentreModalAxis::None;
    };

    let min_padding_axis_both = x_padding.min(y_padding);

    let real_x_padding = match axis {
        CentreModalAxis::None => 0.0,
        CentreModalAxis::Both => min_padding_axis_both,
        _ => x_padding,
    };
    let real_y_padding = match axis {
        CentreModalAxis::None => 0.0,
        CentreModalAxis::Both => min_padding_axis_both,
        _ => y_padding,
    };

    let size = vec2(dsp_size[0] - real_x_padding, dsp_size[1] - real_y_padding);

    ui.set_next_window_size_constraints(size, size);
    ui.set_next_window_size(size, Cond::Always);

    let window_x = safe_divide(real_x_padding, 2.0).unwrap_or(0.0);
    let window_y = safe_divide(real_y_padding, 2.0).unwrap_or(0.0);
    let window_pos = vec2(window_x, window_y);

    ui.set_next_window_pos(window_pos, Cond::Always, vec2(0.0, 0.0));
}
