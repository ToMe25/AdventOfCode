//! The errors that can occur in the argument handling of this program.

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::{IntErrorKind, ParseIntError};

/// The argument handling errors.
///
/// This enum contains all the errors the functions in this package can return.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ArgumentError {
    /// The program received an unknown argument.
    UnknownArg(String),
    /// The program tried to parse an empty string as an argument.
    EmptyArg,
    /// The program tried to parse multiple characters as a single short arg.
    InvalidShortArg,
    /// Parsing an integer value failed.
    InvalidIntVal {
        /// The value that couldn't be parsed, if it is known.
        val: Option<String>,
        /// The [`ParseIntError`] describing why the value couldn't be parsed.
        error: ParseIntError,
    },
    /// The program received a range with one end missing.
    OpenRangeVal,
    /// The program received a value without a preceding argument.
    ValueWithoutArg,
    /// An argument that requires a value was not followed by one.
    MissingRequiredValue {
        /// The argument that didn't receive a value.
        arg: String,
    },
    /// An argument was given a value that it couldn't handle.
    InvalidValue {
        /// The value that could not be handled, if it is known.
        val: Option<String>,
    },
}

impl Display for ArgumentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            &Self::UnknownArg(ref arg) => write!(f, "Received unknown argument \"{arg}\"."),
            &Self::EmptyArg => write!(
                f,
                "The program tried to parse an empty string as an argument. This is a bug."
            ),
            &Self::InvalidShortArg => write!(
                f,
                "The program tried to parse multiple characters as a single short arg for some reason. This is a bug."
            ),
            &Self::InvalidIntVal { ref val, ref error } => match (val, error.kind()) {
                (&_, &IntErrorKind::Empty | &IntErrorKind::Zero) => write!(
                    f,
                    "Failed to parse integer value with error \"{error}\". This is a bug."
                ),
                (&Some(ref val), &IntErrorKind::PosOverflow) => write!(
                    f,
                    "This program cannot currently handle integer arguments greater than 255. Received {val}."
                ),
                (&None, &IntErrorKind::PosOverflow) => write!(
                    f,
                    "This program cannot currently handle integer arguments greater than 255."
                ),
                (&Some(ref val), &IntErrorKind::NegOverflow) => write!(
                    f,
                    "This program cannot currently handle negative integer arguments. Received {val}"
                ),
                (&None, &IntErrorKind::NegOverflow) => write!(
                    f,
                    "This program cannot currently handle negative integer arguments."
                ),
                (&Some(ref val), &IntErrorKind::InvalidDigit) => {
                    write!(f, "Received invalid integer value \"{val}\".")
                }
                (&None, &IntErrorKind::InvalidDigit) => {
                    write!(f, "Received invalid integer value.")
                }
                (&_, &_) => write!(f, "Receivec unknown ParseIntError {error}."),
            },
            &Self::OpenRangeVal => write!(
                f,
                "Received a range with an open end. This isn't supported."
            ),
            &Self::ValueWithoutArg => write!(f, "Received a value without a preceding argument."),
            &Self::MissingRequiredValue { ref arg } => {
                write!(f, "The argument \"{arg}\" is missing its required value.")
            }
            &Self::InvalidValue { val: Some(ref val) } => {
                write!(f, "An argument received the invalid value \"{val}\".")
            }
            &Self::InvalidValue { val: None } => {
                write!(f, "An argument received an invalid value.")
            }
        }
    }
}

impl Error for ArgumentError {}

impl From<ParseIntError> for ArgumentError {
    fn from(value: ParseIntError) -> Self {
        Self::InvalidIntVal {
            val: None,
            error: value,
        }
    }
}
