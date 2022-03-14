pub mod cli;
pub mod util;

#[tokio::main]
async fn main() {
    cli::run().await;
}
