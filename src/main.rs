// TODO: enums for genres, authors, ...
// TODO: MAL, AniList, ... trackers?
// TODO: tokio single vs multi thread
// TODO: HasComic need trait bound?
// TODO: how to handle pagination
// TODO: redownload missing files
//
// NEXT: Downloader
#![forbid(unsafe_code)]
// disable some lints for debug builds
//#![cfg_attr(debug_assertions, allow(dead_code, unused, unused))]

mod error;
mod ext;
mod model;
mod source;
use clap::Parser;
use error::{Error, Result};
use ext::*;
use model::*;
use source::TestSource;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(value_parser)]
    url: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    //let h = reqwest::get(cli.url).await?.text().await?;
    //std::fs::write("tmp/tmp.html", &h)?;
    dbg!(popular(&cli.url)?);
    Ok(())
}

fn latest(source_url: &str) -> Result<Index> {
    let index = match source_url {
        "latest" => TestSource::latest(),
        _ => Err(Error::LatestNotSupported),
    }?;
}
fn popular(source_url: &str) -> Result<Index> {
    let index = match source_url {
        "popular" => TestSource::popular(),
        _ => Err(Error::PopularNotSupported),
    }?;
}

//fn insert_index() {
//
//}

//impl Item {
//    pub fn cover_thumbnail(&self) -> Result<> {
//            
//
//    }
//}
