use crate::{
    api::Api,
    db,
    util::{self, types::Account},
};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use std::{thread, time::Duration};

fn menu_options() -> Vec<String> {
    return vec![
        String::from("Get accounts"),
        String::from("List credit cards"),
        String::from("↵ Back"),
    ];
}

async fn get_accounts(api: &Api, user_key: String) -> Vec<Account> {
    match api.get_accounts(user_key).await {
        Ok(response) => response.accounts,
        Err(e) => {
            println!("{}", e);
            vec![]
        }
    }
}

async fn get_credit_cards(api: &Api, user_key: String) -> Vec<util::types::CreditCard> {
    match api.get_credit_cards(user_key).await {
        Ok(response) => response.credit_cards,
        Err(e) => {
            println!("{}", e);
            vec![]
        }
    }
}

pub async fn menu(api: &Api) {
    let mut selection: usize;

    match api.api_key {
        Some(_) => match db::get_user_key() {
            Some(user_key) => loop {
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
                        let accounts = get_accounts(api, user_key.clone()).await;
                        util::clear_console();
                        for account in &accounts {
                            println!("{}\n", account)
                        }
                        util::back_menu();
                    }
                    1 => {
                        let credit_cards = get_credit_cards(api, user_key.clone()).await;
                        util::clear_console();
                        for cc in &credit_cards {
                            println!("{}\n", cc)
                        }
                        util::back_menu();
                    }
                    2 | _ => break,
                };
            },
            None => {
                println!(
                    "{}",
                    "You're not logged in. Please do so before using this module.".red()
                );
                thread::sleep(Duration::from_millis(1700));
            }
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
