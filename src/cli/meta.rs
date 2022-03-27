use crate::{
    api::Api,
    util::{self, types::Provider},
};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use std::{thread, time::Duration};

fn menu_options() -> Vec<String> {
    return vec![
        String::from("List providers"),
        String::from("Get Provider details"),
        String::from("↵ Back"),
    ];
}

async fn list_providers(api: &Api) -> Vec<Provider> {
    return api.list_providers().await;
}

async fn get_provider_details(providers: &Vec<Provider>) -> &Provider {
    let provider_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a provider (🠕/🠗)")
        .default(0)
        .items(&providers[..])
        .interact()
        .unwrap();

    return &providers[provider_selection];
}

pub async fn menu(api: &Api) {
    let mut selection: usize;
    println!("Fetching cached providers...");
    let mut cached_providers: Vec<Provider> = list_providers(api).await;

    match api.api_key {
        Some(_) => loop {
            util::clear_console();
            util::print_top_message();

            selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select an option (🠕/🠗)")
                .default(0)
                .items(&menu_options()[..])
                .interact()
                .unwrap();

            match selection {
                0 => {
                    cached_providers = list_providers(api).await;
                    for provider in &cached_providers {
                        println!("{}", provider)
                    }
                    util::back_menu();
                }
                1 => {
                    let provider_details = get_provider_details(&cached_providers).await;
                    println!(
                        "{:#?}",
                        serde_json::from_str::<Provider>(
                            &serde_json::to_string(&provider_details)
                                .unwrap()
                                .to_string()
                        )
                        .unwrap()
                    );
                    util::back_menu();
                }
                2 | _ => break,
            };
        },
        None => {
            println!(
                "{}",
                "API Key is not set. Please do so before using this module.".red()
            );
            thread::sleep(Duration::from_millis(1700));
        }
    }
}
