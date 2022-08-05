use crate::Extension;
use scraper::Html;
use scraper::Selector;
use std::error::Error;

pub struct Website {}
impl Extension for Website {
    fn get_cover(book: &Html) -> Result<Option<&str>, Box<dyn Error>> {
        let selector = Selector::parse(".story-info-left img").unwrap();
        let cover = book
            .select(&selector)
            .map(|e| e.value().attr("src"))
            .next()
            .flatten();
        Ok(cover)
    }

    fn get_name(book: &Html) -> Result<Option<&str>, Box<dyn Error>> {
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

    fn get_chapters(book: &Html) -> Result<Option<Vec<&str>>, Box<dyn Error>> {
        let selector = Selector::parse(".panel-story-chapter-list a").unwrap();
        let chapters: Vec<&str> = book
            .select(&selector)
            .filter_map(|e| e.value().attr("href"))
            .collect();
        // Returns None if Vector is empty
        Ok(Some(chapters).filter(|v| !v.is_empty()))
    }

    fn get_pages(chapter: &Html) -> Result<Option<Vec<&str>>, Box<dyn Error>> {
        let selector =
            Selector::parse(".container-chapter-reader img").unwrap();
        let pages: Vec<&str> = chapter
            .select(&selector)
            .filter_map(|e| e.value().attr("data-src"))
            .collect();
        // Returns None if Vector is empty
        Ok(Some(pages).filter(|v| !v.is_empty()))
    }
}
