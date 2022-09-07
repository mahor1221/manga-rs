// TODO: handle scraper errors
// TODO: filter parameters
// TODO: lazy evaluation

use clap::Parser;
use scraper::ElementRef;
use scraper::Html;
use scraper::Selector;
mod error;
use error::Result;
mod model;
use model::comic::*;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(value_parser)]
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    //let h = reqwest::get(cli.url).await?.text().await?;
    //std::fs::write("tmp/tmp.html", &html)?;
    let _comic = TestSource::comic(&cli.url)?;
    dbg!(_comic);

    Ok(())
}

pub trait ComicSource {
    fn comic(url: &str) -> Result<Comic>;
    //fn details() -> Result<Details>;
}

struct TestSource {}
impl TestSource {}
impl ComicSource for TestSource {
    fn comic(url: &str) -> Result<Comic> {
        let html = std::fs::read_to_string(&url)?;
        let html = Html::parse_document(&html);

        let cover_thumbnail_url = "".to_owned().into_boxed_str();
        let name = "".to_owned().into_boxed_str();
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

        let chapters = Box::new([Chapter::default()]);

        let names = {
            let s1 = Selector::parse("#info h1.title span.pretty").unwrap();
            let s2 = Selector::parse("#info h2.title span.before").unwrap();
            let f = |s: &Selector| {
                let v = html
                    .select(s)
                    .next()?
                    .inner_html()
                    // extract the first text between bracket or parenthese blocks.
                    // for example:
                    //     [_] (_) text (_) -> text
                    //     text [_]         -> text
                    //     [_(_)] text      -> text
                    //     text1 (_) text2  -> text1
                    .split(&[']', ')'][..])
                    .filter(|s| !s.trim_start().starts_with(&['[', '('][..]))
                    .collect::<String>()
                    .split(&['[', '('][..])
                    .next()?
                    .trim()
                    .to_owned()
                    .into_boxed_str();
                Some(v)
            };
            let en = f(&s1).unwrap();
            let ja = f(&s2).unwrap();
            //Box::from([Lang::English(en), Lang::Japanese(ja)])
            Box::from([en, ja])
        };

        let (languages, genres, authors, groups, parodies, characters) = {
            let s1 = Selector::parse("#info div.tag-container").unwrap();
            let s2 = Selector::parse("a.tag span.name").unwrap();
            let f = |e: ElementRef| {
                let v = e
                    .select(&s2)
                    .map(|e| e.inner_html().into_boxed_str())
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                Some(v)
            };
            (
                html.select(&s1)
                    .find(|e| e.inner_html().contains("Languages"))
                    .and_then(f)
                    .or_else(|| Some(Box::from([]))),
                html.select(&s1)
                    .find(|e| e.inner_html().contains("Tags"))
                    .and_then(f)
                    .or_else(|| Some(Box::from([]))),
                html.select(&s1)
                    .find(|e| e.inner_html().contains("Artists"))
                    .and_then(f)
                    .or_else(|| Some(Box::from([]))),
                html.select(&s1)
                    .find(|e| e.inner_html().contains("Groups"))
                    .and_then(f)
                    .or_else(|| Some(Box::from([]))),
                html.select(&s1)
                    .find(|e| e.inner_html().contains("Parodies"))
                    .and_then(f)
                    .or_else(|| Some(Box::from([]))),
                html.select(&s1)
                    .find(|e| e.inner_html().contains("Characters"))
                    .and_then(f)
                    .or_else(|| Some(Box::from([]))),
            )
        };

        Ok(Comic {
            source_id: 0,
            name,
            cover_thumbnail_url,
            names,
            cover_url,
            chapters,
            languages,
            genres,
            authors,
            groups,
            parodies,
            characters,
            ..Default::default()
        })
    }
    //fn details() -> Result<Details> {}
}

//let html = std::fs::read_to_string("./tmp/page.html")?;
//let html = Html::parse_document(&html);
//
//let s = Selector::parse("#image-container img").unwrap();
//let cdn = (|| {
//    let v = html.select(&s).next()?.value().attr("src")?;
//    let v = v
//        .strip_suffix("/1.jpg")
//        .or_else(|| v.strip_prefix("/1.png"))?;
//    Some(v)
//})()
//.ok_or_else(|| err)?;
//
//// extract the extension of images from <script>...</script>
//// then generate the urls based on them
//let s = Selector::parse("script").unwrap();
//let (image_urls, image_thumbnail_urls) = (|| {
//    let v: (Vec<_>, Vec<_>) = html
//        .select(&s)
//        .nth(3)?
//        .inner_html()
//        .lines()
//        .nth(4)?
//        .trim_start()
//        .strip_prefix("var images_ext = [\"")?
//        .strip_suffix("\"];")?
//        .split("\",\"")
//        .enumerate()
//        .map(|(i, s)| {
//            let ext = match s {
//                "j" => Some("jpg"),
//                "p" => Some("png"),
//                _ => None,
//            }?;
//            let i = i + 1;
//            Some((
//                format!("{cdn}/{i}.{ext}").into_boxed_str(),
//                format!("{cdn}/{i}t.{ext}").into_boxed_str(),
//            ))
//        })
//        .collect::<Option<Vec<_>>>()?
//        .into_iter()
//        .unzip();
//    Some((v.0.into_boxed_slice(), Some(v.1.into_boxed_slice())))
//})()
//.ok_or(err)?;
