use crate::{api::Api, db, util};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Password, Select};
use std::{thread, time::Duration};

fn menu_options() -> Vec<String> {
    let mut login_option = "Login".to_owned();

    let user = db::get_user();

    match user {
        Some(u) => {
            login_option = [login_option, format!("(Logged in as {})", u)].join(" ");
            return vec![
                String::from(login_option),
                String::from("Logout"),
                String::from("↵ Back"),
            ];
        }
        None => return vec![String::from(login_option), String::from("↵ Back")],
    }
}

async fn login(api: &Api) {
    let providers = api.list_providers().await;

    let provider = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a provider (🠕/🠗)")
        .default(0)
        .items(&providers[..])
        .interact()
        .unwrap();

    let username = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Type your username: ")
        .validate_with({
            move |input: &String| -> Result<(), &str> {
                if input.trim().len() == 0 {
                    Err("Invalid username")
                } else {
                    Ok(())
                }
            }
        })
        .interact_text()
        .unwrap()
        .to_string();

    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Type your password: ")
        .allow_empty_password(false)
        .interact()
        .unwrap()
        .to_string();

    let params = vec![
        ("provider", providers[provider].code.as_str()),
        ("username", username.as_str()),
        ("password", password.as_str()),
    ];

    match api.login(params).await {
        Ok(data) => {
            db::set_pref("USER_KEY".to_string(), data.key);
            db::set_pref("USERNAME".to_string(), username.clone());
            println!(
                "{} {}",
                "Logged in succesfully as:".green(),
                username.bold().green()
            );
            thread::sleep(Duration::from_millis(1600));
        }
        Err(_) => {
            println!("{}", "Wrong credentials".red());
            thread::sleep(Duration::from_millis(1700));
        }
    }
}

pub fn logout() {
    db::delete_pref("USERNAME".to_string());
    db::delete_pref("USER_KEY".to_string())
}

pub async fn menu(api: &Api) {
    let selection: usize;

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
                    login(api).await;
                    break;
                }
                1 => {
                    logout();
                    break;
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
