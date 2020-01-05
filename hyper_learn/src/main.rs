extern crate hyper;
extern crate tokio;

use hyper_learn;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     hyper_learn::client::start::main().await
// }

#[tokio::main]
async fn main() {
    hyper_learn::server::start::main().await
}
