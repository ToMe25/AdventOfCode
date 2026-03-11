//! The module containing all the functionality to run the runners for individual days.
//!
//! Use [`run_day_parts`] to run the runner for a single day, after registering a runner for that day.  
//! Use [`run_days_from_args`] to run the runners for all days specified by a list of arguments.
//!
//! # Examples
//!
//! Running all days from the program arguments:
//!
//! ```
//! # use std::env;
//! # use rust_aoc_2025::args::{self, ArgumentError};
//! # use rust_aoc_2025::dayrunner;
//! #
//! let args = args::parse_arguments(env::args().skip(1))?;
//! dayrunner::run_days_from_args(args);
//! # Ok::<(), ArgumentError>(())
//! ```

use std::path::PathBuf;
use std::rc::Rc;
use std::time::Instant;
use std::{fs, io};

use super::args::instances::ArgumentInstance;
use super::args::{self, ArgumentType};
use super::utils;

mod traits;

pub use traits::BothPartsRunner;
pub use traits::DayRunner;
pub use traits::DayRunnerDate;
pub use traits::Part1Runner;
pub use traits::Part2Runner;

pub mod registry;

/// The struct containing the data a runner is given when executing the solution for a given day.
///
/// This struct is not supposed to be instantiated by runner code, only by the system calling them.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct RunContext {
    /// The number of the day currently being run.
    pub day: u8,
    /// Whether the part one solution for the given day should be run.
    pub part1: bool,
    /// Whether the part two solution for the given day should be run.
    pub part2: bool,
    /// Whether verbose logging is enabled.
    pub verbose: bool,
    /// Whether the execution time for the individual parts should be measured and logged.
    pub time: bool,
    /// The arguments given to this program.
    pub args: Rc<Vec<ArgumentInstance>>,
}

impl RunContext {
    /// Creates a default RunContext for the given date.
    ///
    /// This RunContext will run both parts, but not specify any other non default options.  
    /// It will neither enable verbose logging, nor timing part executions.
    #[allow(unused)]
    fn create_default_for_day(day: u8) -> RunContext {
        Self::create_default_for_day_parts(day, true, true)
    }

    /// Creates a default RunContext for the given part(s) of the given day.
    ///
    /// This RunContext will only run the specified part(s) and not specify any non default options.  
    /// It will neither enable verbose logging, nor timing the part execution.
    #[allow(unused)]
    fn create_default_for_day_parts(day: u8, part1: bool, part2: bool) -> RunContext {
        RunContext {
            day,
            part1,
            part2,
            time: false,
            verbose: false,
            args: Rc::new(Vec::new()),
        }
    }

    /// Returns a [`PathBuf`] containing the absolute path of the input file for the currently running day.
    ///
    /// The resulting path is the file `DayX.txt` where X is the given day, in the input directory.  
    /// The input directory is assumed to be in the parent directory of the directory in which the program is being executed.
    ///
    /// # Errors
    ///
    /// This function returns an [`io::Error`] if the current directory, the input directory,
    /// or any parent directory doesn't exist.  
    /// Also if any of the parent directories are a file, or an unreadable symlink.  
    /// It will not return an error if the input file does not exist.
    pub fn get_day_input_path(&self) -> Result<PathBuf, io::Error> {
        utils::get_input_file_path(self.day)
    }

    /// Returns the contents of the input file for the currently running day as a String.
    ///
    /// The path of the input file is the file `DayX.txt` where X is the given day, in the input directory.  
    /// The input directory is assumed to be in the parent directory of the directory in which the program is being executed.
    ///
    /// # Errors
    ///
    /// This function returns an [`io::Error`] if the input file, the input directory,
    /// or any parent directory doesn't exist or can't be read.
    pub fn read_day_input_file(&self) -> Result<String, io::Error> {
        Ok(fs::read_to_string(self.get_day_input_path()?)?)
    }

    /// Gets a reference to the [`Vec`] containing the arguments given for this day execution.
    ///
    /// The arguments are usually all of the arguments given to this program, parsed as [`ArgumentInstances`](ArgumentInstance).  
    /// The resulting [`Vec`] **MAY** be empty.
    pub fn get_args<'a>(&'a self) -> &'a Vec<ArgumentInstance> {
        &self.args
    }
}

/// Creates a variable with the given name containing an `Option<Instant>` representing the current time, if an optional condition is fulfilled.
///
/// The instant will be `None` if and only if the optional condition fails.
macro_rules! timing_init {
    ($start_name:ident) => {
        timing_init($start_name, true)
    };
    ($start_name:ident,$cond:expr) => {
        let $start_name = if $cond { Some(Instant::now()) } else { None };
    };
}

/// Measures the time since a given start `Option<Instant>`, and stores as a `Option<Duration>`.
///
/// The duration will only be `None` if the start instant was `None`.
macro_rules! timing_measure {
    ($start_name:ident,$dur_name:ident) => {
        let $dur_name = if let Some(ref start) = $start_name {
            Some(start.elapsed())
        } else {
            None
        };
    };
}

/// Prints the result of a timing measurement to the standard output.
macro_rules! timing_print {
    ($dur_name:ident,$day:expr$(,$timing_name:literal)?) => {
        if let Some(duration) = $dur_name {
            println!(
                concat!("Running day {} ", $($timing_name, " ",)? "took {}."),
                $day,
                utils::format_duration(&duration)
            );
        }
    };
}

/// Checks which parts of day solutions should be executed from a given set of arguments.
///
/// Prints an error message if a part that is neither 1 nor 2 is found, and ignores the number.
fn get_parts_from_args<Args>(args: Args) -> (bool, bool)
where
    Args: IntoIterator,
    Args::Item: AsRef<ArgumentInstance>,
{
    let iter = args
        .into_iter()
        .filter(|arg| arg.as_ref().arg == ArgumentType::Part)
        .flat_map(|arg| {
            arg.as_ref()
                .value
                .get_ints_value()
                .expect("part argument missing value")
                .to_owned()
        });

    let mut result = (false, false);
    for arg in iter {
        match arg {
            1 => result.0 = true,
            2 => result.1 = true,
            part => eprintln!("Part argument received invalid part {part}!"),
        }
    }
    result
}

/// Gets the days to be executed for a given set of arguments.
///
/// Ignores days above the number of days for this [Advent of Code](https://adventofcode.com/) with an error message.
fn get_days_from_args<Args>(args: Args) -> Vec<u8>
where
    Args: IntoIterator,
    Args::Item: AsRef<ArgumentInstance>,
{
    args.into_iter()
        .filter(|arg| arg.as_ref().arg == ArgumentType::Day)
        .flat_map(|arg| {
            arg.as_ref()
                .value
                .get_ints_value()
                .expect("day argument missing value")
                .to_owned()
        })
        .filter(|day| {
            if day <= &registry::MAX_DAY {
                true
            } else {
                eprintln!("Day argument received invalid day {day}!");
                false
            }
        })
        .collect()
}

/// Runs the days specified by the given arguments.
///
/// Runs the parts of the days specified by the given arguments.
///
/// Automatically enables timings and verbose logging based on the arguments given.  
/// Runs both parts if no [`Part`](ArgumentType::Part) argument was given.
///
/// If initialization of a day runner fails, neither part will be run, but if either part fails the other part will still be run.
///
/// Prints an error for invalid day and part numbers given.
///
/// # Panics
///
/// If a [`Day`](ArgumentType::Day) or [`Part`](ArgumentType::Part) argument does not have an integer value.
///
/// # Examples
///
/// Running this with the program arguments:
///
/// ```
/// # use std::env;
/// # use rust_aoc_2025::args::{self, ArgumentError};
/// # use rust_aoc_2025::dayrunner;
/// #
/// let args = args::parse_arguments(env::args().skip(1))?;
/// dayrunner::run_days_from_args(args);
/// # Ok::<(), ArgumentError>(())
/// ```
pub fn run_days_from_args<Args>(args: Args)
where
    Args: IntoIterator<Item = ArgumentInstance>,
{
    let args_vec: Vec<ArgumentInstance> = args.into_iter().collect();
    let verbose = args::has_argument(&args_vec, args::ArgumentType::Verbose);
    let time = args::has_argument(&args_vec, args::ArgumentType::Time);
    let parts = if args::has_argument(&args_vec, args::ArgumentType::Part) {
        get_parts_from_args(&args_vec)
    } else {
        (true, true)
    };
    let days = get_days_from_args(&args_vec);
    let args_rc = Rc::new(args_vec);

    if days.is_empty() {
        println!("No days to run specified.");
        return;
    }

    timing_init!(total_start, time);
    for day in days {
        let ctx = RunContext {
            day,
            part1: parts.0,
            part2: parts.1,
            verbose,
            time,
            args: args_rc.clone(),
        };
        run_day(&ctx);
    }
    timing_measure!(total_start, total_dur);
    if let Some(duration) = total_dur {
        println!(
            "Running all days took {}.",
            utils::format_duration(&duration)
        );
    }
}

/// Runs the specified parts of the specified day with the default options.
///
/// This function will print the result, or any errors, returned by the runner parts to the log.  
/// It will also print a message when trying to run a part that isn't implemented.
///
/// If initialization of a day runner fails, neither part will be run, but if either part fails the other part will still be run.
pub fn run_day_parts(day: u8, part1: bool, part2: bool) {
    run_day(&RunContext::create_default_for_day_parts(day, part1, part2));
}

/// Runs the parts of the day specified in the [`RunContext`].
///
/// This function will print the result, or any errors, returned by the runner parts to the log.  
/// It will also print a message when trying to run a part that isn't implemented.
///
/// If initialization of a day runner fails, neither part will be run, but if either part fails the other part will still be run.
///
/// This function will also print the execution timings if the [`ctx.time`](RunContext::time) is enabled.
fn run_day(ctx: &RunContext) {
    timing_init!(init_start, ctx.time);
    let runner = registry::get_runner_for_day(ctx);
    timing_measure!(init_start, init_dur);
    timing_print!(init_dur, ctx.day, "initialization");

    match runner {
        Some(Ok(ref runner)) => {
            if !ctx.part1 && !ctx.part2 {
                println!(
                    "Not running parts for day {} because neither part one nor part two was specified.",
                    ctx.day
                );
            }
            if ctx.part1 {
                timing_init!(part1_start, ctx.time);
                let result = runner.part1(ctx);
                timing_measure!(part1_start, part1_dur);
                match result {
                    Some(result) => {
                        match result {
                            Ok(result) => {
                                println!("Day {} part 1 result: {}", ctx.day, result);
                            }
                            Err(err) => {
                                eprintln!("Day {} part 1 failed.", ctx.day);
                                eprintln!("Error: {err}");
                            }
                        }

                        timing_print!(part1_dur, ctx.day, "part 1");
                    }
                    None => println!(
                        "Not running part 1 for day {} because it isn't implemented.",
                        ctx.day
                    ),
                };
            }

            if ctx.part2 {
                timing_init!(part2_start, ctx.time);
                let result = runner.part2(ctx);
                timing_measure!(part2_start, part2_dur);
                match result {
                    Some(result) => {
                        match result {
                            Ok(result) => {
                                println!("Day {} part 2 result: {}", ctx.day, result);
                            }
                            Err(err) => {
                                eprintln!("Day {} part 2 failed.", ctx.day);
                                eprintln!("Error: {err}");
                            }
                        }

                        timing_print!(part2_dur, ctx.day, "part 2");
                    }
                    None => println!(
                        "Not running part 2 for day {} because it isn't implemented.",
                        ctx.day
                    ),
                }
            }
        }
        Some(Err(ref err)) => {
            eprintln!("Runner for day {} failed to initialize.", ctx.day);
            eprintln!("Error: {err}");
        }
        None => {
            println!(
                "Not running day {} because no runner was registered for that day.",
                ctx.day
            );
        }
    }

    if runner.is_some() {
        timing_measure!(init_start, total_dur);
        timing_print!(total_dur, ctx.day);
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::{base_test_runner, register_runner, test_runner_part};

    use super::super::args;
    use super::{
        DayRunner, DayRunnerDate, Part1Runner, Part2Runner, RunContext, run_day, run_day_parts,
    };

    base_test_runner!(TestRunnerBothParts, 3);
    test_runner_part!(TestRunnerBothParts, 1);
    test_runner_part!(TestRunnerBothParts, 2);

    base_test_runner!(TestRunnerPart2, 4);
    test_runner_part!(TestRunnerPart2, 2);

    /// Registers the test runners for the tests in this module.
    fn register_runners() {
        register_runner!(TestRunnerBothParts);
        register_runner!(TestRunnerPart2);
    }

    #[test]
    fn run_both_parts() {
        register_runners();
        run_day_parts(3, true, true);
    }

    #[test]
    fn run_one_part() {
        register_runners();
        run_day_parts(3, true, false);
    }

    #[test]
    fn run_missing_part() {
        register_runners();
        run_day_parts(4, true, false);
    }

    #[test]
    fn run_timed() {
        let mut ctx = RunContext::create_default_for_day(3);
        ctx.time = true;
        run_day(&ctx);
    }

    #[test]
    fn get_parts_from_args() {
        let args = args::parse_arguments(vec!["-vtp 2", "--part 3"]).unwrap();
        let parts = super::get_parts_from_args(args);
        assert!(!parts.0, "Part 1 was incorrectly detected as enabled.");
        assert!(parts.1, "Part 2 wasn't correctly detected as enabled.");
    }

    #[test]
    fn get_no_parts_from_args() {
        let args = args::parse_arguments(Vec::<String>::new()).unwrap();
        let parts = super::get_parts_from_args(args);
        assert!(!parts.0, "Part 1 was incorrectly detected as enabled.");
        assert!(!parts.1, "Part 2 was incorrectly detected as enabled.");
    }

    #[test]
    fn get_days_from_args() {
        let args = args::parse_arguments(vec!["-d 7-15", "--day 3", "-d 13,2,7,1,9"]).unwrap();
        let days = super::get_days_from_args(args);
        assert_eq!(days, vec![7, 8, 9, 10, 11, 12, 3, 2, 7, 1, 9]);
    }

    #[test]
    fn run_days_from_args() {
        register_runners();
        let args = args::parse_arguments(vec!["-d 2-53", "-v"]).unwrap();
        super::run_days_from_args(args);
    }
}
