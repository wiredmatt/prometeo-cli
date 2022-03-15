use dialoguer::{theme::ColorfulTheme, Select};

use crate::util::{self, db};

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
    let providers = util::list_providers().await;

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a provider (🠕/🠗)")
        .default(0)
        .items(&providers[..])
        .interact()
        .unwrap();

    let params = [("foo", "bar"), ("baz", "quux")];
    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .form(&params)
        .send()
        .await;

    // db::set_pref("API_KEY".to_string(), input);
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
