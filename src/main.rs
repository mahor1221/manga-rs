// TODO: handle scraper errors
// TODO: tokio single vs multi thread

#![forbid(unsafe_code)]
mod error;
mod model;
use clap::Parser;
use error::Result;
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
    dbg!(TestSource::index(&cli.url)?);
    Ok(())
}

pub trait HasComic {
    fn comic(url: &str) -> Result<Comic>;
}
pub trait HasLatestIndex {
    fn latest(url: &str) -> Result<Index>;
}
pub trait HasPopularIndex {
    fn popular(url: &str) -> Result<Index>;
}

struct TestSource {}
impl TestSource {
    fn index(url: &str) -> Result<Index> {
        let html = std::fs::read_to_string(&url)?;
        let html = Html::parse_document(&html);

        let index = {
            let items = Selector::parse("div.gallery").unwrap();
            let name = Selector::parse("a.cover div.caption").unwrap();
            let path_url = Selector::parse("a").unwrap();
            let cover_thumbnail_url = Selector::parse("a.cover img").unwrap();

            html.select(&items)
                .map(|e| {
                    let name =
                        e.select(&name).next()?.inner_html().into_boxed_str();

                    let path_url = e
                        .select(&path_url)
                        .next()?
                        .value()
                        .attr("href")?
                        .to_owned()
                        .into_boxed_str();

                    let cover_thumbnail_url = e
                        .select(&cover_thumbnail_url)
                        .next()?
                        .value()
                        .attr("data-src")?
                        .to_owned()
                        .into_boxed_str();

                    Some(Item {
                        name,
                        path_url,
                        cover_thumbnail_url,
                    })
                })
                .collect::<Option<Index>>()
                .unwrap()
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

impl HasComic for TestSource {
    fn comic(url: &str) -> Result<Comic> {
        let html = std::fs::read_to_string(&url)?;
        let html = Html::parse_document(&html);

        let cover_url = {
            let s = Selector::parse("#cover img").unwrap();
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
            .unwrap()
        };

        let chapters = { Box::new([Chapter::default()]) };

        let other_names = {
            let select = |s: &Selector| {
                Some(html.select(s).next()?.inner_html().into_boxed_str())
            };
            let en = Selector::parse("#info h1.title span.pretty").unwrap();
            let ja = Selector::parse("#info h2.title span.before").unwrap();
            let en = select(&en).unwrap();
            let ja = select(&ja).unwrap();
            Some(Box::from([en, ja]))
        };

        let (languages, genres, authors, groups, parodies, characters) = {
            let tags = Selector::parse("#info div.tag-container").unwrap();
            let tag_name = Selector::parse("a.tag span.name").unwrap();
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
            cover_url,
            chapters,
            other_names,
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
//impl Popular for TestSource {
//    fn popular(url: &str) -> Result<Index> {
//        let html = std::fs::read_to_string(&url)?;
//        let html = Html::parse_document(&html);
//
//    }
//}
