use std::time::Instant;
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

    days::init();
    let start = Instant::now();
    for day in args.days {
        let runner = days::get_day_runner(day).unwrap_or_else(|err| {
            eprintln!(
                "An error occured while trying to get a runner for day {}!",
                day
            );
            eprintln!("Error: {}", err);
            process::exit(2)
        });
        match runner {
            Some(mut run) => {
                let init_start = Instant::now();
                let init_result = run.init();
                if init_result.is_err() {
                    eprintln!("Initializing day {} failed!", day);
                    eprintln!("Error: {}", init_result.err().unwrap());
                    continue;
                }
                if args.time {
                    println!(
                        "Initializing day {} took {}.",
                        day,
                        rust_aoc_2023::format_duration(&init_start.elapsed())
                    );
                }

                let part1_start = Instant::now();
                let part1_result = run.part1();
                if part1_result.is_err() {
                    eprintln!("Running day {} part 1 failed!", day);
                    eprintln!("Error: {}", part1_result.err().unwrap());
                    continue;
                }

                let part1_str = part1_result.unwrap();
                if part1_str.is_some() {
                    println!("Day {} part 1 result: {}", day, part1_str.unwrap());
                }
                if args.time {
                    println!(
                        "Day {} part 1 took {}",
                        day,
                        rust_aoc_2023::format_duration(&part1_start.elapsed())
                    );
                }

                let part2_start = Instant::now();
                let part2_result = run.part2();
                if part2_result.is_err() {
                    eprintln!("Running day {} part 2 failed!", day);
                    eprintln!("Error: {}", part2_result.err().unwrap());
                    continue;
                }

                let part2_str = part2_result.unwrap();
                if part2_str.is_some() {
                    println!("Day {} part 2 result: {}", day, part2_str.unwrap());
                }
                if args.time {
                    println!(
                        "Day {} part 2 took {}",
                        day,
                        rust_aoc_2023::format_duration(&part2_start.elapsed())
                    );
                    println!(
                        "Running day {} took {} in total.",
                        day,
                        rust_aoc_2023::format_duration(&init_start.elapsed())
                    );
                }
            }
            None => eprintln!("No runner for day {} found!", day),
        }
    }
    if args.time {
        println!(
            "Running all days took a total of {}.",
            rust_aoc_2023::format_duration(&start.elapsed())
        );
    }
}
