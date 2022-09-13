//use std::error::Error;
//pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

pub type Result<T> = std::result::Result<T, Error>;

use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("")]
    Io(std::io::Error),
    #[error("")]
    Reqwest(reqwest::Error),
    #[error("")]
    ScraperSelectorParse,
    #[error("")]
    ElementNotFound,
    #[error("")]
    LatestNotSupported,
    #[error("")]
    PopularNotSupported,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

use selectors::parser::SelectorParseErrorKind;
type SelectorParseError<'i> =
    cssparser::ParseError<'i, SelectorParseErrorKind<'i>>;
impl<'i> From<SelectorParseError<'i>> for Error {
    fn from(e: SelectorParseError) -> Error {
        Error::ScraperSelectorParse
    }
}
