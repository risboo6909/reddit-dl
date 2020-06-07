mod downloader;
mod types;

use std::path::Path;

use crate::downloader::Downloader;
use clap::{crate_authors, App, Arg};
use tokio::runtime;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    let matches = App::new("Reddit images downloader")
        .author(crate_authors!())
        .version("0.1")
        .about("Downloads images from the given subreddit")
        .arg(
            Arg::with_name("subreddit")
                .help("subreddit name")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("limit")
                .short("l")
                .long("limit")
                .value_name("LIMIT")
                .help("Max number of images to download (defaults to 100)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("max_conn")
                .short("m")
                .long("max_connections")
                .value_name("MAXCONN")
                .help("Maximum number of simultaneous downloads (defaults to 10)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dir")
                .short("d")
                .long("dir")
                .value_name("DIR")
                .help("Directory to save downloaded images (default is $HOME/Pictures/reddit)")
                .takes_value(true),
        )
        .get_matches();

    let d = Downloader::new(
        matches.value_of("subreddit").unwrap(),
        matches.value_of("dir").map(|x| Path::new(x).to_path_buf()),
        matches
            .value_of("limit")
            .map(|x| x.parse::<usize>().expect("error parsing limit value")),
        matches
            .value_of("max_conn")
            .map(|x| x.parse::<usize>().expect("error parsing max_conn value")),
    );

    let mut threaded_rt = runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()?;

    threaded_rt.block_on(async {
        let urls = d.download_page().await?;
        d.download_urls(&urls).await?;
        Ok(())
    })
}
