use scraper::Html;
use scraper::Selector;
use std::iter::zip;
//use std::process::Command;
mod error;
use error::Result;
// TODO: Replace unwraps with proper errors

#[tokio::main]
async fn main() -> Result<()> {
    //Command::new("sh").args(["-c", "chromedriver"]).output()?;
    const WEBSITE: &'static str = "https://www.useragentstring.com/pages/Browserlist";

    //let mut bypasser =
    //    cloudflare_bypasser::Bypasser::default().retry(3).wait(5);
    //.random_user_agent(true)
    //
    //let (cookie, user_agent);
    //loop {
    //    if let Ok((c, ua)) = bypasser.bypass(WEBSITE) {
    //        cookie = c;
    //        user_agent = ua;
    //        break;
    //    }
    //}
    //
    //let headers = {
    //    let mut h = reqwest::header::HeaderMap::new();
    //    h.insert(reqwest::header::COOKIE, cookie);
    //    h.insert(reqwest::header::USER_AGENT, user_agent);
    //    h
    //};
    //let client = reqwest::ClientBuilder::new()
    //    .default_headers(headers)
    //    .build()?;
    //
    //let html = client.get(WEBSITE).send().await?.text().await?;
    //dbg!(&html);
    //std::fs::write("tmp/temp.html", &html)?;

    //let html = std::fs::read_to_string("./tmp/manga1.html")?;
    //let comic = TestSource::comic(&html)?;
    //dbg!(comic);
    Ok(())
}

#[derive(Debug)]
pub struct Page {
    pub image_url: Box<str>,
    pub thumbnail_url: Option<Box<str>>,
}
#[derive(Debug)]
pub struct Chapter {
    pub name: Option<Box<str>>,
    pub pages: Box<[Page]>,
}
#[derive(Debug, Default)]
pub struct Comic {
    //pub url: Box<str>,
    pub title: Box<str>,
    pub cover_url: Box<str>,
    pub language: Box<str>,
    pub chapters: Box<[Chapter]>,
    pub description: Option<Box<str>>,
    pub tags: Option<Box<[Box<str>]>>,
    pub authors: Option<Box<[Box<str>]>>,
    pub artists: Option<Box<[Box<str>]>>,
    pub groups: Option<Box<[Box<str>]>>,
    pub parodies: Option<Box<[Box<str>]>>,
    pub characters: Option<Box<[Box<str>]>>,
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
            .unwrap()
            .select(&selector2)
            .map(|e| e.inner_html().into_boxed_str())
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let artists = html
            .select(&selector)
            .find(|e| e.inner_html().contains("Artists"))
            .unwrap()
            .select(&selector2)
            .map(|e| e.inner_html().into_boxed_str())
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let groups = html
            .select(&selector)
            .find(|e| e.inner_html().contains("Groups"))
            .unwrap()
            .select(&selector2)
            .map(|e| e.inner_html().into_boxed_str())
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let parodies = html
            .select(&selector)
            .find(|e| e.inner_html().contains("Parodies"))
            .unwrap()
            .select(&selector2)
            .map(|e| e.inner_html().into_boxed_str())
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let characters = html
            .select(&selector)
            .find(|e| e.inner_html().contains("Characters"))
            .unwrap()
            .select(&selector2)
            .map(|e| e.inner_html().into_boxed_str())
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Ok(Comic {
            title,
            cover_url,
            language,
            chapters,
            tags: Some(tags),
            artists: Some(artists),
            groups: Some(groups),
            parodies: Some(parodies),
            characters: Some(characters),
            ..Default::default()
        })
    }
}
