//use reqwest;
use scraper::Html;
use scraper::Selector;
use std::iter::zip;
mod error;
use error::Result;
// TODO: Replace unwraps with proper errors

fn main() -> Result<()> {
    let html = std::fs::read_to_string("./tmp/manga1.html")?;
    let comic = TestSource::comic(&html)?;

    dbg!(comic);
    Ok(())
}

#[derive(Debug)]
pub struct Page {
    pub image_url: Box<str>,
    pub thumbnail_url: Box<str>,
}
#[derive(Debug)]
pub struct Chapter {
    pub name: Option<Box<str>>,
    pub pages: Box<[Page]>,
}
#[derive(Debug)]
pub struct Comic {
    pub name: Box<str>,
    pub description: Option<Box<str>>,
    pub language: Box<str>,
    pub authors: Box<[Box<str>]>,
    pub tags: Option<Box<[Box<str>]>>,
    pub cover_url: Box<str>,
    pub chapters: Box<[Chapter]>,
}

trait ComicSource {
    fn comic(html: &str) -> Result<Comic>;
}

struct TestSource {}
impl ComicSource for TestSource {
    fn comic(html: &str) -> Result<Comic> {
        let html = Html::parse_document(&html);

        let selector = Selector::parse("#info h1.title span.pretty").unwrap();
        let name = html
            .select(&selector)
            .next()
            .unwrap()
            .inner_html()
            .into_boxed_str();

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

        let selector = Selector::parse("#tags div.tag-container").unwrap();
        let selector2 = Selector::parse("a.tag span.name").unwrap();
        let authors = html
            .select(&selector)
            .find(|e| e.inner_html().contains("Artists"))
            .unwrap()
            .select(&selector2)
            .map(|e| e.inner_html().into_boxed_str())
            .collect::<Vec<_>>()
            .into_boxed_slice();

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
        //assert_eq!(pages_thumbnail_url.len(), pages_image_url.len());
        let pages = zip(pages_image_url, pages_thumbnail_url)
            .map(|(image_url, thumbnail_url)| Page {
                image_url,
                thumbnail_url,
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let chapters = Box::from([Chapter { name: None, pages }]);

        Ok(Comic {
            name,
            description: None,
            language,
            authors,
            tags: None,
            cover_url,
            chapters,
        })
    }
}

//struct ManganeloTV {}
//impl Manga for ManganeloTV {
//    fn name(html: &Html) -> Result<Box<str>> {
//        let selector = Selector::parse(".story-info-left img")?;
//        let value = html
//            .select(&selector)
//            .next()
//            .and_then(|e| e.value().attr("title"))
//            .and_then(|s| Some(s.to_string().into_boxed_str()))
//            .filter(|s| !s.is_empty());
//        Ok(value)
//    }
//    fn cover(html: &Html) -> Result<Url> {
//        let selector = Selector::parse(".story-info-left img")?;
//        let value = html
//            .select(&selector)
//            .next()
//            .and_then(|e| e.value().attr("src"))
//            .filter(|s| !s.is_empty());
//        Ok(value)
//    }
//    fn pages(html: &Html) -> Result<Vec<Url>> {
//        let selector = Selector::parse(".container-chapter-reader img")?;
//        let value = html
//            .select(&selector)
//            .filter_map(|e| e.value().attr("data-src").filter(|s| !s.is_empty()))
//            .collect::<Vec<_>>();
//        let value = Some(value).filter(|v| !v.is_empty());
//        Ok(value)
//    }
//}
//impl MangaChapter for ManganeloTV {
//    fn chapters(html: &Html) -> Result<Vec<Url>> {
//        let selector = Selector::parse(".panel-story-chapter-list a")?;
//        let value = html
//            .select(&selector)
//            .filter_map(|e| e.value().attr("href").filter(|s| !s.is_empty()))
//            .collect::<Vec<_>>();
//        let value = Some(value).filter(|v| !v.is_empty());
//        Ok(value)
//    }
//}
