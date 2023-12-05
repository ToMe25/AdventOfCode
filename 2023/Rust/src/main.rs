use std::{env, process};

use rust_aoc_2023::{argparser, days};

fn main() {
    let args = argparser::Arguments::parse(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1)
    });

    if args.help || args.days.is_empty() {
        rust_aoc_2023::print_help();
        return;
    }

    days::init(true);
    rust_aoc_2023::run_days(args.days.iter(), args.time);
}
