pub mod argparser {
	//! Commandline argument handler.
	//!
	//! This module contains the code for parsing the commandline arguments,
	//! and a struct to handle the parsed arguments.
	//!
	//! Since this is a specific purpose parser, it is not possible to add more arguments to parse
	//! without modifying [Arguments].
	//!
	//! # Examples
	//!
	//! The default use of this module is to parse the args from [env::args](std::env::args).
	//! ```
	//! use std::env;
    //! use rust_aoc_2023::argparser::Arguments;
    //!
    //! let args = Arguments::parse(env::args());
    //! ```
    //!
    //! However it is also possible to parse arguments from other sources:
    //! ```
    //! use rust_aoc_2023::argparser::Arguments;
    //!
    //! // The first argument will be ignored.
    //! let argstrs = ["", "--time", "--day", "17"];
    //! let args = Arguments::parse(argstrs.iter().map(|str| str.to_string())).unwrap();
    //!
    //! assert_eq!(args, Arguments {days: vec![17], time: true, help: false});
    //! ```
    use std::error::Error;
    use std::fmt;

    /// Represents all the values given using command line arguments.
    ///
    /// An instance of Arguments stores all the configuration caused by all the given argument strings.  
    /// Use [parse](Self::parse) to create an arguments instance from a string iterator.
    ///
    /// # Examples
    ///
    /// The most common use case for this is to create an Arguments instance from [env::args](std::env::args).
    /// ```
    /// use std::env;
    /// use rust_aoc_2023::argparser::Arguments;
    ///
    /// let args = Arguments::parse(env::args());
    /// ```
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Arguments {
        pub days: Vec<u8>,
        pub help: bool,
        pub time: bool,
    }

    impl Arguments {
        /// Parses an iterator of strings to an Arguments instance.
        ///
        /// Takes an iterator iterating over the arguments to parse,
        /// and returns an arguments instance containing whether to print the help text,
        /// whether to time day executions, and which days to execute.
        ///
        /// The first result of the iterator will be ignored, since that would be the program path with [env::args](std::env::args).
        ///
        /// # Errors
        ///
        /// Will return a [ParseIntError](std::num::ParseIntError) if a day value can't be parsed.  
        /// Will return an [UnknownArgError] if an argument can't be parsed.
        ///
        /// # Examples
        ///
        /// The default usecase is to parse the program arguments.
        /// ```
        /// use std::env;
        /// # use rust_aoc_2023::argparser::Arguments;
        ///
        /// let args = Arguments::parse(env::args());
        /// ```
        ///
        /// It is, however, also possible to parse other iterators, for example from a Vec:
        /// ```
        /// # use rust_aoc_2023::argparser::Arguments;
        /// #
        /// // The first argument will be ignored.
        /// let argstrs: Vec<&str> = vec!["", "--time", "--day", "17"];
        /// let args = Arguments::parse(argstrs.iter().map(|str| str.to_string())).unwrap();
        ///
        /// # assert_eq!(args, Arguments {days: vec![17], time: true, help: false});
        /// ```
        pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Arguments, Box<dyn Error>> {
            args.next();
            let mut days: Vec<u8> = Vec::new();
            let mut help = false;
            let mut time = false;
            let mut next_days = false;
            for mut arg in args {
                arg = arg.trim().to_owned();
                if next_days {
                    next_days = false;
                    let elements: Vec<&str> = arg.split('-').collect();
                    if elements.len() == 1 {
                        let day: u8 = elements[0].parse()?;
                        if day == 0 || day > 25 {
                            // TODO Add more descriptive error.
                            return Err(Box::new(UnknownArgError::new(arg, None)));
                        }
                        days.push(day);
                    } else if elements.len() == 2 {
                        let start: u8 = elements[0].parse()?;
                        let end: u8 = elements[1].parse()?;
                        if start == 0 || end == 0 || start > 25 || end > 25 || start > end {
                            // TODO Add more descriptive error.
                            return Err(Box::new(UnknownArgError::new(arg, None)));
                        }
                        days.extend(start..=end);
                    } else {
                        // TODO Add more descriptive error.
                        return Err(Box::new(UnknownArgError::new(arg, None)));
                    }
                } else if arg.starts_with("--") {
                    arg = arg.to_lowercase();
                    if arg == "--help" {
                        help = true;
                    } else if arg == "--time" {
                        time = true;
                    } else if arg == "--day" {
                        next_days = true;
                    } else {
                        return Err(Box::new(UnknownArgError::new(arg, None)));
                    }
                } else if arg.starts_with('-') {
                    let mut chars = arg.chars();
                    chars.next();
                    for (i, c) in chars.enumerate() {
                        if next_days {
                            // TODO Add more descriptive error.
                            return Err(Box::new(UnknownArgError::new(arg, None)));
                        }

                        if c == 'h' {
                            help = true;
                        } else if c == 't' {
                            time = true;
                        } else if c == 'd' {
                            next_days = true;
                        } else {
                            return Err(Box::new(UnknownArgError::new(arg, Some(i + 1))));
                        }
                    }
                } else {
                    return Err(Box::new(UnknownArgError::new(arg, None)));
                }
            }
            Ok(Arguments { days, help, time })
        }
    }

    /// An error to describing an unknown argument.
    ///
    /// This error can either represent an argument string being unknown,
    /// or a single argument of a compound argument being unknown.
    ///
    /// A compound argument means using multiple single-character arguments without spaces
    /// or hyphens in between.
    ///
    /// # Examples
    ///
    /// An UnknownArgError representing the entire string being invalid:
    /// ```
    /// use rust_aoc_2023::argparser::UnknownArgError;
    ///
    /// let error = UnknownArgError::new(String::from("--test"), None);
    /// # assert_eq!(error.to_string(), "Received unknown argument \"--test\"!");
    /// ```
    ///
    /// An UnknownArgError representing a single invalid argument in a compount argument:
    /// ```
    /// use rust_aoc_2023::argparser::UnknownArgError;
    ///
    /// let error = UnknownArgError::new(String::from("-htxv"), Some(3));
    /// assert_eq!(error.to_string(), "Received unknown argument \"-x\"!");
    /// ```
    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct UnknownArgError {
        arg: String,
        index: Option<usize>,
    }

    impl UnknownArgError {
        /// Constructs a new UnknownArgError.
        ///
        /// When index is `Some` this error represents that one argument in a compount argument is unknown.  
        /// If it is `None` it represents that the whole argument string is unknown.
        ///
        /// An index outside of arg is not supported, and will currently cause the message to indicate the whole
        /// argument being invalid.
        ///
        /// # Examples
        ///
        /// An UnknownArgError representing the entire string being invalid:
        /// ```
        /// # use rust_aoc_2023::argparser::UnknownArgError;
        /// #
        /// let error = UnknownArgError::new(String::from("--test"), None);
        /// # assert_eq!(error.to_string(), "Received unknown argument \"--test\"!");
        /// ```
        pub fn new(arg: String, index: Option<usize>) -> UnknownArgError {
            // TODO validate index.
            UnknownArgError { arg, index }
        }
    }

    impl fmt::Display for UnknownArgError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
            if let Some(idx) = self.index {
                if let Some(c) = self.arg.chars().nth(idx) {
                    return write!(f, "Received unknown argument \"-{}\"!", c);
                }
            }
            write!(f, "Received unknown argument \"{}\"!", self.arg)
        }
    }

    impl Error for UnknownArgError {}

    // TODO add unit tests.
}

use std::env;
use std::io;
use std::path::PathBuf;
use std::process;
use std::time::{Duration, Instant};

/// Gets the absolute path of the input file for the given day.
///
/// This function determines the absolute path of the input file to read for the given day.  
/// The resulting path is the file DayX.txt where X is the given day, in the input directory.  
/// The input directory is assumed to be in the parent directory of the directory in which the program is being executed.
///
/// # Errors
///
/// This function returns an [io::Error] if the current directory, the input directory,
/// or any parent directory doesn't exist.  
/// Also if any of the parent directories are a file, or an unreadable symlink.  
/// It will not cause an error if the input file does not exist.
///
/// # Examples
///
/// ```
/// # use std::path::PathBuf;
/// use rust_aoc_2023;
///
/// let input = rust_aoc_2023::get_input_file(15).unwrap();
/// # let mut path: PathBuf = ["..", "input"].iter().collect();
/// # path = path.canonicalize().unwrap();
/// # path.push("Day15.txt");
/// # assert_eq!(input, path);
/// ```
pub fn get_input_file(day: u8) -> Result<PathBuf, io::Error> {
    let mut input = env::current_dir()?.canonicalize()?;
    if let Some(path) = input.parent() {
        input = path.to_owned();
    }
    input.push("input");
    if !input.try_exists()? {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "input directory does not exist",
        ));
    }
    input.push(format!("Day{}.txt", day));
    Ok(input)
}

/// Converts the given duration to a human readable string.
///
/// This function converts the given duration to a string in this format: `512h 17m 52s 500ms 81µs`.  
/// Each unit that would have a value of 0 isn't printed at all, except for microseconds.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
///
/// use rust_aoc_2023;
///
/// assert_eq!(&rust_aoc_2023::format_duration(&Duration::new(17413, 61234700)), "4h 50m 13s 61ms 234µs");
/// assert_eq!(&rust_aoc_2023::format_duration(&Duration::new(41, 21000000)), "41s 21ms 0µs");
/// ```
pub fn format_duration(dur: &Duration) -> String {
    let mut us = dur.subsec_micros();
    let ms = us / 1000;
    us %= 1000;
    let mut seconds = dur.as_secs();
    let mut minutes = seconds / 60;
    seconds %= 60;
    let hours = minutes / 60;
    minutes %= 60;

    let mut result = String::new();
    if hours > 0 {
        result += &hours.to_string();
        result += "h ";
    }

    if minutes > 0 {
        result += &minutes.to_string();
        result += "m ";
    }

    if seconds > 0 {
        result += &seconds.to_string();
        result += "s ";
    }

    if ms > 0 {
        result += &ms.to_string();
        result += "ms ";
    }

    result += &us.to_string();
    result += "µs";
    result
}

/// Prints the help text for this program.
///
/// Writes a short description for each possible argument to the standard output stream.
///
/// # Examples
///
/// ```
/// use rust_aoc_2023;
///
/// rust_aoc_2023::print_help();
/// ```
pub fn print_help() {
    println!(
        "Usage: {} [<OPTION>] --day <DAY(S)>",
        env::args().next().unwrap_or(String::from("rust_aoc_2023"))
    );
    println!(" -d --day <DAY(S)>   Specifies one or more days to run. Multiple days can be specified using START-END. Can be used more than once.");
    println!(" -t --time           Measures and prints the execution time of each part of each executed day.");
    println!(" -h --help           Prints this help text and exits.");
}

/// Solve a sequence of days.
///
/// This function executes a sequence of [DayRunners](days::DayRunner).  
/// The days will be run in the order the iterator returns them.
///
/// Currently the function prints an error message if something goes wrong.  
/// It also terminates the program if an error occures while trying to get a runner for a day.  
/// TODO implment better error handling.
///
/// An error message will be printed to the log for every day that couldn't be found.
///
/// Setting time to `true` will cause this function to print log messages about the execution times.  
/// One such message will be called for the init, part1, and part2 methods of each day.  
/// In addition one message will be printed containing the total execution time of all days.
///
/// # Panics
///
/// This function panics if it receives a day that is 0 or greater than 25.  
/// It also panics if [days::init] wasn't called before this function.  
/// Note that [days::init] is automatically called by [days::register_day_runner_constructor] and [days::register_day_runner_default].
///
/// # Examples
///
/// Running days from a u8 slice:
/// ```
/// use rust_aoc_2023;
/// use rust_aoc_2023::days;
/// 
/// days::init(true);
/// rust_aoc_2023::run_days([1, 7, 5].iter(), false);
/// ```
///
/// Timing day executions:
/// ```
/// # use rust_aoc_2023;
/// # use rust_aoc_2023::days;
/// #
/// # days::init(true);
/// rust_aoc_2023::run_days([7, 9, 1].iter(), true);
/// ```
pub fn run_days<'a, I: Iterator<Item = &'a u8>>(days: I, time: bool) {
	// TODO move code executing a single day to its own function
    let start = Instant::now();
    for day in days {
    	if *day == 0 || *day > 25 {
    		panic!("Received invalid day {}!", *day);
    	}

        let runner = days::get_day_runner(*day).unwrap_or_else(|err| {
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
                if time {
                    println!(
                        "Day {} initialization took {}.",
                        day,
                        format_duration(&init_start.elapsed())
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
                if time {
                    println!(
                        "Day {} part 1 took {}",
                        day,
                        format_duration(&part1_start.elapsed())
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
                if time {
                    println!(
                        "Day {} part 2 took {}",
                        day,
                        format_duration(&part2_start.elapsed())
                    );
                    println!(
                        "Running day {} took {} in total.",
                        day,
                        format_duration(&init_start.elapsed())
                    );
                }
            }
            None => eprintln!("No runner for day {} found!", day),
        }
    }

    if time {
        println!(
            "Running all days took a total of {}.",
            format_duration(&start.elapsed())
        );
    }
}

pub mod days;
