use crate::{
    db,
    util::types::{self, AccountsResponse, CreditCardsResponse, LoginResponse},
};
use core::panic;
use serde::Deserialize;
use std::time::Duration;

pub struct Api {
    pub client: reqwest::Client,
    pub api_key: Option<String>,
    base_url: String,
}

impl Api {
    pub fn init() -> reqwest::Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;
        let api_key = db::get_api_key();

        Ok(Self {
            client,
            api_key,
            base_url: String::from("https://banking.sandbox.prometeoapi.com"),
        })
    }

    pub fn update_api_key(&mut self) {
        self.api_key = db::get_api_key();
    }

    pub async fn fetch<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        method: &str,
        body: Option<String>,
        form_data: Option<Vec<(&str, &str)>>,
    ) -> Result<T, String> {
        match &self.api_key {
            Some(api_key) => {
                let response = match method {
                    "POST" => {
                        self.client
                            .post([self.base_url.as_str(), endpoint].join("/"))
                            .header("X-API-Key", api_key)
                            .body(body.unwrap_or("{}".to_string()))
                            .send()
                            .await
                    }
                    "POST+FORM" => {
                        self.client
                            .post([self.base_url.as_str(), endpoint].join("/"))
                            .header("X-API-Key", api_key)
                            .form(&form_data)
                            .send()
                            .await
                    }
                    "GET" | _ => {
                        self.client
                            .get([self.base_url.as_str(), endpoint].join("/"))
                            .header("X-API-Key", api_key)
                            .send()
                            .await
                    }
                };

                match response {
                    Ok(res) => match res.text().await {
                        Ok(body) => match serde_json::from_str(body.as_str()) {
                            Ok(data) => Ok(data),
                            Err(_) => Err(body),
                        },
                        Err(e) => panic!("Couldn't read body \n\n{}", e),
                    },
                    Err(_) => panic!("Network error"),
                }
            }
            None => panic!("Missing API KEY"),
        }
    }

    pub async fn login(&self, form_data: Vec<(&str, &str)>) -> Result<LoginResponse, String> {
        self.fetch::<types::LoginResponse>("login/", "POST+FORM", None, Some(form_data))
            .await
    }

    pub async fn list_providers(&self) -> Vec<types::Provider> {
        match self
            .fetch::<types::ProvidersResponse>("provider/", "GET", None, None)
            .await
        {
            Ok(data) => data.providers,
            Err(e) => panic!("Unexpected error\n\n{}", e),
        }
    }

    pub async fn get_provider_details(&self, code: String) -> Result<types::Provider, String> {
        match self
            .fetch::<types::Provider>(&["provider", code.as_str()].join("/"), "GET", None, None)
            .await
        {
            Ok(data) => Ok(data),
            Err(e) => panic!("Unexpected error\n\n{}", e),
        }
    }

    pub async fn get_accounts(&self, user_key: String) -> Result<AccountsResponse, String> {
        self.fetch::<types::AccountsResponse>(
            &[["account", "?key"].join("/"), user_key].join("="),
            "GET",
            None,
            None,
        )
        .await
    }

    pub async fn get_credit_cards(&self, user_key: String) -> Result<CreditCardsResponse, String> {
        self.fetch::<types::CreditCardsResponse>(
            &[["credit-card", "?key"].join("/"), user_key].join("="),
            "GET",
            None,
            None,
        )
        .await
    }
}
