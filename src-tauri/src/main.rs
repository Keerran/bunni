// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use connectors::Connectors;
use specta::collect_types;
use tauri_specta::ts;

mod connectors;

#[tauri::command]
#[specta::specta]
fn get_connectors(connectors: tauri::State<Connectors>) -> Vec<String> {
    connectors.0.iter().map(|c| c.name()).collect()
}

fn main() {
    #[cfg(debug_assertions)]
    ts::export(collect_types![
        get_connectors,
    ], "../src/lib/backend.ts").unwrap();

    tauri::Builder::default()
        .manage(Connectors::new())
        .invoke_handler(tauri::generate_handler![get_connectors])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
