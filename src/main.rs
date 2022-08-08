//use reqwest;
use scraper::Html;
use scraper::Selector;
use std::error::Error;

type Url<'a> = &'a str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let str = std::fs::read_to_string("./tmp/manga1.html")?;
    let html = Html::parse_document(&str);

    let name = TestSource::name(&html)?;
    let cover = TestSource::cover(&html)?;
    let pages = TestSource::pages(&html)?;

    dbg!(name);
    dbg!(cover);
    dbg!(pages);
    Ok(())
}

// TODO: Is it required to check if returned string is empty?
trait Manga {
    // Using Box<str> becouse scrapper sometimes returns String sometimes &str
    fn name(html: &Html) -> Result<Option<Box<str>>, Box<dyn Error>>;
    fn cover(html: &Html) -> Result<Option<Url>, Box<dyn Error>>;
    fn pages(html: &Html) -> Result<Option<Vec<Url>>, Box<dyn Error>>;
}
trait MangaChapter {
    fn chaps(html: &Html) -> Result<Option<Vec<Url>>, Box<dyn Error>>;
}
trait MangaChapterThumbnail {
    fn chaps_thumb(html: &Html) -> Result<Option<Vec<Url>>, Box<dyn Error>>;
}
trait MangaTag {
    fn tags(html: &Html) -> Result<Option<Vec<Url>>, Box<dyn Error>>;
}

struct TestSource {}
impl Manga for TestSource {
    fn name(html: &Html) -> Result<Option<Box<str>>, Box<dyn Error>> {
        let selector = Selector::parse("#info h1.title span.pretty").unwrap();
        let name = html
            .select(&selector)
            .next()
            .and_then(|e| Some(e.inner_html().into_boxed_str()));
        Ok(name)
    }
    fn cover(html: &Html) -> Result<Option<Url>, Box<dyn Error>> {
        let selector = Selector::parse("#cover img").unwrap();
        let cover = html
            .select(&selector)
            .map(|e| e.value().attr("data-src"))
            .next()
            .flatten();
        Ok(cover)
    }
    fn pages(html: &Html) -> Result<Option<Vec<Url>>, Box<dyn Error>> {
        let selector = Selector::parse("#thumbnail-container img").unwrap();
        let pages = html
            .select(&selector)
            .filter_map(|e| e.value().attr("data-src"))
            .collect::<Vec<_>>();
        // Returns None if Vector is empty
        Ok(Some(pages).filter(|v| !v.is_empty()))
    }
}
impl MangaChapterThumbnail for TestSource {
    fn chaps_thumb(html: &Html) -> Result<Option<Vec<Url>>, Box<dyn Error>> {
        let selector = Selector::parse("#thumbnail-container img").unwrap();
        let pages = html
            .select(&selector)
            .filter_map(|e| e.value().attr("data-src"))
            .collect::<Vec<_>>();
        // Returns None if Vector is empty
        Ok(Some(pages).filter(|v| !v.is_empty()))
    }
}

struct ManganeloTV {}
impl Manga for ManganeloTV {
    fn name(html: &Html) -> Result<Option<Box<str>>, Box<dyn Error>> {
        //let selector = Selector::parse(".story-info-right h1").unwrap();
        let selector = Selector::parse(".story-info-left img").unwrap();
        let name = html
            .select(&selector)
            .map(|e| e.value().attr("title"))
            .next()
            .flatten()
            // Returns None if String is empty
            //.map(|e| Some(e.inner_html()).filter(|e| !e.is_empty()))
            .and_then(|s| Some(s.to_string().into_boxed_str()));
        Ok(name)
    }
    fn cover(html: &Html) -> Result<Option<Url>, Box<dyn Error>> {
        let selector = Selector::parse(".story-info-left img").unwrap();
        let cover = html
            .select(&selector)
            .map(|e| e.value().attr("src"))
            .next()
            .flatten();
        Ok(cover)
    }
    fn pages(html: &Html) -> Result<Option<Vec<Url>>, Box<dyn Error>> {
        let selector =
            Selector::parse(".container-chapter-reader img").unwrap();
        let pages: Vec<Url> = html
            .select(&selector)
            .filter_map(|e| e.value().attr("data-src"))
            .collect();
        Ok(Some(pages).filter(|v| !v.is_empty()))
    }
}
impl MangaChapter for ManganeloTV {
    fn chaps(html: &Html) -> Result<Option<Vec<Url>>, Box<dyn Error>> {
        let selector = Selector::parse(".panel-story-chapter-list a").unwrap();
        let chapters = html
            .select(&selector)
            .filter_map(|e| e.value().attr("href"))
            .collect::<Vec<Url>>();
        // Returns None if Vector is empty
        Ok(Some(chapters).filter(|v| !v.is_empty()))
    }
}
