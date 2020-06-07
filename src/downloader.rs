use std::cmp::min;
use std::path::PathBuf;

use bytes::BufMut;
use hyper::{body::HttpBody as _, client::ResponseFuture, Client, Uri};
use hyper_tls::HttpsConnector;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::{
    fs::{create_dir_all, File},
    prelude::*,
};
use url::Url;

use crate::{types, Result};
use futures::TryFutureExt;

const ENDPOINT: &str = "https://www.reddit.com/r/";
const DL_LIMIT: usize = 50;
const MAX_CONN: usize = 10;

pub(crate) struct Downloader {
    pub(crate) url: Uri,

    pub(crate) save_path: PathBuf,
    pub(crate) limit: usize,
    pub(crate) max_conn: usize,
}

fn extract_filename(url: &str) -> Result<String> {
    let tmp = &Url::parse(&url)?;
    let res = &tmp.path();
    Ok(res[1..res.len()].to_owned())
}

impl Downloader {
    pub(crate) fn new(
        subr: &str,
        save_path: Option<PathBuf>,
        limit: Option<usize>,
        max_conn: Option<usize>,
    ) -> Self {
        let limit = limit.unwrap_or(DL_LIMIT);
        let full_url = format!("{}{}.json?limit={}", ENDPOINT, subr, limit);

        Downloader {
            url: full_url
                .parse::<Uri>()
                .unwrap_or_else(|_| panic!("failed to parse URL: {}", full_url)),

            save_path: save_path
                .unwrap_or_else(|| dirs::picture_dir().unwrap().join("reddit").join(subr)),
            limit,
            max_conn: max_conn.unwrap_or(MAX_CONN),
        }
    }

    async fn recv(&self, fut: ResponseFuture) -> Result<bytes::BytesMut> {
        let mut buf = bytes::BytesMut::new();
        let mut res = fut.await?;

        while let Some(next) = res.data().await {
            let chunk = next?;
            buf.put(chunk);
        }

        Ok(buf)
    }

    async fn get_downloader(&self, url: Uri) -> Result<(String, bytes::BytesMut)> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        // receive file blocks async
        let content = self.recv(client.get(url.clone())).await?;

        Ok((url.to_string(), content))
    }

    pub(crate) async fn download_urls(&self, urls: &[Uri]) -> Result<()> {
        let mut urls_to_download = Vec::from(&urls[..min(urls.len(), self.limit)]);

        let progress_bar = ProgressBar::new(urls_to_download.len() as u64).with_style(
            ProgressStyle::default_bar()
                .template("{msg} [{elapsed_precise}] {bar:100.cyan/blue} [{pos}/{len}]")
                .progress_chars("##-"),
        );

        progress_bar.set_message("downloading images");

        while !urls_to_download.is_empty() {
            let mut clients_vec = Vec::with_capacity(self.max_conn);

            for _ in 0..min(self.max_conn, urls_to_download.len()) {
                let client = self.get_downloader(urls_to_download.pop().unwrap());

                clients_vec.push(client.and_then(|(url, res)| async {
                    let file_name = extract_filename(&url)?;

                    // save on disk
                    create_dir_all(&self.save_path).await?;

                    let save_path = self.save_path.join(&file_name);
                    let mut file = File::create(save_path).await?;
                    file.write_all(&res).await?;

                    progress_bar.inc(1);
                    Ok((url, res))
                }));
            }

            // wait all clients to finish
            futures::future::join_all(clients_vec).await;
        }

        progress_bar.finish_with_message("done");

        Ok(())
    }

    fn parse_subr(&self, resp: types::Resp) -> Result<Vec<Uri>> {
        let mut result = Vec::new();

        for post in resp.data.children {
            if let Some(preview) = post.data.preview {
                let tmp = preview.images[0].source.url.clone();
                result.push(tmp.replace("amp;", "").parse::<Uri>()?);
            }
        }

        Ok(result)
    }

    pub(crate) async fn download_page(&self) -> Result<Vec<Uri>> {
        println!("downloading page... ");
        let (_, content) = self.get_downloader(self.url.clone()).await?;
        let urls = self.parse_subr(serde_json::from_slice::<types::Resp>(&content)?)?;
        Ok(urls)
    }
}
