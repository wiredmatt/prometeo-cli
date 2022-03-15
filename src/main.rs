pub mod api;
pub mod cli;
pub mod db;
pub mod util;

#[tokio::main]
async fn main() {
    cli::run().await;
}
