extern crate hyper;
extern crate tokio;

use hyper_learn;

// ***Server***

// Getting started:
// #[tokio::main]
// async fn main() {
//     hyper_learn::server::start::main().await
// }

// Echo:
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    hyper_learn::server::echo::main().await
}

// ***Client***

// Getting started:
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     hyper_learn::client::start::main().await
// }
