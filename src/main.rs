mod count;
mod ft;

use std::{fs::File, io::BufReader, path::PathBuf};

use clap::{Command, arg, command, value_parser};

use crate::count::{Count, RUST};

fn main() {
    let matches = command!()
        .arg(
            arg!(<PATTERN>)
                .id("pattern")
                .required(false)
                .value_parser(value_parser!(PathBuf))
                .default_value("src/main.rs"),
        )
        .get_matches();

    let pattern: &PathBuf = matches.get_one("pattern").unwrap();

    let file = File::open(pattern).unwrap();
    let mut reader = BufReader::new(&file);

    let count = Count::from_reader(&mut reader, RUST).unwrap();

    println!("{} {} {}", count.comment, count.blank, count.code);
}
