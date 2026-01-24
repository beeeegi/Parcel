#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod logging;

use logging::TauriLogger;

fn main() {
    TauriLogger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::select_output_folder,
            commands::select_input_file,
            commands::run_conversion,
            commands::get_logs,
            commands::clear_logs,
            commands::open_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
