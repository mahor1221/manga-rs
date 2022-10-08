use crate::error::Result;
use crate::model::*;
use scraper::Html;

pub trait IsSource {
    fn source() -> Source;
}
pub trait HasComic {
    fn comic(html: &Html) -> Result<raw::Comic>;
}
pub trait HasLatestIndex {
    fn latest() -> Result<raw::Index>;
}
pub trait HasPopularIndex {
    fn popular() -> Result<raw::Index>;
}
