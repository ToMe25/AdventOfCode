//! The arguments enum definition and implementations.
//!
//! The enum has one variant for each possible argument this program can handle.

use std::env;
use std::fmt::{self, Debug, Display, Formatter};
use std::io::{self, Write};
use std::str::FromStr;

use super::ArgumentError;
use super::instances::ArgumentValueType;
use super::tokens::{ArgToken, FromTokenStream, TokenStream};

/// Returns [`Some`] with the given value, or [`None`] if no value is given.
macro_rules! given_or_none {
    () => {
        None
    };
    ($val:expr) => {
        Some($val)
    };
}

/// Generates the implementation of the arguments enum.
macro_rules! arguments_impl {
    ($($(@help $help_str:literal)*
    $variant_name:ident($short_arg:literal,$long_arg:literal,$val_type:ident$(,$val_name:literal)?)),+$(,)?) => {
        /// The possible types of arguments this program can handle.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum ArgumentType {
            $(
                $(
                    #[doc = concat!($help_str, "  ")]
                )*
                $variant_name,
            )+
        }

        impl ArgumentType {
            /// Returns the type of value required by this type of argument.
            pub(super) const fn get_value_type(&self) -> ArgumentValueType {
                match self {
                    $(
                        &Self::$variant_name => ArgumentValueType::$val_type,
                    )+
                }
            }

            /// Returns the argument type for the given short argument.
            ///
            /// # Errors
            ///
            /// This function returns an error of the given short arg isn't known.
            ///
            /// # Examples
            ///
            /// ```
            /// use rust_aoc_2025::args::ArgumentType;
            ///
            /// assert_eq!(ArgumentType::parse_short_arg('v'), Ok(ArgumentType::Verbose));
            /// ```
            pub fn parse_short_arg(short_arg: char) -> Result<Self, ArgumentError> {
                match short_arg {
                    $(
                        $short_arg => Ok(Self::$variant_name),
                    )+
                    _ => Err(ArgumentError::UnknownArg(format!("-{short_arg}")))
                }
            }

            /// Determines the argument type for the given long argument string.
            ///
            /// The string may start with --, but does not have to.
            ///
            /// # Errors
            ///
            /// This function returns an error when the given argument does not exist.
            ///
            /// # Examples
            ///
            /// ```
            /// use rust_aoc_2025::args::ArgumentType;
            ///
            /// assert_eq!(ArgumentType::parse_long_arg("verbose"), Ok(ArgumentType::Verbose));
            /// assert_eq!(ArgumentType::parse_long_arg("--verbose"), Ok(ArgumentType::Verbose));
            /// ```
            pub fn parse_long_arg(long_arg: &str) -> Result<Self, ArgumentError> {
                let mut long_arg = long_arg.trim();
                if long_arg.starts_with("--") {
                    long_arg = long_arg.split_at(2).1;
                }
                if long_arg.is_empty() {
                    return Err(ArgumentError::EmptyArg);
                }
                match long_arg {
                    $(
                        $long_arg => Ok(Self::$variant_name),
                    )+
                    _ => Err(ArgumentError::UnknownArg(format!("--{long_arg}")))
                }
            }

            /// Get the short arg for this argument type.
            ///
            /// Returns the single character argument for this ArgumentType.
            ///
            /// # Examples
            ///
            /// Getting the short arg for the [`Verbose`](Self::Verbose) argument:
            ///
            /// ```
            /// # use rust_aoc_2025::args::ArgumentType;
            /// #
            /// assert_eq!(ArgumentType::Verbose.get_short_arg(), 'v');
            /// ```
            pub const fn get_short_arg(&self) -> char {
                match self {
                    $(
                        &Self::$variant_name => $short_arg,
                    )+
                }
            }

            /// Get the long arg for this argument type.
            ///
            /// Returns the string argument for this ArgumentType.
            pub const fn get_long_arg(&self) -> &'static str {
                match self {
                    $(
                        &Self::$variant_name => $long_arg,
                    )+
                }
            }
        }

        /// Generates the help text for this program, as returned by --help.
        pub fn generate_help_text<Out>(mut out: Out) -> io::Result<()>
        where
            Out: Write,
        {
            // Silently ignore current_exe failing.
            if let Ok(current_exe) = env::current_exe() {
                if let Some(Some(exe_name)) = current_exe.file_name().map(|name| name.to_str()) {
                    println!("Usage: {} [option]... --day <ÐAY(S)>", exe_name);
                }
            }
            // Length of "  -s "
            let mut indent: usize = 5;
            // Length of the longest long arg and value
            indent += [$(
                2 + $long_arg.len() $(
                    + $val_name.len() + 4
                )?
            ),+].iter().max().unwrap_or(&0);
            $(
                write_arg_help(&mut out, $short_arg, $long_arg, given_or_none!($($val_name)?), indent, [$($help_str),*])?;
            )+
            Ok(())
        }
    };
}

/// Writes the help text for a single argument.
///
/// This may result in multiple lines being written to the output.
fn write_arg_help<Out, Iter>(
    mut out: Out,
    short_arg: char,
    long_arg: &str,
    val_name: Option<&str>,
    indent: usize,
    help_lines: Iter,
) -> io::Result<()>
where
    Out: Write,
    Iter: IntoIterator,
    Iter::Item: AsRef<str>,
{
    let mut args = format!("  -{} --{}", short_arg, long_arg);
    if let Some(val_name) = val_name {
        args = format!("{} <{}>", args, val_name);
    }
    write!(out, "{:1$}", args, indent)?;
    // TODO avoid trailing newline?
    let mut first = true;
    for line in help_lines {
        if first {
            first = false;
        } else {
            write!(out, "{:1$}", " ", indent)?;
        }
        writeln!(out, " {}", line.as_ref())?;
    }
    Ok(())
}

impl Display for ArgumentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl TryFrom<char> for ArgumentType {
    type Error = ArgumentError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Self::parse_short_arg(value)
    }
}

impl FromStr for ArgumentType {
    type Err = ArgumentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arg = s.trim();
        if arg.starts_with("--") {
            Self::parse_long_arg(arg.split_at(2).1)
        } else if arg.starts_with("-") {
            if arg.len() == 2 {
                Self::parse_short_arg(arg.chars().next_back().expect("Checked for empty string"))
            } else if arg.len() > 2 {
                Err(ArgumentError::InvalidShortArg)
            } else {
                Err(ArgumentError::EmptyArg)
            }
        } else if arg.len() == 1 {
            Self::parse_short_arg(arg.chars().next().expect("Checked for empty string"))
        } else {
            Self::parse_long_arg(arg)
        }
    }
}

impl FromTokenStream for ArgumentType {
    type Error = ArgumentError;

    fn from_stream(tokens: &mut TokenStream) -> Result<Option<Self>, Self::Error> {
        if tokens.len() == 0 {
            return Ok(None);
        }
        let token = tokens.next_if_argument();
        match token {
            None => Err(ArgumentError::ValueWithoutArg),
            Some(ArgToken::LongArg(ref val)) => Ok(Some(Self::parse_long_arg(val)?)),
            Some(ArgToken::ShortArg(val)) => Ok(Some(Self::parse_short_arg(val)?)),
            Some(_) => unreachable!("invalid argument token"),
        }
    }
}

arguments_impl!(
    @help "Enables additional program output."
    Verbose('v', "verbose", None),
    @help "Measures the time each day, and each part of each day, takes to run."
    Time('t', "time", None),
    @help "Configures which day(s) should be run."
    @help "This argument accepts a single day number, a comma separated list, or a range(start-end, both inclusive)."
    @help "This argument may be supplied multiple times."
    Day('d', "day", Ints, "DAY(S)"),
    @help "Configures which part(s) of the day solutions should be run."
	@help "This argument accepts a single part number, a comma separated list, or a range(start-end, both inclusive)."
	@help "This argument may be supplied multiple times."
    // TODO only apply to the last day argument preceding this.
    Part('p', "part", Ints, "PART(s)"),
    @help "Prints this help text and ends execution."
    Help('h', "help", None),
);

#[cfg(test)]
mod test {
    use super::{ArgumentError, ArgumentType, ArgumentValueType};

    #[test]
    fn get_arg_type_value_type() {
        assert_eq!(
            ArgumentType::Verbose.get_value_type(),
            ArgumentValueType::None
        );
    }

    #[test]
    fn get_arg_type_from_short_arg() {
        assert_eq!(
            ArgumentType::parse_short_arg('v'),
            Ok(ArgumentType::Verbose)
        );
    }

    #[test]
    fn get_arg_type_from_unknown_short_arg() {
        assert_eq!(
            ArgumentType::parse_short_arg('e'),
            Err(ArgumentError::UnknownArg("-e".to_owned()))
        );
    }

    #[test]
    fn get_arg_type_from_long_arg() {
        assert_eq!(
            ArgumentType::parse_long_arg("verbose"),
            Ok(ArgumentType::Verbose)
        );
        assert_eq!(
            ArgumentType::parse_long_arg(" --verbose "),
            Ok(ArgumentType::Verbose)
        );
    }

    #[test]
    fn get_arg_type_from_unknown_long_arg() {
        assert_eq!(
            ArgumentType::parse_long_arg("something"),
            Err(ArgumentError::UnknownArg("--something".to_owned()))
        );
    }

    #[test]
    fn get_arg_type_from_empty_long_arg() {
        assert_eq!(
            ArgumentType::parse_long_arg(""),
            Err(ArgumentError::EmptyArg)
        );
    }

    #[test]
    fn get_arg_type_from_string() {
        assert_eq!("--time".parse::<ArgumentType>(), Ok(ArgumentType::Time));
        assert_eq!("-t".parse::<ArgumentType>(), Ok(ArgumentType::Time));
        assert_eq!("time".parse::<ArgumentType>(), Ok(ArgumentType::Time));
        assert_eq!(" time ".parse::<ArgumentType>(), Ok(ArgumentType::Time));
        assert_eq!("t".parse::<ArgumentType>(), Ok(ArgumentType::Time));
    }

    #[test]
    fn get_arg_type_from_invalid_string() {
        assert_eq!(
            "-time".parse::<ArgumentType>(),
            Err(ArgumentError::InvalidShortArg)
        );
        assert_eq!("".parse::<ArgumentType>(), Err(ArgumentError::EmptyArg));
        assert_eq!("-".parse::<ArgumentType>(), Err(ArgumentError::EmptyArg));
        assert_eq!("--".parse::<ArgumentType>(), Err(ArgumentError::EmptyArg));
    }
}
