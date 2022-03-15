use dialoguer::{theme::ColorfulTheme, Select};

use crate::{api, db, util};

fn menu_options() -> Vec<String> {
    let mut login_option = "Login".to_owned();

    let user = db::get_user();

    match user {
        Some(u) => {
            login_option = [login_option, format!("(Logged in as {})", u.name)].join(" ");
            return vec![
                String::from(login_option),
                String::from("Logout"),
                String::from("↵ Back"),
            ];
        }
        None => return vec![String::from(login_option), String::from("↵ Back")],
    }
}

pub async fn login() {
    let providers = api::list_providers().await;

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a provider (🠕/🠗)")
        .default(0)
        .items(&providers[..])
        .interact()
        .unwrap();

    let params = [("foo", "bar"), ("baz", "quux")];
}

pub async fn menu() {
    let mut selection: usize;

    loop {
        util::clear_console();
        util::print_top_message();

        selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option (🠕/🠗)")
            .default(0)
            .items(&menu_options()[..])
            .interact()
            .unwrap();

        match selection {
            0 => login().await,
            1 | _ => break,
        };
    }
}
