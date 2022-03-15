use serde::Deserialize;
use std::io::Read;

pub mod db;
pub mod types;

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

pub async fn list_providers() -> Vec<types::Provider> {
    fetch::<types::ProvidersResponse>("provider/")
        .await
        .providers
}

pub async fn fetch<T: for<'de> Deserialize<'de>>(endpoint: &str) -> T {
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
                    Err(e) => panic!("Unexpected response shape \n{}", e),
                },
                Err(e) => panic!("Can't parse response\n {}", e),
            }
        }
        None => panic!("Missing API KEY"),
    }
}
