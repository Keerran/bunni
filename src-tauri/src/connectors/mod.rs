use std::ops::Index;

use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use specta::Type;

use self::{mangadex::MangaDex, mangakakalot::MangaKakalot};

mod mangadex;
mod mangakakalot;

#[derive(Serialize, Deserialize, Type)]
pub struct SearchItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub cover_url: String,
}

#[derive(Serialize, Deserialize, Type)]
pub struct Chapter {
    pub id: String,
    pub name: String,
    pub number: f32,
}

#[derive(Serialize, Deserialize, Type)]
pub struct Manga {
    pub desc: SearchItem,
    pub chapters: Vec<Chapter>,
}

#[derive(Serialize, Deserialize, Type, Clone, Copy)]
pub enum Format {
    Normal,
    Long,
}

#[derive(Serialize, Deserialize, Type)]
pub struct ChapterImages {
    pub images: Vec<String>,
    pub format: Format
}

#[async_trait]
pub trait Connector: Send + Sync {
    fn name(&self) -> &str;
    fn url(&self) -> &str;
    async fn search(&self, query: &str) -> Result<Vec<SearchItem>, reqwest::Error>;
    async fn fetch_manga(&self, id: &str) -> Result<Manga, reqwest::Error>;
    async fn fetch_chapter(&self, id: &str) -> Result<ChapterImages, reqwest::Error>;
}

pub struct Connectors(pub Vec<Box<dyn Connector>>);

impl Connectors {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("Bunni/0.0.1")
            .build()
            .expect("failed to build HTTP client");
        Self(vec![
            Box::new(MangaDex::new(client.clone())),
            Box::new(MangaKakalot::new(client.clone())),
        ])
    }
}

impl Index<u32> for Connectors {
    type Output = Box<dyn Connector>;

    fn index(&self, index: u32) -> &Self::Output {
        &self.0[index as usize]
    }
}

