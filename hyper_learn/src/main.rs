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
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     hyper_learn::server::echo::main().await
// }

// Graceful shutdown:
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     if let Err(e) = hyper_learn::server::shutdown::main().await {
//         eprintln!("Server error: {}", e);
//         println!("Server error: {}", e);
//     }
//     Ok(())
// }

// ***Client***

// Getting started:
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     hyper_learn::client::start::main().await
// }

// Advanced client usage:
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     hyper_learn::client::advanced::main().await
// }

// Client configuration:
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    hyper_learn::client::config::main().await
}
