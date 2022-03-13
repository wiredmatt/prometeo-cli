use dialoguer::{theme::ColorfulTheme, Input};
use jfs::Store;
use std::io::Read;

pub fn init_prefs() -> Store {
    match dirs::config_dir() {
        Some(path) => {
            let cfg_path = [path.display().to_string(), "prometeo.json".to_string()].join("/");
            let mut cfg = jfs::Config::default();
            cfg.single = true;

            let res = jfs::Store::new_with_cfg(cfg_path, cfg);

            match res {
                Ok(db) => db,
                Err(e) => panic!("Error: {:?}", e),
            }
        }
        None => panic!("Impossible to get your config dir!"),
    }
}

pub fn get_pref(key: &str) -> Option<String> {
    match init_prefs().get(key) {
        Ok(prefs) => prefs,
        Err(_) => None,
    }
}

pub fn set_pref(key: String, value: String) {
    match init_prefs().save_with_id(&value, &key) {
        Err(e) => panic!("Error {:?}", e),
        Ok(_) => (),
    }
}

pub fn get_file_contents(path: String) -> String {
    let mut file = std::fs::File::open(path).unwrap();
    let contents = &mut String::new();
    file.read_to_string(contents).unwrap();
    return contents.to_string();
}

pub fn print_top_message() {
    println!(
        "\x1b[31m{}\n------------------------------\n【 Ｐｒｏｍｅｔｅｏ　ＣＬＩ 】\n\x1b[0m",
        get_file_contents("src/util/ascii_logo.txt".to_string())
    );
}

pub fn clear_console() {
    assert!(std::process::Command::new("cls")
        .status()
        .or_else(|_| std::process::Command::new("clear").status())
        .unwrap()
        .success());
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

    set_pref("API_KEY".to_string(), input);
}
