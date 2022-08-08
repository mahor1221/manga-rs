//use reqwest;
use scraper::Html;
use scraper::Selector;
mod error;
use error::Result;
type Url<'a> = &'a str;

#[tokio::main]
async fn main() -> Result<'static, ()> {
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

// TODO: check attr if returns None

// Should be used to return None if String or &str is empty.
// Some(String).filter(|s| !s.is_empty())
// It's the same for Vec
// Some(Vec<_>).filter(|v| !v.is_empty())

trait Manga {
    // Using Box<str> becouse scraper sometimes returns String sometimes &str
    fn name(html: &Html) -> Result<Option<Box<str>>>;
    fn cover(html: &Html) -> Result<Option<Url>>;
    fn pages(html: &Html) -> Result<Option<Vec<Url>>>;
}
trait MangaChapter {
    fn chapters(html: &Html) -> Result<Option<Vec<Url>>>;
}
trait MangaChapterThumbnail {
    fn chapters_thumb(html: &Html) -> Result<Option<Vec<Url>>>;
}
trait MangaTag {
    fn tags(html: &Html) -> Result<Option<Vec<Url>>>;
}

struct TestSource {}
impl Manga for TestSource {
    fn name(html: &Html) -> Result<Option<Box<str>>> {
        let selector = Selector::parse("#info h1.title span.pretty").unwrap();
        let value = html
            .select(&selector)
            .next()
            .and_then(|e| Some(e.inner_html().into_boxed_str()))
            .filter(|s| !s.is_empty());
        Ok(value)
    }
    fn cover(html: &Html) -> Result<Option<Url>> {
        let selector = Selector::parse("#cover img")?;
        let value = html
            .select(&selector)
            .next()
            .and_then(|e| e.value().attr("value-src"));
        Ok(value)
    }
    fn pages(html: &Html) -> Result<Option<Vec<Url>>> {
        let selector = Selector::parse("#thumbnail-container img")?;
        let value = html
            .select(&selector)
            .filter_map(|e| e.value().attr("value-src"))
            .collect::<Vec<_>>();
        let value = Some(value).filter(|v| !v.is_empty());
        Ok(value)
    }
}
impl MangaChapterThumbnail for TestSource {
    fn chapters_thumb(html: &Html) -> Result<Option<Vec<Url>>> {
        let selector = Selector::parse("#thumbnail-container img")?;
        let value = html
            .select(&selector)
            .filter_map(|e| e.value().attr("value-src"))
            .collect::<Vec<_>>();
        let value = Some(value).filter(|v| !v.is_empty());
        Ok(value)
    }
}

struct ManganeloTV {}
impl Manga for ManganeloTV {
    fn name(html: &Html) -> Result<Option<Box<str>>> {
        let selector = Selector::parse(".story-info-left img")?;
        let value = html
            .select(&selector)
            .next()
            .and_then(|e| e.value().attr("title"))
            .and_then(|s| Some(s.to_string().into_boxed_str()));
        Ok(value)
    }
    fn cover(html: &Html) -> Result<Option<Url>> {
        let selector = Selector::parse(".story-info-left img")?;
        let value = html
            .select(&selector)
            .next()
            .and_then(|e| e.value().attr("src"));
        Ok(value)
    }
    fn pages(html: &Html) -> Result<Option<Vec<Url>>> {
        let selector = Selector::parse(".container-chapter-reader img")?;
        let value = html
            .select(&selector)
            .filter_map(|e| e.value().attr("value-src"))
            .collect::<Vec<_>>();
        let value = Some(value).filter(|v| !v.is_empty());
        Ok(value)
    }
}
impl MangaChapter for ManganeloTV {
    fn chapters(html: &Html) -> Result<Option<Vec<Url>>> {
        let selector = Selector::parse(".panel-story-chapter-list a")?;
        let value = html
            .select(&selector)
            .filter_map(|e| e.value().attr("href"))
            .collect::<Vec<_>>();
        let value = Some(value).filter(|v| !v.is_empty());
        Ok(value)
    }
}
