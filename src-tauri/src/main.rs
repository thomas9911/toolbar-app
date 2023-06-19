// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    AppHandle, CustomMenuItem, GlobalWindowEvent, Manager, PageLoadPayload, SystemTray,
    SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu, Window,
};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn system_tray() -> SystemTray {
    let mut sub_tray_menu = SystemTrayMenu::new();
    for i in 1..10 {
        sub_tray_menu = sub_tray_menu.add_item(CustomMenuItem::new(
            format!("testing{i}"),
            format!("Testing{i}"),
        ));
    }

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_item(show)
        .add_submenu(SystemTraySubmenu::new("testing", sub_tray_menu));

    let system_tray = SystemTray::new().with_menu(tray_menu);

    system_tray
}

fn on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a left click");
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a right click");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a double click");
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => app.exit(0),
            "hide" => {
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
            }
            "show" => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            _ => {}
        },
        _ => {}
    }
}

fn on_window_event(event: GlobalWindowEvent) {
    match event.event() {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            event.window().hide().unwrap();
            api.prevent_close();
        }
        _ => {}
    }
}

fn on_page_load(window: Window, _: PageLoadPayload) {
    window.hide().unwrap();
}

fn main() {
    tauri::Builder::default()
        .system_tray(system_tray())
        .on_page_load(on_page_load)
        .on_system_tray_event(on_system_tray_event)
        .on_window_event(on_window_event)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
