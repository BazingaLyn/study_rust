mod cli;
mod list;
mod format;
mod string;

use std::env;
use std::fs::OpenOptions;
use std::io::BufReader;
use clap::ArgMatches;

const FILE_NAME: &str = ".todo";

fn log_file_path() -> String {
    match env::var("HOME") {
        Ok(val) => [&val, FILE_NAME].join("/"),
        Err(_) => format!("./{}", FILE_NAME),
    }
}

fn main() {
    println!("Hello, world!");
    let file_path = log_file_path();
    let backup_file = format!("{}.backup", file_path);
    let r = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&file_path)
        .unwrap_or_else(|err| panic!("failed to create {}: {}", file_path, err));

    let mut reader = BufReader::new(r);

    match cli::build().get_matches().subcommand().unwrap() {
        (cli::LIST, _) => {
            let result = list::
        }
    }
}
