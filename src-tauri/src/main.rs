#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod msn_msgr_ui_manager;
use msn_msgr_ui_manager::send_now_playing;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handle_send_now_playing])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn handle_send_now_playing(artist: String, title: String, album: String) {
    let format = "{1} - {0}";
    send_now_playing(format, &artist, &title, &album);
}
