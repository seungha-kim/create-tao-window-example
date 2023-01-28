#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Weak;
use tao::platform::macos::WindowBuilderExtMacOS;
// use tao::platform::windows::WindowBuilderExtWindows;
use tauri::Manager;

fn main() {
    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("failed to build app");

    app.run(move |handle, event| match event {
        tauri::RunEvent::Ready => {
            add_overlay(handle);

            // The panic happens whether or not the overlay struct
            // is kept around.
            // let state: tauri::State<OverlayState> = handle.state();
            // state.set_overlay(overlay);
        }
        _ => {}
    })
}

pub fn add_overlay(app_handle: &tauri::AppHandle) -> Weak<tao::window::Window> {
    let window = app_handle
        .get_window("main")
        .expect("failed to get main window");

    // let hwnd = window.hwnd().expect("failed to get HWND");
    

    app_handle
        .create_tao_window(move || {
            let ns_window = window.ns_window().expect("failed to get NSWindow");
            let window_builder = tao::window::WindowBuilder::new()
                .with_always_on_top(false)
                .with_decorations(true)
                .with_resizable(true)
                .with_visible(true)
                .with_position(tao::dpi::LogicalPosition::<u32>::new(30, 30))
                // .with_owner_window(hwnd)
                .with_parent_window(ns_window)
                .with_inner_size(tao::dpi::LogicalSize::<u32>::new(200, 200));

            ("Overlay".to_string(), window_builder)
        })
        .expect("failed to create overlay window")

}
