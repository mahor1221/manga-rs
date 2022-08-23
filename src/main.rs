use clap::Parser;
use scraper::ElementRef;
use scraper::Html;
use scraper::Selector;
use std::iter::zip;
mod error;
use error::Result;
// TODO: Replace unwraps with proper errors

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

    let html = std::fs::read_to_string("./tmp/tmp.html")?;
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
#[derive(Debug, Default)]
pub struct Comic {
    //pub url: Str,
    pub title: Str,
    pub cover_url: Str,
    pub language: Str,
    pub chapters: Arr<Chapter>,
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

        let selector = Selector::parse("#info h1.title span.pretty").unwrap();
        let title = html
            .select(&selector)
            .next()
            .unwrap()
            .inner_html()
            .split(" | ")
            .next() // Romaji
            //.last() // English
            .unwrap()
            .to_owned()
            .into_boxed_str();

        let selector = Selector::parse("#cover img").unwrap();
        let cover_url = html
            .select(&selector)
            .next()
            .unwrap()
            .value()
            .attr("data-src")
            .unwrap()
            .to_owned()
            .into_boxed_str();

        let selector = Selector::parse("#thumbnail-container a").unwrap();
        let pages_image_url = html.select(&selector).map(|e| {
            e.value().attr("href").unwrap().to_owned().into_boxed_str()
        });
        let selector = Selector::parse("#thumbnail-container img").unwrap();
        let pages_thumbnail_url = html.select(&selector).map(|e| {
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

        let selector = Selector::parse("#tags div.tag-container").unwrap();
        let selector2 = Selector::parse("a.tag span.name").unwrap();
        let f = |e: ElementRef| {
            Some(
                e.select(&selector2)
                    .map(|e| e.inner_html().into_boxed_str())
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            )
        };

        let language = html
            .select(&selector)
            .find(|e| e.inner_html().contains("Languages"))
            .unwrap()
            .select(&selector2)
            .last()
            .unwrap()
            .inner_html()
            .into_boxed_str();

        let tags = html
            .select(&selector)
            .find(|e| e.inner_html().contains("Tags"))
            .and_then(f);

        let artists = html
            .select(&selector)
            .find(|e| e.inner_html().contains("Artists"))
            .and_then(f);

        let groups = html
            .select(&selector)
            .find(|e| e.inner_html().contains("Groups"))
            .and_then(f);

        let parodies = html
            .select(&selector)
            .find(|e| e.inner_html().contains("Parodies"))
            .and_then(f);

        let characters = html
            .select(&selector)
            .find(|e| e.inner_html().contains("Characters"))
            .and_then(f);

        Ok(Comic {
            title,
            cover_url,
            language,
            chapters,
            tags: tags,
            artists: Some(artists),
            groups: Some(groups),
            parodies: Some(parodies),
            characters: Some(characters),
            ..Default::default()
        })
    }
}
