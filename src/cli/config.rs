use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::{util, db};

fn menu_options() -> Vec<String> {
    let mut set_api_option = "Set API Key".to_owned();

    if db::get_api_key().is_some() {
        set_api_option = [set_api_option, "(Already set)".to_string()].join(" ");
    }

    vec![String::from(set_api_option), String::from("↵ Back")]
}

pub fn set_api_key() {
    let input = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Paste your API Key: ")
        .validate_with({
            move |input: &String| -> Result<(), &str> {
                if input.len() == 64 {
                    Ok(())
                } else {
                    Err("API Keys must be of length 64.")
                }
            }
        })
        .interact_text()
        .unwrap();

    db::set_pref("API_KEY".to_string(), input);
}

pub fn menu() {
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
            0 => set_api_key(),
            1 | _ => break,
        }
    }
}
