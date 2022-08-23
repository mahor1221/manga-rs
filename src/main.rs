// TODO: Replace unwraps with proper errors .ok_or(err)?
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

    let comic = TestSource::comic(&cli.url)?;
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
    pub titles: Arr<Lang<Str>>,
    pub cover_url: Str,
    pub languages: Arr<Str>,
    pub chapters: Arr<Chapter>,
    pub status: Option<Status>,
    pub description: Option<Str>,
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
        let h = std::fs::read_to_string("./tmp/page.html")?;
        let h = Html::parse_document(&h);
        let s = Selector::parse("script").unwrap();
        let pages = (|| {
            let v = h
                .select(&s)
                .nth(3)?
                .inner_html()
                .lines()
                .nth(4)?
                .trim_start()
                .strip_prefix("var images_ext = [\"")?
                .strip_suffix("\"];")?
                .split("\",\"")
                .map(|s| s.to_owned().into_boxed_str())
                .collect::<Vec<_>>()
                .into_boxed_slice();
            Some(v)
        })()
        .unwrap();
        dbg!(pages);

        let pages = Box::from([]);
        let chapters = Box::new([Chapter { name: None, pages }]);

        let h = std::fs::read_to_string(url)?;
        let h = Html::parse_document(&h);
        let s = Selector::parse("#cover img").unwrap();
        let cover_url = (|| {
            let v = h
                .select(&s)
                .next()?
                .value()
                .attr("data-src")?
                .to_owned()
                .into_boxed_str();
            Some(v)
        })()
        .unwrap();

        let s = Selector::parse("#info").unwrap();
        let h = h.select(&s).next().unwrap().inner_html();
        let h = Html::parse_fragment(&h);

        let f = |h: &Html, s: &Selector| {
            let v = h
                .select(&s)
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
        let s = Selector::parse("h1.title span.pretty").unwrap();
        let en = f(&h, &s).unwrap();
        let s = Selector::parse("h2.title span.before").unwrap();
        let ja = f(&h, &s).unwrap();
        let titles = Box::from([Lang::English(en), Lang::Japanese(ja)]);

        let s = Selector::parse("a.tag span.name").unwrap();
        let f = |e: ElementRef| {
            let v = e
                .select(&s)
                .map(|e| e.inner_html().into_boxed_str())
                .collect::<Vec<_>>()
                .into_boxed_slice();
            Some(v)
        };
        let s = Selector::parse("#tags div.tag-container").unwrap();
        let languages = h
            .select(&s)
            .find(|e| e.inner_html().contains("Languages"))
            .and_then(f)
            .unwrap_or(Box::from([]));
        let tags = h
            .select(&s)
            .find(|e| e.inner_html().contains("Tags"))
            .and_then(f)
            .or(Some(Box::from([])));
        let authors = h
            .select(&s)
            .find(|e| e.inner_html().contains("Artists"))
            .and_then(f)
            .or(Some(Box::from([])));
        let groups = h
            .select(&s)
            .find(|e| e.inner_html().contains("Groups"))
            .and_then(f)
            .or(Some(Box::from([])));
        let parodies = h
            .select(&s)
            .find(|e| e.inner_html().contains("Parodies"))
            .and_then(f)
            .or(Some(Box::from([])));
        let characters = h
            .select(&s)
            .find(|e| e.inner_html().contains("Characters"))
            .and_then(f)
            .or(Some(Box::from([])));

        Ok(Comic {
            titles,
            cover_url,
            languages,
            chapters,
            tags,
            authors,
            groups,
            parodies,
            characters,
            ..Default::default()
        })
    }
}
