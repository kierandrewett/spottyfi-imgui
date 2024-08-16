use std::f64::MIN;

use easy_imgui::{cgmath::Array, vec2, ChildFlags, Cond, WindowFlags};

use crate::{
    constants::{UI_APP_NAME, UI_APP_VERSION, UI_MODAL_PADDING},
    widget::ComponentContext,
};

use super::{do_centre_modal, CentreModalAxis};

pub fn build(context: &ComponentContext) {
    context.ui.open_popup("###about_modal");

    do_centre_modal(context.ui, &[0.4, UI_MODAL_PADDING], CentreModalAxis::Vertical);

    context.ui.popup_modal_config("About###about_modal")
        .flags(WindowFlags::NoResize | WindowFlags::AlwaysAutoResize | WindowFlags::NoMove)
        .with(|| {
            context.ui.with_push(context.widget.font_h1, || context.ui.text(UI_APP_NAME));
            if let Some(version) = UI_APP_VERSION {
                context.ui.with_push(context.widget.font_bold, || {
                    context.ui.text(&format!("Version v{}", version))
                });
            }
        });
}
