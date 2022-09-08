// Allow closures to capture variables
//https://github.com/matklad/once_cell/issues/156

// This is an anti-pattern but There isn't any other way
// https://rust-unofficial.github.io/patterns/anti_patterns/deref.html

use std::fmt;
use std::ops::{Deref, DerefMut};

type Lazy<T> = once_cell::unsync::Lazy<T, Box<dyn FnOnce() -> T + 'static>>;
pub struct LazyBoxedInit<T>(Lazy<T>);

impl<T> LazyBoxedInit<T> {
    pub fn new<F: FnOnce() -> T + 'static>(init: F) -> Self {
        Self(Lazy::new(Box::new(init)))
    }
}

impl<T> Deref for LazyBoxedInit<T> {
    type Target = Lazy<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for LazyBoxedInit<T> {
    fn deref_mut(&mut self) -> &mut Lazy<T> {
        &mut self.0
    }
}

impl<T: fmt::Debug> fmt::Debug for LazyBoxedInit<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

/*
let html = std::fs::read_to_string("./tmp/page.html")?;
let html = Html::parse_document(&html);

let s = Selector::parse("#image-container img").unwrap();
let cdn = (|| {
   let v = html.select(&s).next()?.value().attr("src")?;
   let v = v
       .strip_suffix("/1.jpg")
       .or_else(|| v.strip_prefix("/1.png"))?;
   Some(v)
})()
.ok_or_else(|| err)?;

// extract the extension of images from <script>...</script>
// then generate the urls based on them
let s = Selector::parse("script").unwrap();
let (image_urls, image_thumbnail_urls) = (|| {
   let v: (Vec<_>, Vec<_>) = html
       .select(&s)
       .nth(3)?
       .inner_html()
       .lines()
       .nth(4)?
       .trim_start()
       .strip_prefix("var images_ext = [\"")?
       .strip_suffix("\"];")?
       .split("\",\"")
       .enumerate()
       .map(|(i, s)| {
           let ext = match s {
               "j" => Some("jpg"),
               "p" => Some("png"),
               _ => None,
           }?;
           let i = i + 1;
           Some((
               format!("{cdn}/{i}.{ext}").into_boxed_str(),
               format!("{cdn}/{i}t.{ext}").into_boxed_str(),
           ))
       })
       .collect::<Option<Vec<_>>>()?
       .into_iter()
       .unzip();
   Some((v.0.into_boxed_slice(), Some(v.1.into_boxed_slice())))
})()
.ok_or(err)?;
*/
