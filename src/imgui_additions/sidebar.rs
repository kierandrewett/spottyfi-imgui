use std::ffi::CString;

use easy_imgui::{
    easy_imgui_sys::{
        ImGuiDir, ImGui_BeginViewportSideBar, ImGui_End, ImGui_EndChild, ImGui_GetMainViewport,
    },
    WindowFlags,
};

pub enum ViewportSidebarDirection {
    Up,
    Down,
    Left,
    Right,
}

pub fn begin_main_viewport_sidebar(
    window_name: &str,
    window_flags: WindowFlags,
    direction: ViewportSidebarDirection,
    size: f32,
) {
    unsafe {
        let viewport = ImGui_GetMainViewport();
        let window_name = CString::new(window_name).unwrap();
        let window_flags = window_flags.bits();
        let window_dir = match direction {
            ViewportSidebarDirection::Up => ImGuiDir::ImGuiDir_Up,
            ViewportSidebarDirection::Down => ImGuiDir::ImGuiDir_Down,
            ViewportSidebarDirection::Left => ImGuiDir::ImGuiDir_Left,
            ViewportSidebarDirection::Right => ImGuiDir::ImGuiDir_Right,
        };

        ImGui_BeginViewportSideBar(
            window_name.as_ptr(),
            viewport,
            window_dir,
            size,
            window_flags,
        );
    }
}

pub fn end_main_viewport_sidebar() {
    unsafe {
        ImGui_End();
    }
}
