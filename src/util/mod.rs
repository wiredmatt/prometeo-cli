use std::io::Read;

pub mod types;

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
