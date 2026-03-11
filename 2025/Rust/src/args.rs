//! The argument handling for this year of my [Advent of Code](https://adventofcode.com) solutions.
//!
//! This module exists to provide an easier-to-use representation of the arguments the program received.
//!
//! # Examples
//!
//! Parsing the program arguments:
//!
//! ```
//! # use rust_aoc_2025::args;
//! #
//! use std::env;
//!
//! // Ignore the executable path, else the parsing will fail.
//! let args = args::parse_arguments(env::args().skip(1));
//! ```
//!
//! Parsing a list of strings:
//!
//! ```
//! # use rust_aoc_2025::args::{self, ArgumentType};
//! # use rust_aoc_2025::args::instances::{ArgumentInstance, ArgumentValue};
//! #
//! let args = args::parse_arguments(vec!["-td", "13"]);
//! assert_eq!(args, Ok(vec![
//!     ArgumentInstance {arg: ArgumentType::Time, value: ArgumentValue::None},
//!     ArgumentInstance {arg: ArgumentType::Day, value: ArgumentValue::Ints(vec![13])},
//! ]));

mod arguments_enum;
mod error;
pub mod instances;
mod tokens;

pub use arguments_enum::{ArgumentType, generate_help_text};
pub use error::ArgumentError;

use instances::ArgumentInstance;
use tokens::TokenStream;

// TODO use a type alias to allow changing the integer value type later.

/// Parses a list of strings to a [`Vec`] of [`ArgumentInstances`](ArgumentInstance).
///
/// **NOTE:** This function does not ignore the first value from the [`Iterator`], so remember to skip it when using it with [`env::args()`](std::env::args).
///
/// # Errors
///
/// This function returns an error if  
///  * an unknown argument was received
///  * an argument was missing its value
///  * a value without an argument was received
///  * an invalid value was received.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use rust_aoc_2025::args::{self, ArgumentType};
/// # use rust_aoc_2025::args::instances::{ArgumentInstance, ArgumentValue};
/// #
/// let args = args::parse_arguments(vec!["-v", "--day", "15"]);
/// assert_eq!(args, Ok(vec![
///     ArgumentInstance {arg: ArgumentType::Verbose, value: ArgumentValue::None},
///     ArgumentInstance {arg: ArgumentType::Day, value: ArgumentValue::Ints(vec![15])},
/// ]));
/// ```
pub fn parse_arguments<Iter>(strings: Iter) -> Result<Vec<ArgumentInstance>, ArgumentError>
where
    Iter: IntoIterator,
    Iter::Item: AsRef<str>,
{
    let mut ts = TokenStream::parse_strings(strings)?;
    let mut args = Vec::new();
    while let Some(inst) = ts.parse()? {
        args.push(inst);
    }
    Ok(args)
}

/// Checks whether the given list of arguments contains an argument of the specified type.
///
/// This function does not check how many of said arguments exist, nor what values they have, if any.
///
/// This function also does not check whether the value of said argument is valid.
///
/// # Examples
///
/// ```
/// # use rust_aoc_2025::args::{self, ArgumentError, ArgumentType};
/// #
/// let args = args::parse_arguments(vec!["-thd", "12"])?;
/// assert!(args::has_argument(args, ArgumentType::Time));
/// #
/// # Ok::<(), ArgumentError>(())
/// ```
pub fn has_argument<Args>(args: Args, arg: ArgumentType) -> bool
where
    Args: IntoIterator,
	Args::Item: AsRef<ArgumentInstance>
{
    args.into_iter().any(|inst| inst.as_ref().arg == arg)
}
