use std::collections::HashMap;

use async_trait::async_trait;
use serde::Deserialize;

use crate::connectors::{Chapter, ChapterImages, Connector, Format, Manga, SearchItem};

#[derive(Clone)]
pub struct MangaDex {
    client: reqwest::Client
}

impl MangaDex {
    pub fn new(client: reqwest::Client) -> Self {
        MangaDex { client }
    }
}

type LocalizedString = HashMap<String, String>;

#[derive(Deserialize)]
struct CoverArt {
    #[serde(rename = "fileName")]
    file_name: String,
}

#[derive(Deserialize)]
struct ItemRelationship {
    #[serde(rename = "type")]
    item_type: String,
    attributes: Option<CoverArt>,
}

#[derive(Deserialize)]
struct ItemAttributes {
    title: LocalizedString,
    description: LocalizedString,
}

#[derive(Deserialize)]
struct SearchManga {
    id: String,
    attributes: ItemAttributes,
    relationships: Vec<ItemRelationship>,
}

impl From<SearchManga> for SearchItem {
    fn from(value: SearchManga) -> Self {
        let id = value.id;
        let cover = value
            .relationships
            .into_iter()
            .find(|e| e.item_type == "cover_art")
            .map(|x| x.attributes.unwrap().file_name)
            .unwrap_or_default();
        let titles = value.attributes.title;
        let cover_url = format!("https://uploads.mangadex.org/covers/{id}/{cover}.256.jpg");
        SearchItem {
            id,
            title: titles
                .get("en")
                .or(titles.get("jp"))
                .map(|s| s.to_string())
                .unwrap_or_default(),
            description: value
                .attributes
                .description
                .get("en")
                .map(|s| s.to_string())
                .unwrap_or_default(),
            cover_url,
        }
    }
}

#[derive(Deserialize)]
struct ChapterAttributes {
    volume: Option<String>,
    chapter: Option<String>,
}

#[derive(Deserialize)]
struct TagAttributes {
    name: LocalizedString,
}

#[derive(Deserialize)]
struct MangaTag {
    attributes: TagAttributes,
}

#[derive(Deserialize)]
struct MangaAttributes {
    tags: Vec<MangaTag>,
}

#[derive(Deserialize)]
struct ChapterRelationships {
    #[serde(rename = "type")]
    item_type: String,
    attributes: Option<MangaAttributes>,
}

#[derive(Deserialize)]
struct ChapterData {
    id: String,
    attributes: ChapterAttributes,
    relationships: Vec<ChapterRelationships>,
}

impl From<ChapterData> for Chapter {
    fn from(value: ChapterData) -> Self {
        Chapter {
            id: value.id,
            name: {
                let vol = value.attributes.volume.map(|v| format!("Vol.{v} "));
                let chap = value
                    .attributes
                    .chapter
                    .clone()
                    .map(|c| format!("Chap.{c}"));

                format!("{}{}", vol.unwrap_or_default(), chap.unwrap_or_default())
            },
            number: value
                .attributes
                .chapter
                .and_then(|c| c.parse().ok())
                .unwrap_or(0f32),
            read: None,
        }
    }
}

#[derive(Deserialize)]
struct AtHomeChapter {
    data: Vec<String>,
    hash: String,
}

#[derive(Deserialize)]
struct AtHomeData {
    #[serde(rename = "baseUrl")]
    base_url: String,
    chapter: AtHomeChapter,
}

#[derive(Deserialize)]
struct ApiResponse<T> {
    data: T,
}

#[async_trait]
impl Connector for MangaDex {
    fn name(&self) -> &str {
        "MangaDex"
    }

    fn url(&self) -> &str {
        "https://api.mangadex.org"
    }

    async fn search(&self, query: &str) -> Result<Vec<SearchItem>, reqwest::Error> {
        let url = self.url();
        let results = self.client.get(format!(
            "{url}/manga?title={query}&includes[]=cover_art&limit=50"
        ))
        .send()
        .await?
        .json::<ApiResponse<Vec<SearchManga>>>()
        .await?;

        Ok(results.data.into_iter().map(|it| it.into()).collect())
    }

    async fn fetch_manga(&self, id: &str) -> Result<Manga, reqwest::Error> {
        let url = self.url();
        let manga = self.client.get(format!("{url}/manga/{id}?includes[]=cover_art"))
            .send()    
            .await?
            .json::<ApiResponse<SearchManga>>()
            .await?;
        let volumes = self.client.get(format!(
            "{url}/chapter?limit=100&manga={id}&translatedLanguage[]=en&order[chapter]=desc"
        ))
        .send()
        .await?
        .json::<ApiResponse<Vec<ChapterData>>>()
        .await?;

        Ok(Manga {
            desc: manga.data.into(),
            chapters: volumes.data.into_iter().map(|c| c.into()).collect(),
        })
    }

    async fn fetch_chapter(&self, id: &str) -> Result<ChapterImages, reqwest::Error> {
        let url = self.url();
        let chapter = self.client.get(format!("{url}/chapter/{id}?includes[]=manga"))
            .send()
            .await?
            .json::<ApiResponse<ChapterData>>()
            .await?
            .data;
        let at_home = self.client.get(format!("{url}/at-home/server/{id}"))
            .send()
            .await?
            .json::<AtHomeData>()
            .await?;

        let tags = chapter
            .relationships
            .into_iter()
            .find(|r| r.item_type == "manga")
            .unwrap()
            .attributes
            .unwrap()
            .tags;
        let base = format!("{}/data/{}", at_home.base_url, at_home.chapter.hash);

        let is_long = tags
            .into_iter()
            .find(|t| t.attributes.name["en"] == "Long Strip")
            .is_some();

        Ok(ChapterImages {
            format: if is_long {
                Format::Long
            } else {
                Format::Normal
            },
            images: at_home
                .chapter
                .data
                .into_iter()
                .map(|file| format!("{base}/{file}"))
                .collect(),
        })
    }
}
