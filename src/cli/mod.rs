pub mod auth;
pub mod config;
pub mod main;

pub async fn run() {
    main::menu().await;
}
