#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::process;

fn main() {
    let matches = App::new("day2")
        .about("Solving AoC2018::day2 https://adventofcode.com/2018/day/2")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("INPUT")
                .help("The input file to use")
                .default_value("input/day2.txt"),
        )
        .get_matches();

    if let Err(e) = day2::run(&matches) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
