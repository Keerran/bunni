/* eslint-disable */
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

declare global {
    interface Window {
        __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
    }
}

// Function avoids 'window not defined' in SSR
const invoke = () => window.__TAURI_INVOKE__;

export function getConnectors() {
    return invoke()<string[]>("get_connectors")
}

export function searchManga(idx: number, query: string) {
    return invoke()<SearchItem[]>("search_manga", { idx,query })
}

export function fetchManga(idx: number, id: string) {
    return invoke()<Manga>("fetch_manga", { idx,id })
}

export function fetchChapter(idx: number, id: string) {
    return invoke()<ChapterImages>("fetch_chapter", { idx,id })
}

export function toggleLiked(connectorIdx: number, id: string) {
    return invoke()<boolean>("toggle_liked", { connectorIdx,id })
}

export function isLiked(connectorIdx: number, mangaId: string) {
    return invoke()<boolean>("is_liked", { connectorIdx,mangaId })
}

export function fetchLiked() {
    return invoke()<([number, Manga])[]>("fetch_liked")
}

export function setMangaView(connectorIdx: number, mangaId: string, long: boolean) {
    return invoke()<null>("set_manga_view", { connectorIdx,mangaId,long })
}

export function getMangaView(connectorIdx: number, mangaId: string) {
    return invoke()<Format | null>("get_manga_view", { connectorIdx,mangaId })
}

export function markChapterRead(connectorIdx: number, chapterId: string) {
    return invoke()<null>("mark_chapter_read", { connectorIdx,chapterId })
}

export type ChapterImages = { images: string[]; format: Format }
export type Manga = { desc: SearchItem; chapters: Chapter[] }
export type SearchItem = { id: string; title: string; description: string; cover_url: string }
export type Chapter = { id: string; name: string; number: number; read: boolean | null }
export type Format = "Normal" | "Long"
