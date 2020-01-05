// use hyper::Client;
// use hyper::body::HttpBody as _;
// use tokio::io::{stdout, AsyncWriteExt as _};
//
// pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     let client = Client::new();
//     let uri = "http://httpbin.org/ip".parse()?;
//     let mut resp = client.get(uri).await?;
//     println!("Response: {}", resp.status());
//
//     while let Some(chunk) = resp.body_mut().data().await {
//         stdout().write_all(&chunk?).await?;
//     }
//
//     Ok(())
// }

#![deny(warnings)]
#![warn(rust_2018_idioms)]
use std::env;

use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn main() -> Result<()> {
    pretty_env_logger::init();

    // Some simple CLI args requirements...
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client <url>");
            return Ok(());
        }
    };

    // HTTPS requires picking a TLS implementation, so give a better
    // warning if the user tries to request an 'https' URL.
    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme_str() != Some("http") {
        println!("This example only works with 'http' URLs.");
        return Ok(());
    }

    fetch_url(url).await
}

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let client = Client::new();

    let mut res = client.get(url).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }

    println!("\n\nDone!");

    Ok(())
}
