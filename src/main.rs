//use reqwest;
use scraper::Html;
use scraper::Selector;
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
    pub cover_url: Option<Box<str>>,
    pub pages: Box<[Page]>,
}
#[derive(Debug)]
pub struct Comic {
    pub name: Box<str>,
    pub cover_url: Box<str>,
    pub author: Box<str>,
    pub description: Box<str>,
    pub tags: Option<Box<[Box<str>]>>,
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

        let selector = Selector::parse("#thumbnail-container img").unwrap();
        let pages_thumbnail_url = html
            .select(&selector)
            .map(|e| {
                e.value()
                    .attr("data-src")
                    .unwrap()
                    .to_owned()
                    .into_boxed_str()
            })
            .collect::<Vec<_>>();

        //let selector = Selector::parse("#thumbnail-container img").unwrap();
        let pages_image_url = html
            .select(&selector)
            .map(|e| {
                e.value()
                    .attr("data-src")
                    .unwrap()
                    .to_owned()
                    .into_boxed_str()
            })
            .collect::<Vec<_>>();

        // check two vectors lenght
        //let pages = pages_image_url.iter().zip(pages_image_url.iter());

        Ok(Comic {
            name,
            cover_url,
            author: Box::from(""),
            description: Box::from(""),
            tags: None,
            chapters: Box::from([Chapter {
                name: None,
                cover_url: None,
                pages: Box::from([]),
            }]),
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
