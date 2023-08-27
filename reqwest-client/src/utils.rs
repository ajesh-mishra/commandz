use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::Read;
use std::path::Path;

fn basename(path: String, sep: char) -> String {
    path
        .trim()
        .trim_matches('"')
        .rsplit(sep)
        .next()
        .unwrap()
        .to_owned()
}

// TODO: Only supports Bash and Zsh shell
fn get_history_file() -> OsString {
    env::var_os("HISTFILE").unwrap_or_else(|| {
        let home = env::var_os("HOME").unwrap();
        let shell_path = env::var_os("SHELL").unwrap();
        let shell = basename(format!("{:?}", shell_path), '/');
        let history_file = format!(".{shell}_history");

        Path::new(&home).join(&history_file).into()
    })
}

// TODO: Could be optimized
pub fn get_last_commands(_n: i32) -> Vec<String> {
    let history_file = get_history_file();

    let mut file = fs::File::open(history_file)
        .expect("Failed to open the history file!");

    let mut text = String::new();

    file.read_to_string(&mut text)
        .expect("Failed to read the history file");

    let mut commands = Vec::with_capacity(10);

    for (index, line) in text.lines().rev().enumerate() {
        if index >= 15 {
            break;
        }
        commands.push(line.to_owned());
    }

    commands.reverse();
    commands
}