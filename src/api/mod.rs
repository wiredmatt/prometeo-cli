use serde::Deserialize;

use crate::{db, util::types};

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
