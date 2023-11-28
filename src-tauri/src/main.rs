// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use connectors::Connectors;
use specta::collect_types;
use tauri::State;
use tauri_specta::ts;

use crate::connectors::SearchItem;

mod connectors;

#[tauri::command]
#[specta::specta]
fn get_connectors(connectors: State<Connectors>) -> Vec<String> {
    connectors.0.iter().map(|c| c.name().to_string()).collect()
}

#[tauri::command]
#[specta::specta]
async fn search_manga(
    connectors: State<'_, Connectors>,
    idx: u32,
    query: &str,
) -> Result<Vec<SearchItem>, String> {
    connectors[idx].search(query).await.map_err(|e| e.to_string())
}

fn main() {
    #[cfg(debug_assertions)]
    ts::export(
        collect_types![get_connectors, search_manga],
        "../src/lib/backend.ts",
    )
    .unwrap();

    tauri::Builder::default()
        .manage(Connectors::new())
        .invoke_handler(tauri::generate_handler![get_connectors, search_manga])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
