//use reqwest;
use scraper::Html;
use scraper::Selector;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manga = std::fs::read_to_string("./tmp/manga1.html")?;
    let manga = Html::parse_document(&manga);

    let name = Manga::name(&manga)?;
    //let cover = Manga::cover(&manga)?;
    //let chapters = Manga::chapters(&manga)?;
    //let pages = Manga::pages(&chapter)?;

    dbg!(name);
    //dbg!(cover);
    //dbg!(chapters);
    //dbg!(pages);
    Ok(())
}

trait Extension {
    fn cover(manga: &Html) -> Result<Option<&str>, Box<dyn Error>>;
    fn name(manga: &Html) -> Result<Option<&str>, Box<dyn Error>>;
    fn chapters(manga: &Html) -> Result<Option<Vec<&str>>, Box<dyn Error>>;
    fn pages(chapter: &Html) -> Result<Option<Vec<&str>>, Box<dyn Error>>;
}

struct Manga {}
impl Extension for Manga {
    fn name(book: &Html) -> Result<Option<&str>, Box<dyn Error>> {
        //let selector = Selector::parse(".story-info-right h1").unwrap();
        let selector = Selector::parse(".story-info-left img").unwrap();
        let name = book
            .select(&selector)
            // Returns None if String is empty
            //.map(|e| Some(e.inner_html()).filter(|e| !e.is_empty()))
            .map(|e| e.value().attr("title"))
            .next()
            .flatten();
        Ok(name)
    }

    fn cover(book: &Html) -> Result<Option<&str>, Box<dyn Error>> {
        let selector = Selector::parse(".story-info-left img").unwrap();
        let cover = book
            .select(&selector)
            .map(|e| e.value().attr("src"))
            .next()
            .flatten();
        Ok(cover)
    }

    fn chapters(book: &Html) -> Result<Option<Vec<&str>>, Box<dyn Error>> {
        let selector = Selector::parse(".panel-story-chapter-list a").unwrap();
        let chapters: Vec<&str> = book
            .select(&selector)
            .filter_map(|e| e.value().attr("href"))
            .collect();
        // Returns None if Vector is empty
        Ok(Some(chapters).filter(|v| !v.is_empty()))
    }

    fn pages(chapter: &Html) -> Result<Option<Vec<&str>>, Box<dyn Error>> {
        let selector =
            Selector::parse(".container-chapter-reader img").unwrap();
        let pages: Vec<&str> = chapter
            .select(&selector)
            .filter_map(|e| e.value().attr("data-src"))
            .collect();
        Ok(Some(pages).filter(|v| !v.is_empty()))
    }
}


struct ManganeloTV {}
impl Extension for ManganeloTV {
    fn name(book: &Html) -> Result<Option<&str>, Box<dyn Error>> {
        //let selector = Selector::parse(".story-info-right h1").unwrap();
        let selector = Selector::parse(".story-info-left img").unwrap();
        let name = book
            .select(&selector)
            // Returns None if String is empty
            //.map(|e| Some(e.inner_html()).filter(|e| !e.is_empty()))
            .map(|e| e.value().attr("title"))
            .next()
            .flatten();
        Ok(name)
    }

    fn cover(book: &Html) -> Result<Option<&str>, Box<dyn Error>> {
        let selector = Selector::parse(".story-info-left img").unwrap();
        let cover = book
            .select(&selector)
            .map(|e| e.value().attr("src"))
            .next()
            .flatten();
        Ok(cover)
    }

    fn chapters(book: &Html) -> Result<Option<Vec<&str>>, Box<dyn Error>> {
        let selector = Selector::parse(".panel-story-chapter-list a").unwrap();
        let chapters: Vec<&str> = book
            .select(&selector)
            .filter_map(|e| e.value().attr("href"))
            .collect();
        // Returns None if Vector is empty
        Ok(Some(chapters).filter(|v| !v.is_empty()))
    }

    fn pages(chapter: &Html) -> Result<Option<Vec<&str>>, Box<dyn Error>> {
        let selector =
            Selector::parse(".container-chapter-reader img").unwrap();
        let pages: Vec<&str> = chapter
            .select(&selector)
            .filter_map(|e| e.value().attr("data-src"))
            .collect();
        Ok(Some(pages).filter(|v| !v.is_empty()))
    }
}
