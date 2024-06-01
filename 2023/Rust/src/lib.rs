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
    //! let args = Arguments::parse(argstrs).unwrap();
    //!
    //! assert_eq!(args.get_days(), &vec![17]);
    //! assert_eq!(args.time, true);
    //! assert_eq!(args.help, false);
    //! ```
    use std::error::Error;
    use std::fmt;
    use std::num::ParseIntError;

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
        days: Vec<u8>,
        pub help: bool,
        pub time: bool,
        parts: (bool, bool),
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
        /// Will return a [ParseIntError] if a day value can't be parsed.  
        /// Will return an [UnknownArgError] if an argument can't be parsed.
        ///
        /// # Examples
        ///
        /// The default usecase is to parse the program arguments.
        /// ```
        /// use std::env;
        /// use rust_aoc_2023::argparser::Arguments;
        ///
        /// let args = Arguments::parse(env::args());
        /// ```
        ///
        /// It is, however, also possible to parse other iterators, for example from a Vec:
        /// ```
        /// use rust_aoc_2023::argparser::Arguments;
        ///
        /// // The first argument will be ignored.
        /// let argstrs: Vec<&str> = vec!["", "--time", "--day", "17"];
        /// let args = Arguments::parse(argstrs).unwrap();
        ///
        /// # assert_eq!(args.get_days(), &vec![17]);
        /// # assert_eq!(args.time, true);
        /// # assert_eq!(args.help, false);
        /// ```
        pub fn parse<Iter>(args: Iter) -> Result<Arguments, Box<dyn Error>>
        where
            Iter: IntoIterator,
            Iter::Item: AsRef<str>,
        {
            let mut argi = args.into_iter().peekable();
            // Ignore executable path.
            argi.next();
            let mut days: Vec<u8> = Vec::new();
            let mut help = false;
            let mut time = false;
            let mut parts = (false, false);
            let mut next_days = false;
            let mut next_parts = false;
            let mut last_arg = None;
            while argi.peek().is_some() {
                let next = argi.next().expect("peek returned some, next returned none");
                let mut arg = next.as_ref().trim();
                let argl;

                if next_days {
                    next_days = false;
                    days.extend(Self::parse_int_vals(arg)?);
                    for day in days.iter() {
                        if day == &0 || day > &25 {
                            // TODO Add more descriptive error.
                            return Err(Box::new(UnknownArgError::new(arg, None)));
                        }
                    }
                } else if next_parts {
                    next_parts = false;
                    for part in Self::parse_int_vals(arg)? {
                        if part == 1 {
                            parts.0 = true;
                        } else if part == 2 {
                            parts.1 = true;
                        } else {
                            // TODO Add more descriptive error.
                            return Err(Box::new(UnknownArgError::new(arg, None)));
                        }
                    }
                } else if arg.starts_with("--") {
                    argl = Some(arg.to_lowercase());
                    arg = argl.as_ref().unwrap();
                    if arg == "--help" {
                        help = true;
                    } else if arg == "--time" {
                        time = true;
                    } else if arg == "--day" {
                        next_days = true;
                    } else if arg == "--part" {
                        next_parts = true;
                    } else {
                        return Err(Box::new(UnknownArgError::new(arg, None)));
                    }
                } else if arg.starts_with('-') {
                    let mut chars = arg.chars();
                    chars.next();
                    for (i, c) in chars.enumerate() {
                        if next_days || next_parts {
                            // TODO Add more descriptive error.
                            return Err(Box::new(UnknownArgError::new(arg, None)));
                        }

                        if c == 'h' {
                            help = true;
                        } else if c == 't' {
                            time = true;
                        } else if c == 'd' {
                            next_days = true;
                        } else if c == 'p' {
                            next_parts = true;
                        } else {
                            // TODO Add more descriptive error.
                            return Err(Box::new(UnknownArgError::new(arg, Some(i + 1))));
                        }
                    }
                } else {
                    return Err(Box::new(UnknownArgError::new(arg, None)));
                }

                if argi.peek().is_none() {
                    last_arg = Some(arg.to_owned());
                }
            }

            if (next_days || next_parts) && last_arg.is_some() {
                // TODO Add more descriptive error.
                return Err(Box::new(UnknownArgError::new(last_arg.unwrap(), None)));
            }

            if !parts.0 && !parts.1 {
                parts = (true, true);
            }

            Ok(Arguments {
                days,
                help,
                time,
                parts,
            })
        }

        /// Parses a string argument to a vector of the numbers it represents.
        ///
        /// This function can handle individual numbers, ranges(start-end, both inclusive), and lists(I,J,K).  
        /// As it returns unsigned integers it cannot parse negative numbers.
        ///
        /// # Errors
        ///
        /// This function fails when a number can't be parsed, or a range doesn't have an end value.
        fn parse_int_vals<Str: AsRef<str>>(string: Str) -> Result<Vec<u8>, ParseIntError> {
            let mut result = Vec::new();
            if string.as_ref().contains("-") {
                let elements: Vec<&str> = string.as_ref().split('-').collect();
                if elements.len() == 1 {
                    // TODO Add more descriptive error.
                    return Err(string
                        .as_ref()
                        .parse::<u8>()
                        .err()
                        .expect("parsing string with - should fail"));
                } else if elements.len() == 2 {
                    let start: u8 = elements[0].parse()?;
                    let end: u8 = elements[1].parse()?;
                    if start > end {
                        result.extend((end..=start).rev());
                    } else {
                        result.extend(start..=end);
                    }
                } else {
                    // TODO Add more descriptive error.
                    return Err(string
                        .as_ref()
                        .parse::<u8>()
                        .err()
                        .expect("parsing multiple separated integers should fail"));
                }
            } else if string.as_ref().contains(",") {
                result = string
                    .as_ref()
                    .split(',')
                    .filter(|element| !element.is_empty())
                    .map(|element| element.parse())
                    .collect::<Result<_, _>>()?;
            } else {
                result.push(string.as_ref().parse()?);
            }

            Ok(result)
        }

        /// Returns a reference to the vector of days that should be run.
        ///
        /// # Examples
        ///
        /// ```
        /// use rust_aoc_2023::argparser::Arguments;
        ///
        /// // The first argument will be ignored.
        /// let args = Arguments::parse(["", "--time", "--day", "17", "-d", "9"]).unwrap();
        /// assert_eq!(args.get_days(), &vec![17, 9]);
        /// ```
        pub fn get_days<'a>(&'a self) -> &'a Vec<u8> {
            &self.days
        }

        /// Checks whether the given part should be run.
        ///
        /// Returns `Some(true)` if the part should be run, `Some(false)` if not, and `None` if part is invalid.  
        /// Valid part numbers are only 1 and 2.
        ///
        /// TODO Replace Option with Result
        ///
        /// # Examples
        ///
        /// ```
        /// use rust_aoc_2023::argparser::Arguments;
        ///
        /// // The first argument will be ignored.
        /// let args = Arguments::parse(["", "-d", "5", "-p", "2"]).unwrap();
        ///
        /// assert_eq!(args.run_part(2), Some(true));
        /// assert_eq!(args.run_part(1), Some(false));
        /// assert_eq!(args.run_part(3), None);
        /// ```
        pub fn run_part(&self, part: u8) -> Option<bool> {
            match part {
                1 => Some(self.parts.0),
                2 => Some(self.parts.1),
                _ => None,
            }
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
        /// let error = UnknownArgError::new("--test", None);
        /// assert_eq!(error.to_string(), "Received unknown argument \"--test\"!");
        /// ```
        ///
        /// An UnknonwArgError representing a single character being invalid:
        /// ```
        /// # use rust_aoc_2023::argparser::UnknownArgError;
        /// #
        /// let error = UnknownArgError::new("-xvh", Some(2));
        /// assert_eq!(error.to_string(), "Received unknown argument \"-v\"!");
        /// ```
        ///
        /// An index after the end of the argument means the entire argument will be considered invalid:
        /// ```
        /// # use rust_aoc_2023::argparser::UnknownArgError;
        /// #
        /// let error = UnknownArgError::new("-vht", Some(4));
        /// assert_eq!(error.to_string(), "Received unknown argument \"-vht\"!");
        /// ```
        pub fn new(arg: impl AsRef<str>, mut index: Option<usize>) -> UnknownArgError {
            let arg_str = arg.as_ref().to_owned();
            if let Some(i) = index {
                if i >= arg_str.len() {
                    index = None;
                }
            }
            UnknownArgError {
                arg: arg_str,
                index,
            }
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

    // TODO add UnknonwArgError unit tests.

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::env;
        use std::error::Error;
        use std::hint::black_box;
        use std::num::ParseIntError;

        /// Parse the value of `env::args()`
        #[test]
        fn args_parse_env() -> Result<(), Box<dyn Error>> {
            black_box(Arguments::parse(env::args())?);
            Ok(())
        }

        /// Parse all valid short-args
        #[test]
        fn args_parse_shortargs() -> Result<(), Box<dyn Error>> {
            black_box(Arguments::parse(vec![
                "", "-t", "-h", "-d", "15", "-p", "1",
            ])?);
            Ok(())
        }

        /// Parse all valid long args
        #[test]
        fn args_parse_longargs() -> Result<(), Box<dyn Error>> {
            black_box(Arguments::parse(vec![
                "", "--time", "--help", "--day", "11", "--part", "2",
            ])?);
            Ok(())
        }

        /// Parse mixed short and long args
        #[test]
        fn args_parse_mixed_args() -> Result<(), Box<dyn Error>> {
            black_box(Arguments::parse(["", "-t", "--day", "3"])?);
            Ok(())
        }

        /// Parse multiple short args as a single argument
        #[test]
        fn args_parse_multi_shortarg() -> Result<(), Box<dyn Error>> {
            black_box(Arguments::parse(["", "-td", "2"])?);
            Ok(())
        }

        /// Parse multiple day args
        #[test]
        fn args_parse_multi_day() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                Arguments::parse(["", "-d", "5", "-d", "7", "--day", "13"])?.get_days(),
                &vec![5, 7, 13]
            );
            Ok(())
        }

        /// Parse a range of days as a single arg
        #[test]
        fn args_parse_day_range() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                Arguments::parse(["", "-d", "7-12"])?.get_days(),
                &vec![7, 8, 9, 10, 11, 12]
            );
            Ok(())
        }

        /// Parse a list of days as a single arg
        #[test]
        fn args_parse_day_list() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                Arguments::parse(["", "-t", "--day", "8,9,15"])?.get_days(),
                &vec![8, 9, 15]
            );
            Ok(())
        }

        /// Parse multiple days that aren't correctly ordered
        #[test]
        fn args_parse_days_unordered() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                Arguments::parse(["", "-d", "9", "-d", "11", "-d", "1", "-d", "10"])?.get_days(),
                &vec![9, 11, 1, 10]
            );
            Ok(())
        }

        /// Attempt to parse an unknown shortarg
        #[test]
        fn args_parse_invalid() {
            let args = Arguments::parse(["", "-x"]);
            assert!(matches!(
                args.err().map(|err| err.downcast::<UnknownArgError>()),
                Some(_)
            ));
        }

        /// Attempt to parse day outside valid range
        #[test]
        fn args_parse_day_out_of_range() {
            let args = Arguments::parse(["", "-d", "30"]);
            assert!(matches!(
                args.err().map(|err| err.downcast::<UnknownArgError>()),
                Some(_)
            ));
        }

        /// Attempt to parse an non-integer day
        #[test]
        fn args_parse_nonint_day() {
            let args = Arguments::parse(["", "-d", "f"]);
            assert!(matches!(
                args.err().map(|err| err.downcast::<ParseIntError>()),
                Some(_)
            ));
        }

        /// Attempt to parse a day arg without a parameter
        #[test]
        fn args_parse_day_no_param() {
            let args = Arguments::parse(["", "-d"]);
            assert!(matches!(
                args.err().map(|err| err.downcast::<UnknownArgError>()),
                Some(_)
            ));
        }

        /// Parse a single part to run
        #[test]
        fn args_parse_part() -> Result<(), Box<dyn Error>> {
            let args = Arguments::parse(["", "-p", "1"])?;
            assert_eq!(args.run_part(1), Some(true));
            assert_eq!(args.run_part(2), Some(false));
            Ok(())
        }

        /// Parse a list of parts to run
        #[test]
        fn args_parse_part_list() -> Result<(), Box<dyn Error>> {
            let args = Arguments::parse(["", "-p", "1,2"])?;
            assert_eq!(args.run_part(1), Some(true));
            assert_eq!(args.run_part(2), Some(true));
            Ok(())
        }

        /// Parse a range of parts to run
        #[test]
        fn args_parse_part_range() -> Result<(), Box<dyn Error>> {
            let args = Arguments::parse(["", "-p", "1-2"])?;
            assert_eq!(args.run_part(1), Some(true));
            assert_eq!(args.run_part(2), Some(true));
            Ok(())
        }

        /// Attempt to parse a part outside the valid range
        #[test]
        fn args_parse_part_out_of_range() {
            let args = Arguments::parse(["", "-p", "7"]);
            assert!(matches!(
                args.err().map(|err| err.downcast::<UnknownArgError>()),
                Some(_)
            ));
        }

        /// Parse a single int value
        #[test]
        fn args_parse_ints_single() -> Result<(), ParseIntError> {
            assert_eq!(Arguments::parse_int_vals("7")?, vec![7]);
            Ok(())
        }

        /// Parse a range of ints
        #[test]
        fn args_parse_ints_range() -> Result<(), ParseIntError> {
            assert_eq!(
                Arguments::parse_int_vals("20-29")?,
                vec![20, 21, 22, 23, 24, 25, 26, 27, 28, 29]
            );
            Ok(())
        }

        /// Parse a reverse range
        #[test]
        fn args_parse_ints_range_reverse() -> Result<(), ParseIntError> {
            assert_eq!(
                Arguments::parse_int_vals("17-12")?,
                vec![17, 16, 15, 14, 13, 12]
            );
            Ok(())
        }

        /// Parse a list of ints
        #[test]
        fn args_parse_ints_list() -> Result<(), ParseIntError> {
            assert_eq!(Arguments::parse_int_vals("11,12,16")?, vec![11, 12, 16]);
            Ok(())
        }

        /// Parse an unordered list of ints
        #[test]
        fn args_parse_ints_list_unordered() -> Result<(), ParseIntError> {
            assert_eq!(Arguments::parse_int_vals("19,7,15,3")?, vec![19, 7, 15, 3]);
            Ok(())
        }

        /// Parse a list of ints ending with a comma
        #[test]
        fn args_parse_ints_list_ending_comma() -> Result<(), ParseIntError> {
            assert_eq!(Arguments::parse_int_vals("5,9,12,")?, vec![5, 9, 12]);
            Ok(())
        }

        /// Attempt to parse a string containing a non-numeric character
        #[test]
        fn args_parse_ints_invalid_char() {
            assert!(Arguments::parse_int_vals("15a").is_err());
        }

        /// Attempt to parse a negative number
        #[test]
        fn args_parse_ints_negative_val() {
            assert!(Arguments::parse_int_vals("-7").is_err());
        }

        /// Attempt to parse a range without an end
        #[test]
        fn args_parse_ints_open_range() {
            assert!(Arguments::parse_int_vals("19-").is_err());
        }

        /// Attempt to check for running a part out of range.
        #[test]
        fn args_run_part_out_of_range() -> Result<(), Box<dyn Error>> {
            let args = Arguments::parse(["", "-d", "2"])?;
            assert!(args.run_part(3).is_none());
            Ok(())
        }

        /// Check whether both parts should be run without an explicit parts arg.
        #[test]
        fn args_run_parts_no_parts_arg() -> Result<(), Box<dyn Error>> {
            let args = Arguments::parse(["", "-d", "3"])?;
            assert_eq!(args.run_part(1), Some(true));
            assert_eq!(args.run_part(2), Some(true));
            Ok(())
        }
    }
}

use std::borrow::Borrow;
use std::env;
use std::io;
use std::path::PathBuf;
use std::process;
use std::time::{Duration, Instant};

use argparser::Arguments;

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
        env::current_exe()
            .map(|path| path.to_str().map(|str| str.to_owned()))
            .unwrap_or(Some(String::from("rust_aoc_2023")))
            .as_deref()
            .unwrap_or("rust_aoc_2023")
    );
    println!(" -d --day  <DAY(S)>    Specifies one or more days to run.");
    println!(
        "                       Multiple days can be specified using START-END or DAY,DAY,DAY."
    );
    println!("                       Can be used more than once.");
    println!(" -p --part <PART(S)>   Specifies one or more parts of the days to run. Valid parts are 1 and 2.");
    println!(
        "                       Multiple parts can be specified using START-END or PART,PART."
    );
    println!("                       Can be used more than once.");
    println!("                       Applies to all days. Default is 1,2.");
    println!(" -t --time             Measures and prints the execution time of each part of each executed day.");
    println!(" -h --help             Prints this help text and exits.");
}

/// Run the [`DayRunners`](days::DayRunner) for all the days specified by the commandline arguments.
///
/// This function runs the parts specified in the [`Arguments`] instance of the days from said [`Arguments`] instance.
///
/// It will print the outputs from said days to the console.  
/// If [`Arguments::time`] is `true` it will also print the time the initialization and parts of each day took.  
/// As well as the total time running all days took.
///
/// If a [`DayRunner`] fails, or cannot be found a message will be written to the log.
///
/// # Panics
///
/// This function panics if
/// * a specified day is 0 or greater than 25,
/// * if [`days::init`] wasn't called\*, or
/// * a [`DayRunner`] panics.
///
/// \* Note that [`days::init`] is called by [`days::register_day_runner_constructor`] and [`days::register_day_runner_default`].
///
/// # Examples
///
/// ```
/// use rust_aoc_2023;
/// use rust_aoc_2023::argparser::Arguments;
///
/// // The first string will be ignored.
/// let args = Arguments::parse(["", "-d", "5", "--day", "4-7"]).unwrap();
/// rust_aoc_2023::days::init(true);
/// rust_aoc_2023::run_days_from_args(&args);
/// ```
///
/// [`DayRunner`]: days::DayRunner
pub fn run_days_from_args(args: &Arguments) {
    let start = Instant::now();
    for day in args.get_days() {
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
                match init_result {
                    Err(err) => {
                        eprintln!("Initializing day {} failed!", day);
                        eprintln!("Error: {}", err);
                        continue;
                    }
                    Ok(_) => (),
                }

                if args.time {
                    println!(
                        "Day {} initialization took {}.",
                        day,
                        format_duration(&init_start.elapsed())
                    );
                }

                let run_part_1 = args.run_part(1).expect("part 1 unknown");
                let part_1_failed = run_part_1 && !run_runner_part(&run, *day, 1, args.time);
                if !part_1_failed && args.run_part(2).expect("part 2 unknown") {
                    run_runner_part(&run, *day, 2, args.time);
                }

                if args.time {
                    println!(
                        "Running day {} took {} in total.",
                        day,
                        format_duration(&init_start.elapsed())
                    );
                }
            }

            None => {
                eprintln!("No runner for day {} found!", day);
            }
        };
    }

    if args.time {
        println!(
            "Running all days took a total of {}.",
            format_duration(&start.elapsed())
        );
    }
}

/// Solve a sequence of days.
///
/// This function executes a sequence of [DayRunners](days::DayRunner).  
/// The days will be run in the order the iterator returns them.
///
/// Currently the function prints an error message if something goes wrong.  
/// It also terminates the program if an error occures while trying to get a runner for a day.
///
/// An error message will be printed to the log for every day for which no runner could be found.
///
/// Setting time to `true` will cause this function to print log messages about the execution times.  
/// One such message will be written for the init, part1, and part2 methods of each day.  
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
/// rust_aoc_2023::run_days([1, 7, 4], false);
/// ```
///
/// Timing day executions:
/// ```
/// # use rust_aoc_2023;
/// # use rust_aoc_2023::days;
/// #
/// # days::init(true);
/// rust_aoc_2023::run_days([7, 9, 1], true);
/// ```
#[deprecated = "run_days can't support parts arg, please use run_days_fom_args instead."]
pub fn run_days<Iter>(days: Iter, time: bool)
where
    Iter: IntoIterator,
    Iter::Item: Borrow<u8>,
{
    let start = Instant::now();
    for day in days {
        #[allow(deprecated)]
        run_day(day.borrow().to_owned(), time);
    }

    if time {
        println!(
            "Running all days took a total of {}.",
            format_duration(&start.elapsed())
        );
    }
}

/// Solves a both parts of a single day of the [Advent of Code](https://adventofcode.com/).
///
/// This function executes a single [DayRunner](days::DayRunner).
///
/// This function prints an error message if something goes wrong.  
/// It also terminates the program if an error occures while trying to get a runner for a day.
///
/// An error message will be printed to the log for every day for which no runner could be found.
///
/// Setting time to `true` will cause this function to print log messages about the execution times.  
/// One such message will be written for the init, part1, and part2 methods of the day.
///
/// # Panics
///
/// This function panics if it receives a day that is 0 or greater than 25.  
/// It also panics if [days::init] wasn't called before this function.  
/// Note that [days::init] is automatically called by [days::register_day_runner_constructor] and [days::register_day_runner_default].
///
/// # Examples
///
/// ```
/// use rust_aoc_2023;
/// use rust_aoc_2023::days;
///
/// days::init(true);
/// rust_aoc_2023::run_day(13, false);
/// ```
pub fn run_day(day: u8, time: bool) {
    if day == 0 || day > 25 {
        panic!("Received invalid day {}!", day);
    }

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
            match init_result {
                Err(err) => {
                    eprintln!("Initializing day {} failed!", day);
                    eprintln!("Error: {}", err);
                    return;
                }
                Ok(_) => (),
            }

            if time {
                println!(
                    "Day {} initialization took {}.",
                    day,
                    format_duration(&init_start.elapsed())
                );
            }

            if run_runner_part(&run, day, 1, time) {
                run_runner_part(&run, day, 2, time);
            }

            if time {
                println!(
                    "Running day {} took {} in total.",
                    day,
                    format_duration(&init_start.elapsed())
                );
            }
        }
        None => {
            eprintln!("No runner for day {} found!", day);
        }
    }
}

/// Runs a single part of a single day.
///
/// This function also runs [`DayRunner::init`].  
/// Not running [`DayRunner::init`] before running either of the parts may result in arbitrary crashes.  
/// However [`DayRunner::init`] should not be called twice either, so only use this function if you do not intend to run both parts.  
/// If you wish to run both parts, use [`run_day`] instead.
///
/// This function will print the result of the executed part to the standard output.
///
/// Setting time to `true` will cause this function to print log messages about the execution times.  
/// One such message will be written for the init function and one for the part itself.
///
/// Not finding a [`DayRunner`] for a day will cause a log message to be printed.
///
/// If an error occurs while getting a day runner the programm will be terminated with exit code 2.
///
/// TODO implment better error handling.
///
/// # Panics
///
/// This function panics if day is 0 or greather than 25.  
/// It also panics if part is neither 1 nor 2.  
/// In addition this function panics if [`days::init`] wasn't called before this function.  
/// Note that [`days::init`] is automatically called by [`days::register_day_runner_constructor`] and [`days::register_day_runner_default`].  
/// A day part function may also panic for any or no reason at all.
///
/// # Examples
///
/// Running a a part of a day:
/// ```
/// use rust_aoc_2023;
/// use rust_aoc_2023::days;
///
/// days::init(true);
/// assert!(rust_aoc_2023::run_day_part(7, 1, false));
/// ```
///
/// Attempting to run day 0 part 1:
/// ```should_panic
/// use rust_aoc_2023;
/// use rust_aoc_2023::days;
///
/// days::init(true);
/// rust_aoc_2023::run_day_part(0, 1, false);
/// ```
///
/// Attempting to run part 3 of a day:
/// ```should_panic
/// use rust_aoc_2023;
/// use rust_aoc_2023::days;
///
/// days::init(true);
/// rust_aoc_2023::run_day_part(5, 3, false);
/// ```
///
/// [`DayRunner`]: days::DayRunner
/// [`DayRunner::init`]: days::DayRunner::init
pub fn run_day_part(day: u8, part: u8, time: bool) -> bool {
    if day == 0 || day > 25 {
        panic!("Received invalid day {}!", day);
    }

    if part != 1 && part != 2 {
        panic!("Received invalid part {}!", part);
    }

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
            match init_result {
                Err(err) => {
                    eprintln!("Initializing day {} failed!", day);
                    eprintln!("Error: {}", err);
                    return false;
                }
                Ok(_) => (),
            }

            if time {
                println!(
                    "Day {} initialization took {}.",
                    day,
                    format_duration(&init_start.elapsed())
                );
            }

            run_runner_part(&run, day, part, time)
        }
        None => {
            eprintln!("No runner for day {} found!", day);
            false
        }
    }
}

/// Runs a single part of a [`DayRunner`].
///
/// This function does not run [`DayRunner::init`].  
/// Not running [`DayRunner::init`] before running either of the parts may result in arbitrary crashes.  
/// However [`DayRunner::init`] should not be called twice either.
///
/// This function will print the result of the executed part to the standard output.
///
/// Since a [`DayRunner`] does not know which day it is for, this function also has to be given the day number for logging purposes.
///
/// Setting time to `true` will cause this function to print a log messag containg the execution time.
///
/// # Panics
///
/// This function panics if part is neither 1 nor 2.  
/// A day part function may also panic for any or no reason at all.
///
/// # Examples
///
/// Running both parts of a day:
/// ```
/// use rust_aoc_2023;
/// use rust_aoc_2023::days;
///
/// days::init(true);
/// let mut runner = days::get_day_runner(5).unwrap().unwrap();
/// // run_runner_part will not run init.
/// runner.init();
/// assert!(rust_aoc_2023::run_runner_part(&runner, 7, 1, false));
/// assert!(rust_aoc_2023::run_runner_part(&runner, 7, 2, false));
/// ```
///
/// Attempting to run day 9 part 0:
/// ```should_panic
/// use rust_aoc_2023;
/// use rust_aoc_2023::days;
///
/// days::init(true);
/// let mut runner = days::get_day_runner(9).unwrap().unwrap();
/// runner.init();
/// rust_aoc_2023::run_runner_part(&runner, 9, 0, false);
/// ```
///
/// [`DayRunner`]: days::DayRunner
/// [`DayRunner::init`]: days::DayRunner::init
pub fn run_runner_part(runner: &Box<dyn days::DayRunner>, day: u8, part: u8, time: bool) -> bool {
    let part_start = Instant::now();
    let part_result = match part {
        1 => runner.part1(),
        2 => runner.part2(),
        _ => panic!("Received invalid part {}!", part),
    };

    match part_result {
        Err(err) => {
            eprintln!("Running day {} part {} failed!", day, part);
            eprintln!("Error: {}", err);
            return false;
        }
        Ok(Some(result)) => {
            println!("Day {} part {} result: {}", day, part, result);
        }
        Ok(None) => (),
    }

    if time {
        println!(
            "Day {} part {} took {}",
            day,
            part,
            format_duration(&part_start.elapsed())
        );
    }

    true
}

pub mod days;
