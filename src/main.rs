use reqwest;
use scraper::Html;
use std::error::Error;

mod extension;

trait Extension {
    fn get_cover(manga: &Html) -> Result<Option<&str>, Box<dyn Error>>;
    fn get_name(manga: &Html) -> Result<Option<&str>, Box<dyn Error>>;
    fn get_chapters(manga: &Html) -> Result<Option<Vec<&str>>, Box<dyn Error>>;
    fn get_pages(chapter: &Html) -> Result<Option<Vec<&str>>, Box<dyn Error>>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manga = std::fs::read_to_string("./tmp/manga.html")?;
    let manga = Html::parse_document(&manga);
    let chapter = std::fs::read_to_string("./tmp/chapter.html")?;
    let chapter = Html::parse_document(&chapter);

    let name = extension::Website::get_name(&manga)?;
    let chapters = extension::Website::get_chapters(&manga)?;
    let cover = extension::Website::get_cover(&manga)?;
    let pages = extension::Website::get_pages(&chapter)?;

    Ok(())
}
