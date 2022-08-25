// TODO: handle scraper errors
// TODO: filter parameters
// TODO: lazy evaluation
use clap::Parser;
use scraper::ElementRef;
use scraper::Html;
use scraper::Selector;
mod error;
use error::Result;

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

type Str = Box<str>;
type Arr<T> = Box<[T]>;

#[derive(Debug)]
pub enum Status {
    Canceled,
    Complete,
    Discontinued,
    Hiatus,
    Ongoing,
}

#[derive(Debug)]
pub enum Type {
    Doujinshi,
    Manga,
    Manhua,
    Manhwa,
    Oel,
    OneShot,
}

#[derive(Debug)]
pub enum Lang<T> {
    English(T),
    Romaji(T),
    Japanese(T),
    Chinese(T),
}

#[derive(Debug)]
pub struct Chapter {
    pub url: Str,
    pub name: Str,
    //pub image_urls: Arr<Str>,
    //pub image_thumbnail_urls: Option<Arr<Str>>,
}

#[derive(Debug, Default)]
pub struct Comic {
    pub url: Str,
    pub cover_url: Str,
    pub chapters: Arr<Chapter>,
    pub names: Arr<Lang<Str>>,
    pub description: Option<Str>,
    pub r#type: Option<Type>,
    pub publish_status: Option<Status>,
    pub scan_status: Option<Status>,
    pub languages: Option<Arr<Str>>,
    pub tags: Option<Arr<Str>>,
    pub authors: Option<Arr<Str>>,
    pub groups: Option<Arr<Str>>,
    pub parodies: Option<Arr<Str>>,
    pub characters: Option<Arr<Str>>,
}

trait ComicSource {
    fn comic(html: &str) -> Result<Comic>;
}

struct TestSource {}
impl ComicSource for TestSource {
    fn comic(url: &str) -> Result<Comic> {
        let err = "err";
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
            .ok_or(err)?
        };

        let chapters = {
            let url = format!("{url}/1").into_boxed_str();
            let name = "1".to_owned().into_boxed_str();
            Box::from([Chapter { url, name }])
        };

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
            let en = f(&s1).ok_or(err)?;
            let ja = f(&s2).ok_or(err)?;

            Box::from([Lang::English(en), Lang::Japanese(ja)])
        };

        let (languages, tags, authors, groups, parodies, characters) = {
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

        let url = url.to_owned().into_boxed_str();

        Ok(Comic {
            url,
            cover_url,
            chapters,
            names,
            languages,
            tags,
            authors,
            groups,
            parodies,
            characters,
            ..Default::default()
        })
    }
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
//.ok_or(err)?;
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
