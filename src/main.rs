mod count;
mod output;

use std::{ffi::OsStr, fs::File, io::BufReader, path::PathBuf};

use clap::{Command, arg, command, value_parser};

use crate::count::{Count, LANGUAGES, RUST, TEXT};

fn main() {
    let matches = command!()
        .arg(
            arg!(<PATTERN>)
                .num_args(0..)
                .id("pattern")
                .required(false)
                .value_parser(value_parser!(PathBuf))
                .default_value("src/main.rs"),
        )
        .get_matches();

    let patterns: Vec<&PathBuf> = matches.get_many("pattern").unwrap().collect();

    patterns.iter().for_each(|p| {
        let ext = p.extension().unwrap_or(OsStr::new("")).to_string_lossy();

        let lang = LANGUAGES.iter().find(|l| l.is_file(&ext)).unwrap_or(&TEXT);

        let file = File::open(p).unwrap();
        let mut reader = BufReader::new(&file);

        let count = Count::from_reader(&mut reader, *lang).unwrap();

        println!(
            "{} {} {} {} {}",
            p.file_name().unwrap().to_string_lossy(),
            lang.name,
            count.comment,
            count.blank,
            count.code
        );
    });
}
