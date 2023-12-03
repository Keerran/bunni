// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(async_closure)]

use connectors::{ChapterImages, Connectors, Format, Manga};
use futures::future::join_all;
use prefs::StoredManga;
use serde::Serialize;
use specta::collect_types;
use tauri::{Manager, State, AppHandle};
use tauri_specta::ts;

use crate::{connectors::SearchItem, prefs::UserPrefs};

mod connectors;
mod prefs;

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
    connectors[idx]
        .search(query)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
async fn fetch_manga(
    prefs: State<'_, UserPrefs>,
    connectors: State<'_, Connectors>,
    idx: u32,
    id: &str,
) -> Result<Manga, String> {
    let read = {
        let mut data = prefs.inner.lock().unwrap();
        data.read.entry(idx).or_default().clone()
    };
    connectors[idx]
        .fetch_manga(id)
        .await
        .map(|mut manga| {
            manga.chapters.iter_mut().for_each(|c| {
                c.read = Some(read.contains(&c.id));
            });
            manga
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
async fn fetch_chapter(
    connectors: State<'_, Connectors>,
    idx: u32,
    id: &str,
) -> Result<ChapterImages, String> {
    connectors[idx]
        .fetch_chapter(id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
async fn toggle_liked(
    prefs: State<'_, UserPrefs>,
    connector_idx: u32,
    id: &str,
) -> Result<bool, ()> {
    let mut data = prefs.inner.lock().unwrap();
    let item = StoredManga {
        manga_id: id.to_string(),
        connector_idx,
    };

    let is_liked = data.liked.contains(&item);
    if is_liked {
        data.liked.retain(|it| *it != item);
    } else {
        data.liked.push(item);
    }

    drop(data);
    prefs.save().unwrap();

    Ok(!is_liked)
}

#[tauri::command]
#[specta::specta]
async fn is_liked(
    prefs: State<'_, UserPrefs>,
    connector_idx: u32,
    manga_id: String,
) -> Result<bool, ()> {
    let data = prefs.inner.lock().unwrap();
    let manga = StoredManga {
        connector_idx,
        manga_id,
    };
    Ok(data.liked.contains(&manga))
}

#[tauri::command]
#[specta::specta]
async fn fetch_liked(
    connectors: State<'_, Connectors>,
    prefs: State<'_, UserPrefs>,
) -> Result<Vec<(u32, Manga)>, ()> {
    let data = prefs.inner.lock().unwrap().liked.clone();
    Ok(join_all(data.iter().map(|saved| {
        let connector = &connectors[saved.connector_idx];
        connector.fetch_manga(&saved.manga_id)
    }))
    .await
    .into_iter()
    .enumerate()
    .filter_map(|(i, r)| Some((data[i].connector_idx, r.ok()?)))
    .collect())
}

#[tauri::command]
#[specta::specta]
fn set_manga_view(
    prefs: State<'_, UserPrefs>,
    connector_idx: u32,
    manga_id: String,
    long: bool,
) -> Result<(), ()> {
    let mut data = prefs.inner.lock().unwrap();

    data.views.entry(connector_idx).or_default().insert(
        manga_id,
        match long {
            true => Format::Long,
            false => Format::Normal,
        },
    );

    drop(data);
    prefs.save().unwrap();

    Ok(())
}

#[tauri::command]
#[specta::specta]
fn get_manga_view(
    prefs: State<'_, UserPrefs>,
    connector_idx: u32,
    manga_id: String,
) -> Result<Option<Format>, ()> {
    let data = prefs.inner.lock().unwrap();
    Ok(data.views.get(&connector_idx).and_then(|c| c.get(&manga_id)).copied())
}

#[tauri::command]
#[specta::specta]
fn mark_chapter_read(
    app: AppHandle,
    prefs: State<'_, UserPrefs>,
    connector_idx: u32,
    chapter_id: String,
) -> Result<(), ()> {
    let mut data = prefs.inner.lock().unwrap();
    data.read.entry(connector_idx).or_default().insert(chapter_id.clone());
    drop(data);
    prefs.save().unwrap();

    app.emit_all("chapter_read", ReadEvent {
        connector_idx, chapter_id
    }).unwrap();

    Ok(())
}

#[derive(Clone, Serialize)]
struct ReadEvent {
    connector_idx: u32,
    chapter_id: String,
}

fn main() {
    #[cfg(debug_assertions)]
    ts::export(
        collect_types![
            get_connectors,
            search_manga,
            fetch_manga,
            fetch_chapter,
            toggle_liked,
            is_liked,
            fetch_liked,
            set_manga_view,
            get_manga_view,
            mark_chapter_read,
        ],
        "../src/lib/backend.ts",
    )
    .unwrap();

    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app.path_resolver().app_data_dir().unwrap();
            println!("data_dir={data_dir:?}");
            let _handle = app.handle();
            app.manage(UserPrefs::new(data_dir));
            app.manage(Connectors::new());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_connectors,
            search_manga,
            fetch_manga,
            fetch_chapter,
            toggle_liked,
            is_liked,
            fetch_liked,
            set_manga_view,
            get_manga_view,
            mark_chapter_read,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
