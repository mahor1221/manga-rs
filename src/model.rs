// TODO: enums for genres, authors, ...
// TODO: how to handle url paths

type Str = Box<str>;
type Arr<T> = Box<[T]>;

pub mod comic {
    use super::{Arr, Str};

    #[derive(Debug)]
    pub struct Source {
        pub id: usize,
        pub nsfw: bool,
        pub pirate: bool,
        pub url: &'static str,
        pub name: &'static str,
        pub icon: &'static str,
        pub version: &'static str,
        pub languages: Arr<Lang>,
    }

    pub type Index = Arr<Item>;
    #[derive(Debug)]
    pub struct Item {
        //pub id: usize,
        //pub source_id: usize,
        //pub is_anime: bool,
        pub name: Str,
        pub path_url: Str,
        pub cover_thumbnail_url: Str,
    }

    //pub struct Anime {}
    #[derive(Debug, Default)]
    pub struct Comic {
        pub item_id: usize,
        pub cover_url: Str,
        pub chapters: Arr<Chapter>,

        pub description: Option<Str>,
        pub other_names: Option<Arr<Str>>,
        // filters
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
        pub id: usize,
        pub comic_id: usize,
        pub name: Str,
        //pub image_urls: Arr<Str>,
        //pub image_thumbnail_urls: Option<Arr<Str>>,
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
