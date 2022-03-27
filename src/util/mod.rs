use std::io::Read;
use colored::*;

pub mod types;

pub fn get_file_contents(path: String) -> String {
    let mut file = std::fs::File::open(path).unwrap();
    let contents = &mut String::new();
    file.read_to_string(contents).unwrap();
    return contents.to_string();
}

pub fn print_top_message() {
    println!(
        "{}{}",
        get_file_contents("src/util/ascii_logo.txt".to_string()).red(),
        "\n【 Ｐｒｏｍｅｔｅｏ　ＣＬＩ 】".bright_red()
    );
}

pub fn clear_console() {
    assert!(std::process::Command::new("cls")
        .status()
        .or_else(|_| std::process::Command::new("clear").status())
        .unwrap()
        .success());
}
