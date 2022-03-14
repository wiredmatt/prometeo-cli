use core::fmt;
use std::io::Read;

use serde::{Deserialize, Serialize};

pub mod db;

pub fn get_file_contents(path: String) -> String {
    let mut file = std::fs::File::open(path).unwrap();
    let contents = &mut String::new();
    file.read_to_string(contents).unwrap();
    return contents.to_string();
}

pub fn print_top_message() {
    println!(
        "\x1b[31m{}\n------------------------------\n【 Ｐｒｏｍｅｔｅｏ　ＣＬＩ 】\n\x1b[0m",
        get_file_contents("src/util/ascii_logo.txt".to_string())
    );
}

pub fn clear_console() {
    assert!(std::process::Command::new("cls")
        .status()
        .or_else(|_| std::process::Command::new("clear").status())
        .unwrap()
        .success());
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Provider {
    pub code: String,
    pub country: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProvidersResponse {
    pub providers: Vec<Provider>,
    pub status: String,
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.code)
    }
}
pub async fn list_providers() -> Vec<Provider> {
    rekuest::<ProvidersResponse>("provider/").await.providers
}

pub async fn rekuest<T: for<'de> Deserialize<'de>>(endpoint: &str) -> T {
    match db::get_api_key() {
        Some(api_key) => {
            match reqwest::Client::new()
                .get(["https://banking.sandbox.prometeoapi.com", endpoint].join("/"))
                .header("X-API-Key", api_key)
                .send()
                .await
            {
                Ok(res) => match res.json::<T>().await {
                    Ok(data) => data,
                    Err(e) => panic!("{}", e),
                },
                Err(_) => panic!("Can't parse response"),
            }
        }
        None => panic!("Missing API KEY"),
    }
}
