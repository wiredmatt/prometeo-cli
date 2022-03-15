use serde::Deserialize;

use crate::{db, util::types};

pub async fn list_providers() -> Vec<types::Provider> {
    fetch::<types::ProvidersResponse>("provider/", "GET", None)
        .await
        .providers
}

pub async fn fetch<T: for<'de> Deserialize<'de>>(
    endpoint: &str,
    protocol: &str,
    body: Option<String>,
) -> T {
    match db::get_api_key() {
        Some(api_key) => {
            let client = reqwest::Client::new();

            let res = match protocol {
                "POST" => {
                    client
                        .post(["https://banking.sandbox.prometeoapi.com", endpoint].join("/"))
                        .header("X-API-Key", api_key)
                        .body(body.unwrap_or("{}".to_string()))
                        .send()
                        .await
                }
                "GET" | _ => {
                    client
                        .get(["https://banking.sandbox.prometeoapi.com", endpoint].join("/"))
                        .header("X-API-Key", api_key)
                        .send()
                        .await
                }
            };

            match res {
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
