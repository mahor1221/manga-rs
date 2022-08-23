use clap::Parser;
use scraper::ElementRef;
use scraper::Html;
use scraper::Selector;
use std::iter::zip;
mod error;
use error::Result;
// TODO: Replace unwraps with proper errors .ok_or(err)?

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(value_parser)]
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    //let html = reqwest::get(cli.url).await?.text().await?;
    //std::fs::write("tmp/tmp.html", &html)?;

    let html = std::fs::read_to_string(cli.url)?;
    let comic = TestSource::comic(&html)?;
    dbg!(comic);
    Ok(())
}

type Str = Box<str>;
type Arr<T> = Box<[T]>;

#[derive(Debug)]
pub struct Page {
    pub image_url: Str,
    pub thumbnail_url: Option<Str>,
}

#[derive(Debug)]
pub struct Chapter {
    pub name: Option<Str>,
    pub pages: Arr<Page>,
}

#[derive(Debug)]
pub enum Status {
    Completed,
    Ongoing,
}

#[derive(Debug)]
pub enum Lang<T> {
    English(T),
    Romaji(T),
    Japanese(T),
    Chinese(T),
}

#[derive(Debug, Default)]
pub struct Comic {
    //pub url: Str,
    pub title: Arr<Lang<Str>>,
    pub cover_url: Str,
    pub languages: Arr<Str>,
    pub chapters: Arr<Chapter>,
    pub status: Option<Status>,
    pub description: Option<Str>,
    pub tags: Option<Arr<Str>>,
    pub authors: Option<Arr<Str>>,
    pub artists: Option<Arr<Str>>,
    pub groups: Option<Arr<Str>>,
    pub parodies: Option<Arr<Str>>,
    pub characters: Option<Arr<Str>>,
}

trait ComicSource {
    fn comic(html: &str) -> Result<Comic>;
}

struct TestSource {}
impl ComicSource for TestSource {
    fn comic(html: &str) -> Result<Comic> {
        let html = Html::parse_document(&html);

        let s = Selector::parse("#info h1.title span.pretty").unwrap();
        let en = html
            .select(&s)
            .next()
            .unwrap()
            .inner_html()
            .to_owned()
            .into_boxed_str();
        let s = Selector::parse("#info h1.title span.before").unwrap();
        let ja = html
            .select(&s)
            .next()
            .unwrap()
            .inner_html()
            .to_owned()
            .into_boxed_str();
        let title = Box::from([Lang::English(en), Lang::Japanese(ja)]);

        let s = Selector::parse("#cover img").unwrap();
        let cover_url = html
            .select(&s)
            .next()
            .unwrap()
            .value()
            .attr("data-src")
            .unwrap()
            .to_owned()
            .into_boxed_str();

        let s = Selector::parse("#thumbnail-container a").unwrap();
        let pages_image_url = html.select(&s).map(|e| {
            e.value().attr("href").unwrap().to_owned().into_boxed_str()
        });
        let s = Selector::parse("#thumbnail-container img").unwrap();
        let pages_thumbnail_url = html.select(&s).map(|e| {
            e.value()
                .attr("data-src")
                .unwrap()
                .to_owned()
                .into_boxed_str()
        });
        let pages = zip(pages_image_url, pages_thumbnail_url)
            .map(|(image_url, thumbnail_url)| Page {
                image_url,
                thumbnail_url: Some(thumbnail_url),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let chapters = Box::new([Chapter { name: None, pages }]);

        let s = Selector::parse("#tags div.tag-container").unwrap();
        let f = |e: ElementRef| {
            let s = Selector::parse("a.tag span.name").unwrap();
            Some(
                e.select(&s)
                    .map(|e| e.inner_html().into_boxed_str())
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            )
        };

        let languages = html
            .select(&s)
            .find(|e| e.inner_html().contains("Languages"))
            .and_then(f)
            .unwrap_or(Box::from([]));

        let tags = html
            .select(&s)
            .find(|e| e.inner_html().contains("Tags"))
            .and_then(f)
            .or(Some(Box::from([])));

        let artists = html
            .select(&s)
            .find(|e| e.inner_html().contains("Artists"))
            .and_then(f)
            .or(Some(Box::from([])));

        let groups = html
            .select(&s)
            .find(|e| e.inner_html().contains("Groups"))
            .and_then(f)
            .or(Some(Box::from([])));

        let parodies = html
            .select(&s)
            .find(|e| e.inner_html().contains("Parodies"))
            .and_then(f)
            .or(Some(Box::from([])));

        let characters = html
            .select(&s)
            .find(|e| e.inner_html().contains("Characters"))
            .and_then(f)
            .or(Some(Box::from([])));

        Ok(Comic {
            title,
            cover_url,
            languages,
            chapters,
            tags,
            artists,
            groups,
            parodies,
            characters,
            ..Default::default()
        })
    }
}
