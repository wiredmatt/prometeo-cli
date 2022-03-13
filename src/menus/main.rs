use crate::{menus, util};
use dialoguer::{theme::ColorfulTheme, Select};

fn menu_options() -> Vec<String> {
    vec![
        String::from("Auth"),
        String::from("Transactional Data"),
        String::from("Meta"),
        String::from("Config"),
        String::from("Exit"),
    ]
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
            3 => menus::config::menu(),
            4 | _ => break,
        }
    }
}
