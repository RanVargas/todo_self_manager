#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Emitter, Manager, AppHandle};
use tauri::tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState};


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn set_title(app_handle: tauri::AppHandle, title: String) {
    // "default" is the standard tray icon ID if you created only one
    match app_handle.tray_by_id("default") {
        Some(tray) => {
            if let Err(e) = tray.set_title(Some(&title)) {
                eprintln!("error updating tray title: {}", e);
            }
        }
        None => {
            eprintln!("tray icon with id 'default' not found");
        }
    }
}

fn main() {
    //let tray = SystemTray::new();
    todo_self_manager_lib::run();
    let win_app = tauri::Builder::default()
        .setup(|app| {
            let tray = TrayIconBuilder::new().build(app)?;
            Ok(())
        }).on_window_event(|window, event| match event {
        tauri::WindowEvent::CloseRequested {api, ..} => {
            window.hide().unwrap();
            api.prevent_close();
        }

        _ => {}
    }).on_tray_icon_event(|tray, event| match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } => {
            println!("left click pressed and released");

            let app = tray.app_handle();
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.unminimize();
                let _ = window.show();
                let _ = window.show();
            }
        }
        _ => {
            println!("{event:?}");
        }
    }).invoke_handler(tauri::generate_handler![set_title])
        .run(tauri::generate_context!())
        .expect("error while running app");

    /*
    .invoke_handler(tauri::generate_handler![set_title])
    .run(tauri::generate_context!())
    .expect("error while running tauri application")
    */
    //todo_self_manager_lib::run();
    //tauri::tray::TrayIconBuilder::default().
    /*tauri::Builder::default()
    .on_window_event(|event| match event.event() {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            let window = event.window();
            window.hide().unwrap();
            api.prevent_close();
        }
        _ => {}
    })
    .system_tray(tray)
    .on_system_tray_event(|app, event| match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
        }
        _ => {}
    })
    .invoke_handler(tauri::generate_handler![set_title])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");*/
}
