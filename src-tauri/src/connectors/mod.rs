use std::ops::Index;

use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use specta::Type;

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

#[derive(Serialize, Deserialize, Type)]
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
        Self(vec![])
    }
}

impl Index<usize> for Connectors {
    type Output = Box<dyn Connector>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

