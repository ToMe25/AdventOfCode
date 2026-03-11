use std::process::ExitCode;
use std::{env, io};

use rust_aoc_2025::args::{self, ArgumentType};
use rust_aoc_2025::{dayrunner, days};

fn main() -> ExitCode {
    let args = args::parse_arguments(env::args().skip(1));
    match args {
        Ok(args) => {
            if args.is_empty() || args::has_argument(&args, ArgumentType::Help) {
                args::generate_help_text(io::stdout()).unwrap();
                return ExitCode::SUCCESS;
            }
            days::register_runners();
            dayrunner::run_days_from_args(args);
            return ExitCode::SUCCESS;
        }
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    }
}
