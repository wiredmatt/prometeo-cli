use crate::{api, cli, util};
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

pub async fn menu() {
    match api::Api::init() {
        Ok(mut api) => {
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
                    0 => cli::auth::menu(&api).await,
                    3 => cli::config::menu(&mut api),
                    4 | _ => break,
                };
            }
        }
        Err(_) => todo!(),
    }
}
