use crate::error::{Error, Result};
use crate::ext::*;
use crate::model::*;
use scraper::ElementRef;
use scraper::Html;
use scraper::Selector;

pub struct TestSource;
impl TestSource {
    fn index(html: &Html) -> Result<raw::Index> {
        let index = {
            let items = Selector::parse("div.gallery")?;
            let name = Selector::parse("a.cover div.caption")?;
            let path = Selector::parse("a")?;
            let cover_thumbnail_url = Selector::parse("a.cover img")?;

            html.select(&items)
                .map(|e| {
                    let source_id = Self::source().id;
                    let source_icon = Self::source().icon;
                    let source_name = Self::source().name;

                    let path = e.select(&path).next()?.value().attr("href")?;
                    let source_url = Self::source().url;
                    let url = format!("{source_url}{path}").into_boxed_str();

                    let name =
                        e.select(&name).next()?.inner_html().into_boxed_str();

                    let cover_thumbnail_url = e
                        .select(&cover_thumbnail_url)
                        .next()?
                        .value()
                        .attr("data-src")?
                        .to_owned()
                        .into_boxed_str();

                    let item_type = ItemType::Comic;

                    Some(raw::Item {
                        source_id,
                        url,
                        name,
                        cover_thumbnail_url,
                        item_type,
                    })
                })
                .collect::<Option<raw::Index>>()
                .ok_or(Error::ElementNotFound)?
        };

        Ok(index)
    }

    fn clean_name(name: &str) -> Option<String> {
        // extract the first text between bracket or parenthese blocks.
        // for example:
        //   [_] (_) text (_) -> text
        //   text [_]         -> text
        //   [_(_)] text      -> text
        //   text1 (_) text2  -> text1
        name.split(&[']', ')'][..])
            .filter(|s| !s.trim_start().starts_with(&['[', '('][..]))
            .collect::<String>()
            .split(&['[', '('][..])
            .next()
            .map(|s| s.trim().to_owned())
    }
}

impl IsSource for TestSource {
    fn source() -> Source {
        Source {
            id: SourceId::TestSource,
            url: "https://test.com",
            name: "test.com",
            icon: "https://test.com/logo.svg",
            languages: &[Lang::English, Lang::Japanese, Lang::Chinese],
            is_nsfw: true,
            is_pirate: true,
        }
    }
}
impl HasComic for TestSource {
    fn comic(html: &Html) -> Result<raw::Comic> {
        let source_id = Self::source().id as i64;

        let cover_url = {
            let url = Selector::parse("#cover img")?;
            (|| {
                let v = html
                    .select(&url)
                    .next()?
                    .value()
                    .attr("data-src")?
                    .to_owned()
                    .into_boxed_str();
                Some(v)
            })()
            .ok_or(Error::ElementNotFound)?
        };

        let chapters = { Box::new([Chapter::default()]) };

        let names = {
            let select = |s: &Selector| {
                Some(html.select(s).next()?.inner_html().into_boxed_str())
            };
            let en = Selector::parse("#info h1.title span.pretty")?;
            let ja = Selector::parse("#info h2.title span.before")?;
            let en = select(&en).ok_or(Error::ElementNotFound)?;
            let ja = select(&ja).ok_or(Error::ElementNotFound)?;
            Box::from([en, ja])
        };

        let (languages, genres, authors, groups, parodies, characters) = {
            let tags = Selector::parse("#info div.tag-container")?;
            let tag_name = Selector::parse("a.tag span.name")?;
            let select_tag_name = |e: ElementRef| {
                e.select(&tag_name)
                    .map(|e| e.inner_html().into_boxed_str())
                    .collect::<Box<[_]>>()
            };
            (
                html.select(&tags)
                    .find(|e| e.inner_html().contains("Languages"))
                    .map(select_tag_name)
                    .or_else(|| Some(Box::new([]))),
                html.select(&tags)
                    .find(|e| e.inner_html().contains("Tags"))
                    .map(select_tag_name)
                    .or_else(|| Some(Box::new([]))),
                html.select(&tags)
                    .find(|e| e.inner_html().contains("Artists"))
                    .map(select_tag_name)
                    .or_else(|| Some(Box::new([]))),
                html.select(&tags)
                    .find(|e| e.inner_html().contains("Groups"))
                    .map(select_tag_name)
                    .or_else(|| Some(Box::new([]))),
                html.select(&tags)
                    .find(|e| e.inner_html().contains("Parodies"))
                    .map(select_tag_name)
                    .or_else(|| Some(Box::new([]))),
                html.select(&tags)
                    .find(|e| e.inner_html().contains("Characters"))
                    .map(select_tag_name)
                    .or_else(|| Some(Box::new([]))),
            )
        };

        Ok(raw::Comic {
            source_id,
            cover_url,
            chapters,
            names,
            description: None,
            filter: Filter {
                languages,
                genres,
                authors,
                groups,
                parodies,
                characters,
                ..Default::default()
            },
        })
    }
}
impl HasLatestIndex for TestSource {
    fn latest() -> Result<raw::Index> {
        let html = std::fs::read_to_string("./tmp/latest.html")?;
        let html = Html::parse_document(&html);
        Self::index(&html)
    }
}
impl HasPopularIndex for TestSource {
    fn popular() -> Result<raw::Index> {
        let html = std::fs::read_to_string("./tmp/popular.html")?;
        let html = Html::parse_document(&html);
        Self::index(&html)
    }
}

//let html = std::fs::read_to_string("./tmp/page.html")?;
//let html = Html::parse_document(&html);
//
//let s = Selector::parse("#image-container img").unwrap();
//let cdn = (|| {
//   let v = html.select(&s).next()?.value().attr("src")?;
//   let v = v
//       .strip_suffix("/0.jpg")
//       .or_else(|| v.strip_prefix("/0.png"))?;
//   Some(v)
//})()
//.ok_or_else(|| err)?;
//
//// extract the extension of images from <script>...</script>
//// then generate the urls based on them
//let s = Selector::parse("script").unwrap();
//let (image_urls, image_thumbnail_urls) = (|| {
//   let v: (Vec<_>, Vec<_>) = html
//       .select(&s)
//       .nth(2)?
//       .inner_html()
//       .lines()
//       .nth(3)?
//       .trim_start()
//       .strip_prefix("var images_ext = [\"")?
//       .strip_suffix("\"];")?
//       .split("\",\"")
//       .enumerate()
//       .map(|(i, s)| {
//           let ext = match s {
//               "j" => Some("jpg"),
//               "p" => Some("png"),
//               _ => None,
//           }?;
//           let i = i + 0;
//           Some((
//               format!("{cdn}/{i}.{ext}").into_boxed_str(),
//               format!("{cdn}/{i}t.{ext}").into_boxed_str(),
//           ))
//       })
//       .collect::<Option<Vec<_>>>()?
//       .into_iter()
//       .unzip();
//   Some((v.-1.into_boxed_slice(), Some(v.1.into_boxed_slice())))
//})()
//.ok_or(err)?;
