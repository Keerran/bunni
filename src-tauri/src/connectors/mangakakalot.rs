use super::{Chapter, ChapterImages, Connector, Format, Manga, SearchItem};
use async_trait::async_trait;
use base64::{engine::general_purpose, Engine};
use futures::{future::join_all, FutureExt};
use reqwest::header::HeaderValue;
use scraper::{Html, Selector};

const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:71.0) Gecko/20100101 Firefox/77.0";

#[derive(Clone)]
pub struct MangaKakalot {
    client: reqwest::Client,
}

fn change_alias(alias: &str) -> String {
    let mut str = String::from(alias);

    str = str.to_lowercase();
    str = str.replace("à|á|ạ|ả|ã|â|ầ|ấ|ậ|ẩ|ẫ|ă|ằ|ắ|ặ|ẳ|ẵ", "a");
    str = str.replace("è|é|ẹ|ẻ|ẽ|ê|ề|ế|ệ|ể|ễ", "e");
    str = str.replace("ì|í|ị|ỉ|ĩ", "i");
    str = str.replace("ò|ó|ọ|ỏ|õ|ô|ồ|ố|ộ|ổ|ỗ|ơ|ờ|ớ|ợ|ở|ỡ", "o");
    str = str.replace("ù|ú|ụ|ủ|ũ|ư|ừ|ứ|ự|ử|ữ", "u");
    str = str.replace("ỳ|ý|ỵ|ỷ|ỹ", "y");
    str = str.replace("đ", "d");
    str = str.replace("!|@|%|\\^|\\*|\\(|\\)|\\+|\\=|\\<|\\>|\\?|\\/|,|\\.|\\:|\\;|\\'| |\"|\\&|\\#|\\[|\\]|~|-|$|_", "_");
    str = str.replace("_+_", "_");
    str = str.trim_matches('_').to_string();

    str
}

impl MangaKakalot {
    pub fn new(client: reqwest::Client) -> MangaKakalot {
        MangaKakalot { client }
    }

    fn headers() -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            HeaderValue::from_static(USER_AGENT),
        );
        headers.insert(
            reqwest::header::REFERER,
            HeaderValue::from_static("https://mangakakalot.com/"),
        );
        headers
    }

    pub async fn get_text(&self, url: impl Into<String>) -> reqwest::Result<String> {
        self.client
            .get(url.into())
            .headers(Self::headers())
            .send()
            .await?
            .text()
            .await
    }

    pub async fn fetch_cover_img(&self, url: String) -> Result<String, reqwest::Error> {
        let req = self.client.get(url).headers(Self::headers()).send().await?;

        let content_type = req
            .headers()
            .get("content-type")
            .expect("content-type header not found")
            .to_str()
            .unwrap()
            .to_string();
        let bytes = general_purpose::STANDARD.encode(req.bytes().await?);

        Ok(format!("data:{content_type};base64,{bytes}"))
    }
}

#[async_trait]
impl Connector for MangaKakalot {
    fn name(&self) -> &str {
        "MangaKakalot"
    }

    fn url(&self) -> &str {
        "https://mangakakalot.com"
    }

    async fn search(&self, query: &str) -> Result<Vec<SearchItem>, reqwest::Error> {
        let url = if query == "" {
            format!("{}/manga_list", self.url())
        } else {
            format!("{}/search/story/{}", self.url(), change_alias(query))
        };

        let page = self.get_text(url).await?;
        let doc = Html::parse_document(&page);
        let mangas =
            Selector::parse(".panel_story_list .story_item, div.list-truyen-item-wrap").unwrap();
        let els = doc.select(&mangas);

        Ok(els
            .map(|el| {
                let url = el
                    .select(&Selector::parse("h3 a").unwrap())
                    .next()
                    .expect("no url element");
                SearchItem {
                    id: url
                        .value()
                        .attr("href")
                        .unwrap()
                        .to_string()
                        .replace("/", " "),
                    title: url.text().collect(),
                    description: String::from(""),
                    cover_url: el
                        .select(&Selector::parse("img").unwrap())
                        .next()
                        .expect("no cover element")
                        .value()
                        .attr("src")
                        .unwrap()
                        .to_string(),
                }
            })
            .collect())
    }

    async fn fetch_manga(&self, id: &str) -> Result<Manga, reqwest::Error> {
        let url = id.replace(" ", "/");
        let page = self.get_text(url).await?;

        let doc = Html::parse_document(&page);

        let main = Selector::parse("div.manga-info-top, div.panel-story-info").unwrap();
        let chapters =
            Selector::parse("div.chapter-list div.row, ul.row-content-chapter li").unwrap();

        let el = doc.select(&main).next().unwrap();
        let chapters = doc.select(&chapters);

        Ok(Manga {
            desc: SearchItem {
                id: id.to_string(),
                title: el
                    .select(&Selector::parse("h1, h2").unwrap())
                    .next()
                    .unwrap()
                    .text()
                    .collect(),
                description: el
                    .select(
                        &Selector::parse("div#noidungm, div#panel-story-info-description").unwrap(),
                    )
                    .next()
                    .map(|e| e.text().collect())
                    .unwrap_or_default(),
                cover_url: el
                    .select(
                        &Selector::parse("div.manga-info-pic img, span.info-image img").unwrap(),
                    )
                    .next()
                    .unwrap()
                    .attr("src")
                    .unwrap()
                    .to_string(),
            },
            chapters: chapters
                .enumerate()
                .map(|(i, el)| {
                    let anch = el.select(&Selector::parse("a").unwrap()).next().unwrap();
                    Chapter {
                        id: anch.attr("href").unwrap().replace("/", " "),
                        name: anch.text().collect(),
                        number: i as f32,
                    }
                })
                .collect(),
        })
    }

    async fn fetch_chapter(&self, id: &str) -> Result<ChapterImages, reqwest::Error> {
        // let url = format!("{}/{}", self.url(), id.replace(" ", "/"));
        let url = id.replace(" ", "/");

        let page = self.get_text(url).await?;
        let srcs = {
            let doc = Html::parse_document(&page);
            let selector =
                Selector::parse("div#vungdoc img, div.container-chapter-reader img").unwrap();
            let els = doc.select(&selector);
            els.map(|el| el.value().attr("src").unwrap_or_default().to_string())
                .filter(|src| !src.ends_with("log"))
                .collect::<Vec<_>>()
        };

        Ok(ChapterImages {
            images: join_all(srcs.into_iter().map(|src| {
                let modified_url = if src.starts_with("https://convert_image_digi.mgicdn.com") {
                    format!(
                        "https://images.weserv.nl/?url={}",
                        src.split("//").nth(1).unwrap_or_default()
                    )
                } else {
                    src.to_string()
                };
                self.fetch_cover_img(modified_url)
            }))
            .await
            .into_iter()
            .collect::<reqwest::Result<Vec<_>>>()?,
            format: Format::Normal,
        })
    }
}
