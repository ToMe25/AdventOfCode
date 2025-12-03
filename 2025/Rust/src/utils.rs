//! Some general utilities for this project.

use std::path::PathBuf;
use std::time::Duration;
use std::{env, io};

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
/// use rust_aoc_2025::utils;
///
/// assert_eq!(&utils::format_duration(&Duration::new(17413, 61234700)), "4h 50m 13s 61ms 234µs");
/// assert_eq!(&utils::format_duration(&Duration::new(41, 21000000)), "41s 21ms 0µs");
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

/// Gets the absolute path of the input file for the given day.
///
/// This function determines the absolute path of the input file to read for the given day.  
/// The resulting path is the file `DayX.txt` where X is the given day, in the input directory.  
/// The input directory is assumed to be in the parent directory of the directory in which the program is being executed.
///
/// # Errors
///
/// This function returns an [`io::Error`] if the current directory, the input directory,
/// or any parent directory doesn't exist.  
/// Also if any of the parent directories are a file, or an unreadable symlink.  
/// It will not return an error if the input file does not exist.
///
/// # Examples
///
/// ```
/// # use std::path::PathBuf;
/// use rust_aoc_2025::utils;
///
/// let input = utils::get_input_file_path(15).unwrap();
/// # let mut path: PathBuf = ["..", "input"].iter().collect();
/// # path = path.canonicalize().unwrap();
/// # path.push("Day15.txt");
/// # assert_eq!(input, path);
/// ```
pub fn get_input_file_path(day: u8) -> Result<PathBuf, io::Error> {
    let mut input = env::current_dir()?.canonicalize()?;
    if let Some(path) = input.parent() {
        input = path.to_owned();
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "current working directory doesn't have a parent",
        ));
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
