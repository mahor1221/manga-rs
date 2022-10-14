use crate::error::Result;

trait Download {
    fn download() -> Result<()>;
}

#[derive(Debug)]
pub struct Source {
    pub id: SourceId,
    pub url: &'static str, //key ?
    pub name: &'static str,
    pub icon: &'static str,
    pub languages: &'static [Lang],
    pub is_nsfw: bool,
    pub is_pirate: bool,
}

type Str = Box<str>;
type Arr<T> = Box<[T]>;

pub type Index = Arr<Item>;
#[derive(Debug)]
pub struct Item {
    pub source_icon: Str,
    pub source_name: Str,
    //pub id: i64,
    pub url: Str,
    pub cover_thumbnail_url: Str,
    pub name: Str,
    pub item_type: ItemType,
}
impl From<raw::Item> for Item {
    fn from(
        raw::Item {
            source_id,
            url,
            cover_thumbnail_url,
            name,
            item_type,
        }: raw::Item,
    ) -> Self {
        Self {
            url,
            cover_thumbnail_url,
            name,
            item_type,
        }
    }
}
impl Download for Item {
    fn download() -> Result<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Comic {
    pub source_id: i64,
    pub item_id: i64,
    pub cover_url: Str,
    pub names: Arr<Str>,
    pub description: Option<Str>,
    pub chapters: Arr<Chapter>,
    pub last_chapter_num: i64,
    pub last_page_num: i64,
    pub filter: Filter,
}

#[derive(Debug, Default)]
pub struct Filter {
    pub comic_type: Option<ComicType>,
    pub publish_status: Option<Status>,
    pub scan_status: Option<Status>,
    pub languages: Option<Arr<Str>>,
    pub genres: Option<Arr<Str>>,
    pub authors: Option<Arr<Str>>,
    pub groups: Option<Arr<Str>>,
    pub parodies: Option<Arr<Str>>,
    pub characters: Option<Arr<Str>>,
}

#[derive(Debug, Default)]
pub struct Chapter {
    pub comic_id: i64,
    pub id: i64,
    pub name: Str,
    pub pages_count: i64,
}

#[derive(Debug)]
pub enum ItemType {
    Comic,
    Anime,
    Novel,
}

#[derive(Debug)]
pub enum SourceId {
    TestSource = 1,
}

#[derive(Debug)]
pub enum ComicType {
    Doujinshi,
    Manga,
    Manhua,
    Manhwa,
    Oel,
    OneShot,
}

#[derive(Debug)]
pub enum Status {
    Canceled,
    Complete,
    Discontinued,
    Hiatus,
    Ongoing,
}

#[derive(Debug)]
pub enum Lang {
    English,
    Romaji,
    Japanese,
    Chinese,
}

pub mod sql {}

pub mod raw {
    use super::*;

    pub type Index = Arr<Item>;
    #[derive(Debug)]
    pub struct Item {
        pub source_id: SourceId,
        pub url: Str,
        pub cover_thumbnail_url: Str,
        pub name: Str,
        pub item_type: ItemType,
    }

    #[derive(Debug)]
    pub struct Comic {
        pub source_id: i64,
        pub source_icon: Str,
        pub source_name: Str,
        pub cover_url: Str,
        pub names: Arr<Str>,
        pub description: Option<Str>,
        pub chapters: Arr<Chapter>,
        pub filter: Filter,
    }
}

//use chrono::{NaiveDateTime, Utc};
//
//#[derive(Debug, Clone)]
//pub struct Chapter {
//    pub id: i64,
//    pub source_id: i64,
//    pub manga_id: i64,
//    pub title: String,
//    pub path: String,
//    pub number: f64,
//    pub scanlator: String,
//    pub uploaded: NaiveDateTime,
//    pub date_added: NaiveDateTime,
//    pub downloaded_path: Option<String>,
//    pub next: Option<i64>,
//    pub prev: Option<i64>,
//}
//
//impl From<tanoshi_lib::models::ChapterInfo> for Chapter {
//    fn from(ch: tanoshi_lib::models::ChapterInfo) -> Self {
//        Self {
//            id: 0,
//            source_id: ch.source_id,
//            manga_id: 0,
//            title: ch.title,
//            path: ch.path,
//            number: ch.number,
//            scanlator: ch.scanlator.unwrap_or_default(),
//            uploaded: NaiveDateTime::from_timestamp(ch.uploaded, 0),
//            date_added: Utc::now().naive_utc(),
//            downloaded_path: None,
//            next: None,
//            prev: None,
//        }
//    }
//}
