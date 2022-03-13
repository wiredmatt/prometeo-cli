use dialoguer::{theme::ColorfulTheme, Select};

use crate::util;

fn menu_options() -> Vec<String> {
    let mut set_api_option = "Set API Key".to_owned();

    if util::get_pref("API_KEY").is_some() {
        set_api_option = [set_api_option, "(Already set)".to_string()].join(" ");
    }

    vec![String::from(set_api_option), String::from("↵ Back")]
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
            0 => util::set_api_key(),
            1 | _ => break,
        }
    }
}
