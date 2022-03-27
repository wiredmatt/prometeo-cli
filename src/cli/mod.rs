pub mod auth;
pub mod config;
pub mod main;
pub mod meta;
pub mod transactional_data;

pub async fn run() {
    main::menu().await;
}
