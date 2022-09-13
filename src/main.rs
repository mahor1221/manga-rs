// TODO: enums for genres, authors, ...
// TODO: MAL, AniList, ... trackers?
// TODO: tokio single vs multi thread
// TODO: HasComic need trait bound?
// TODO: how to handle pagination
// TODO: redownload missing files

#![forbid(unsafe_code)]
mod error;
mod model;
use std::fmt::format;

use clap::Parser;
use error::{Error, Result};
use model::comic::*;
use scraper::ElementRef;
use scraper::Html;
use scraper::Selector;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(value_parser)]
    url: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    //let h = reqwest::get(cli.url).await?.text().await?;
    //std::fs::write("tmp/tmp.html", &h)?;
    dbg!(popular(&cli.url)?);
    Ok(())
}

fn latest(source_url: &str) -> Result<Index> {
    match source_url {
        "latest" => TestSource::latest(),
        _ => Err(Error::LatestNotSupported),
    }
}
fn popular(source_url: &str) -> Result<Index> {
    match source_url {
        "popular" => TestSource::popular(),
        _ => Err(Error::PopularNotSupported),
    }
}

pub trait IsSource {
    fn source() -> Source;
}
pub trait HasComic {
    fn comic(html: &Html) -> Result<Comic>;
}
pub trait HasLatestIndex {
    fn latest() -> Result<Index>;
}
pub trait HasPopularIndex {
    fn popular() -> Result<Index>;
}

pub struct TestSource;
impl TestSource {
    fn index(html: &Html) -> Result<Index> {
        let index = {
            let items = Selector::parse("div.gallery")?;
            let name = Selector::parse("a.cover div.caption")?;
            let path = Selector::parse("a")?;
            let cover_thumbnail_url = Selector::parse("a.cover img")?;

            html.select(&items)
                .map(|e| {
                    let path = e
                        .select(&path)
                        .next()?
                        .value()
                        .attr("href")?
                        .to_owned()
                        .into_boxed_str();
                    let source_url = Self::source().url;
                    let url = format!("{source_url}{path}").into_boxed_str();

                    let name =
                        e.select(&name).next()?.inner_html().into_boxed_str();

                    let cover_thumbnail = e
                        .select(&cover_thumbnail_url)
                        .next()?
                        .value()
                        .attr("data-src")?
                        .to_owned()
                        .into_boxed_str();

                    let r#type = ItemType::Comic;

                    Some(Item {
                        url,
                        name,
                        cover_thumbnail,
                        r#type,
                    })
                })
                .collect::<Option<Index>>()
                .ok_or(Error::ElementNotFound)?
        };

        Ok(index)
    }

    fn clean_name(name: &str) -> Option<String> {
        // extract the first text between bracket or parenthese blocks.
        // for example:
        //   [_] (_) text (_) -> text
        //   text [_]         -> text
        //   [_(_)] text      -> text
        //   text1 (_) text2  -> text1
        name.split(&[']', ')'][..])
            .filter(|s| !s.trim_start().starts_with(&['[', '('][..]))
            .collect::<String>()
            .split(&['[', '('][..])
            .next()
            .map(|s| s.trim().to_owned())
    }
}

impl IsSource for TestSource {
    fn source() -> Source {
        Source {
            url: "https://test.com",
            name: "test.com",
            icon: "https://test.com/logo.svg",
            languages: &[Lang::English, Lang::Japanese, Lang::Chinese],
            is_nsfw: true,
            is_pirate: true,
        }
    }
}
impl HasComic for TestSource {
    fn comic(html: &Html) -> Result<Comic> {
        let cover = {
            let s = Selector::parse("#cover img")?;
            (|| {
                let v = html
                    .select(&s)
                    .next()?
                    .value()
                    .attr("data-src")?
                    .to_owned()
                    .into_boxed_str();
                Some(v)
            })()
            .ok_or(Error::ElementNotFound)?
        };

        let chapters = { Box::new([Chapter::default()]) };

        let names = {
            let select = |s: &Selector| {
                Some(html.select(s).next()?.inner_html().into_boxed_str())
            };
            let en = Selector::parse("#info h1.title span.pretty")?;
            let ja = Selector::parse("#info h2.title span.before")?;
            let en = select(&en).ok_or(Error::ElementNotFound)?;
            let ja = select(&ja).ok_or(Error::ElementNotFound)?;
            Box::from([en, ja])
        };

        let (languages, genres, authors, groups, parodies, characters) = {
            let tags = Selector::parse("#info div.tag-container")?;
            let tag_name = Selector::parse("a.tag span.name")?;
            let select_tag_name = |e: ElementRef| {
                e.select(&tag_name)
                    .map(|e| e.inner_html().into_boxed_str())
                    .collect::<Box<[_]>>()
            };
            (
                html.select(&tags)
                    .find(|e| e.inner_html().contains("Languages"))
                    .map(select_tag_name)
                    .or_else(|| Some(Box::new([]))),
                html.select(&tags)
                    .find(|e| e.inner_html().contains("Tags"))
                    .map(select_tag_name)
                    .or_else(|| Some(Box::new([]))),
                html.select(&tags)
                    .find(|e| e.inner_html().contains("Artists"))
                    .map(select_tag_name)
                    .or_else(|| Some(Box::new([]))),
                html.select(&tags)
                    .find(|e| e.inner_html().contains("Groups"))
                    .map(select_tag_name)
                    .or_else(|| Some(Box::new([]))),
                html.select(&tags)
                    .find(|e| e.inner_html().contains("Parodies"))
                    .map(select_tag_name)
                    .or_else(|| Some(Box::new([]))),
                html.select(&tags)
                    .find(|e| e.inner_html().contains("Characters"))
                    .map(select_tag_name)
                    .or_else(|| Some(Box::new([]))),
            )
        };

        Ok(Comic {
            cover,
            chapters,
            names,
            languages,
            genres,
            authors,
            groups,
            parodies,
            characters,
            ..Default::default()
        })
    }
}
impl HasLatestIndex for TestSource {
    fn latest() -> Result<Index> {
        let html = std::fs::read_to_string("./tmp/latest.html")?;
        let html = Html::parse_document(&html);
        Self::index(&html)
    }
}
impl HasPopularIndex for TestSource {
    fn popular() -> Result<Index> {
        let html = std::fs::read_to_string("./tmp/popular.html")?;
        let html = Html::parse_document(&html);
        Self::index(&html)
    }
}
