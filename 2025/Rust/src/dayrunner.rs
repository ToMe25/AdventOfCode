//! The module containing all the functionality to run the runners for individual days.
//!
//! To do so run [`run_day_parts`] after registering a runner for that day.

use std::path::PathBuf;
use std::{fs, io};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct RunContext {
    /// The number of the day currently being run.
    pub day: u8,
    /// Whether the part one solution for the given day should be run.
    pub part1: bool,
    /// Whether the part two solution for the given day should be run.
    pub part2: bool,
}

impl RunContext {
    /// Creates a default RunContext for the given date.
    ///
    /// This run context will run both parts, but not specify any other non default options that may be added later.
    #[allow(unused)]
    fn create_default_for_day(day: u8) -> RunContext {
        RunContext {
            day,
            part1: true,
            part2: true,
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
    ///	The path of the input file is the file `DayX.txt` where X is the given day, in the input directory.  
    /// The input directory is assumed to be in the parent directory of the directory in which the program is being executed.
    ///
    ///	# Errors
    ///
    /// This function returns an [`io::Error`] if the input file, the input directory,
    /// or any parent directory doesn't exist or can't be read.
    pub fn read_day_input_file(&self) -> Result<String, io::Error> {
        Ok(fs::read_to_string(self.get_day_input_path()?)?)
    }
}

/// Runs the specified parts of the specified day with the default options.
///
/// This function will print the result, or any errors, returned by the runner parts to the log.  
/// It will also print a message when trying to run a part that isn't implemented.
pub fn run_day_parts(day: u8, part1: bool, part2: bool) {
    run_day(&RunContext { day, part1, part2 });
}

/// Runs the parts of the day specified in the [`RunContext`].
///
/// This function will print the result, or any errors, returned by the runner parts to the log.  
/// It will also print a message when trying to run a part that isn't implemented.
fn run_day(ctx: &RunContext) {
    let runner = registry::get_runner_for_day(ctx);
    if let Some(runner) = runner {
        match runner {
            Ok(runner) => {
                if !ctx.part1 && !ctx.part2 {
                    println!(
                        "Not running parts for day {} because neither part one nor part two was specified.",
                        ctx.day
                    );
                    return;
                }
                if ctx.part1 {
                    let result = runner.part1(ctx);
                    match result {
                        Some(Ok(result)) => println!("Day {} part 1 result: {}", ctx.day, result),
                        Some(Err(err)) => {
                            eprintln!("Day {} part 1 failed.", ctx.day);
                            eprintln!("Error: {err}");
                        }
                        None => println!(
                            "Not running part 1 for day {} because it isn't implemented.",
                            ctx.day
                        ),
                    };
                }
                if ctx.part2 {
                    let result = runner.part2(ctx);
                    match result {
                        Some(Ok(result)) => println!("Day {} part 2 result: {}", ctx.day, result),
                        Some(Err(err)) => {
                            eprintln!("Day {} part 2 failed.", ctx.day);
                            println!("Error: {}", err);
                        }
                        None => println!(
                            "Not running part 2 for day {} because it isn't implemented.",
                            ctx.day
                        ),
                    }
                }
            }
            Err(err) => {
                eprintln!("Runner for day {} failed to initialize.", ctx.day);
                eprintln!("Error: {err}");
            }
        }
    } else {
        println!(
            "Not running day {} because no runner was registered for that day.",
            ctx.day
        );
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::register_runner;

    use super::{DayRunner, DayRunnerDate, Part1Runner, Part2Runner, RunContext, run_day_parts};

    struct TestRunnerBothParts();

    impl DayRunner for TestRunnerBothParts {
        fn init(_ctx: &RunContext) -> Result<Self, Box<dyn Error>>
        where
            Self: Sized,
        {
            Ok(TestRunnerBothParts())
        }
    }

    impl DayRunnerDate for TestRunnerBothParts {
        const DAY: u8 = 3;
    }

    impl Part1Runner for TestRunnerBothParts {
        fn part1(&self, ctx: &RunContext) -> Result<String, Box<dyn Error>> {
            assert_eq!(
                Self::DAY,
                ctx.day,
                "This runner was designed for  day {}, but run for day {}.",
                Self::DAY,
                ctx.day
            );
            Ok(format!("Day {} part 1", Self::DAY).to_owned())
        }
    }

    impl Part2Runner for TestRunnerBothParts {
        fn part2(&self, ctx: &RunContext) -> Result<String, Box<dyn Error>> {
            assert_eq!(
                Self::DAY,
                ctx.day,
                "This runner was designed for  day {}, but run for day {}.",
                Self::DAY,
                ctx.day
            );
            Ok(format!("Day {} part 2", Self::DAY).to_owned())
        }
    }

    #[test]
    fn test_run() {
        register_runner!(TestRunnerBothParts);
        run_day_parts(3, true, true);
    }

    // TODO add more unit tests.
}
