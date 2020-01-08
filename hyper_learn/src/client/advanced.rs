use hyper::Client;
// use hyper::{Body, Method, Request, Uri};
use hyper::Uri;
// use tokio::io::{stdout, AsyncWriteExt as _};

pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // let req = Request::builder()
    //     .method(Method::POST)
    //     .uri("http://httpbin.org/post")
    //     .header("content-type", "application/json")
    //     .body(Body::from(r#"{"library":"hyper"}"#))?;
    //
    // let client = Client::new();
    // let resp = client.request(req).await?;
    // println!("Response: {}", resp.status());
    
    // While await allows us to write “asynchronous” code in a way that looks “synchronous”,
    // to take full advantage of it, we can make multiple requests in parallel instead of serially.
    // We’re going to take advantage of “joining” futures.

    let client = Client::new();

    let ip_fut = async {
        let resp = client.get(Uri::from_static("http://httpbin.org/ip")).await?;
        hyper::body::to_bytes(resp.into_body()).await
    };

    let headers_fut = async {
        let resp = client.get(Uri::from_static("http://httpbin.org/headers")).await?;
        hyper::body::to_bytes(resp.into_body()).await
    };

    let (ip, headers) = futures::try_join!(ip_fut, headers_fut)?;

    println!("ip: {:?}", ip);
    println!("headers: {:?}", headers);

    Ok(())
}
