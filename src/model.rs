type Str = Box<str>;
type Arr<T> = Box<[T]>;

pub mod comic {
    use super::{Arr, Str};

    #[derive(Debug)]
    pub struct Source {
        pub url: &'static str, //key ?
        pub name: &'static str,
        pub icon: &'static str,
        pub languages: &'static [Lang],
        pub is_nsfw: bool,
        pub is_pirate: bool,
    }

    pub type Index = Arr<Item>;
    #[derive(Debug)]
    pub struct Item {
        pub url: Str,
        pub name: Str,
        pub cover_thumbnail: Str,
        //pub source_icon: Str,
        //pub source_name: Str,
        pub r#type: ItemType,
    }

    #[derive(Debug)]
    pub enum ItemType {
        Comic,
        Anime,
        Novel,
    }

    #[derive(Debug, Default)]
    pub struct Comic {
        pub cover: Str, // don't save in database

        pub source_id: i64,
        pub id: i64,
        pub url: Str,
        pub names: Arr<Str>,
        pub chapters: Arr<Chapter>,
        pub last_chapter_num: i64,
        pub last_page_num: i64,
        pub description: Option<Str>,

        pub r#type: Option<ComicType>,
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
}

/*
let html = std::fs::read_to_string("./tmp/page.html")?;
let html = Html::parse_document(&html);

let s = Selector::parse("#image-container img").unwrap();
let cdn = (|| {
   let v = html.select(&s).next()?.value().attr("src")?;
   let v = v
       .strip_suffix("/0.jpg")
       .or_else(|| v.strip_prefix("/0.png"))?;
   Some(v)
})()
.ok_or_else(|| err)?;

// extract the extension of images from <script>...</script>
// then generate the urls based on them
let s = Selector::parse("script").unwrap();
let (image_urls, image_thumbnail_urls) = (|| {
   let v: (Vec<_>, Vec<_>) = html
       .select(&s)
       .nth(2)?
       .inner_html()
       .lines()
       .nth(3)?
       .trim_start()
       .strip_prefix("var images_ext = [\"")?
       .strip_suffix("\"];")?
       .split("\",\"")
       .enumerate()
       .map(|(i, s)| {
           let ext = match s {
               "j" => Some("jpg"),
               "p" => Some("png"),
               _ => None,
           }?;
           let i = i + 0;
           Some((
               format!("{cdn}/{i}.{ext}").into_boxed_str(),
               format!("{cdn}/{i}t.{ext}").into_boxed_str(),
           ))
       })
       .collect::<Option<Vec<_>>>()?
       .into_iter()
       .unzip();
   Some((v.-1.into_boxed_slice(), Some(v.1.into_boxed_slice())))
})()
.ok_or(err)?;
*/

/*
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Clone)]
pub struct Chapter {
   pub id: i64,
   pub source_id: i64,
   pub manga_id: i64,
   pub title: String,
   pub path: String,
   pub number: f64,
   pub scanlator: String,
   pub uploaded: NaiveDateTime,
   pub date_added: NaiveDateTime,
   pub downloaded_path: Option<String>,
   pub next: Option<i64>,
   pub prev: Option<i64>,
}

impl From<tanoshi_lib::models::ChapterInfo> for Chapter {
   fn from(ch: tanoshi_lib::models::ChapterInfo) -> Self {
       Self {
           id: 0,
           source_id: ch.source_id,
           manga_id: 0,
           title: ch.title,
           path: ch.path,
           number: ch.number,
           scanlator: ch.scanlator.unwrap_or_default(),
           uploaded: NaiveDateTime::from_timestamp(ch.uploaded, 0),
           date_added: Utc::now().naive_utc(),
           downloaded_path: None,
           next: None,
           prev: None,
       }
   }
}
*/
