pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
// if I forward scrapper erros using commented section of this mod the
// compiler will complain on running functions with `html: &Html` variable:
// cannot return value referencing local variable `html`

//use selectors::parser::SelectorParseErrorKind;
//type ScraperError<'a> = cssparser::ParseError<'a, SelectorParseErrorKind<'a>>;
//pub type Result<'a, T> = std::result::Result<T, Error<'a>>;

//#[derive(Debug)]
//pub enum Error<'a> {
//    StdError(Box<dyn std::error::Error>),
//    IoError(std::io::Error),
//    ParseError(ScraperError<'a>),
//}
//
//impl<'a> From<Box<dyn std::error::Error>> for Error<'_> {
//    fn from(err: Box<dyn std::error::Error>) -> Self {
//        Error::StdError(err)
//    }
//}
//
//impl<'a> From<std::io::Error> for Error<'_> {
//    fn from(err: std::io::Error) -> Self {
//        Error::IoError(err)
//    }
//}
//
//impl<'a> From<ScraperError<'a>> for Error<'a> {
//    fn from(err: ScraperError<'a>) -> Self {
//        Error::ParseError(err)
//    }
//}
