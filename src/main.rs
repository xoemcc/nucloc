mod count;
mod ft;

use std::path::PathBuf;

use clap::{Command, arg, command, value_parser};

use crate::count::count_file;

fn main() {
    let matches = command!()
        .arg(
            arg!(<PATTERN>)
                .id("pattern")
                .required(false)
                .value_parser(value_parser!(PathBuf))
                .default_value("src/count.rs"),
        )
        .get_matches();

    let pattern: &PathBuf = matches.get_one("pattern").unwrap();

    let count = count_file(pattern).unwrap().code;

    println!("{count}")
}
