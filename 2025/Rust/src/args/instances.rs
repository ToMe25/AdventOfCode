//! The implementation of everything related to argument instances.
//!
//! This module contains the implementations of everything required for individual instances of the arguments.  
//!
//! This applies to both the values for arguments, and the argument instance including its value.
//!
//! While there is nothing stopping one from directly instanciating these classes, they are intended to be obtained only via [`parse_arguments`](super::parse_arguments).

use std::fmt::{self, Debug, Display, Formatter};

use super::tokens::{ArgToken, FromTokenStream, TokenStream};
use super::{ArgumentError, ArgumentType};

/// The type of value required by a specific argument.
// TODO encode optional arguments somehow.
// TODO handle string type argument values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ArgumentValueType {
    /// The argument does not require a value.
    None,
    /// The argument requires one or more integers as a value.
    Ints,
}

impl Display for ArgumentValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

/// The value for a single argument.
///
/// Can currently only be either a list of integers or nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentValue {
    /// The argument does not have a value.
    None,
    /// The argument was given one or more integer values.
    Ints(Vec<u8>),
}

impl ArgumentValue {
    /// Returns the integer value represented by this instance, if any.
    ///
    /// Returns [`None`] if the value type isn't [`Ints`](Self::Ints).
    ///
    /// The resulting slice **MAY** by empty.  
    /// One reason for this is that the program received a single comma as a value, which is interpreted as an empty list.
    pub fn get_ints_value<'a>(&'a self) -> Option<&'a [u8]> {
        match self {
            &Self::None => None,
            &Self::Ints(ref val) => Some(val),
        }
    }

    /// Returns the [`ArgumentValueType`] of this ArgumentValue.
    pub(super) fn get_type(&self) -> ArgumentValueType {
        match self {
            &Self::None => ArgumentValueType::None,
            &Self::Ints(_) => ArgumentValueType::Ints,
        }
    }
}

impl FromTokenStream for ArgumentValue {
    type Error = ArgumentError;

    fn from_stream(tokens: &mut TokenStream) -> Result<Option<Self>, Self::Error> {
        if tokens.len() == 0 {
            return Ok(None);
        }
        let token = tokens.next_if_value();
        match token {
            None => Ok(Some(Self::None)),
            Some(ArgToken::IntValue(val)) => Ok(Some(Self::Ints(vec![val]))),
            Some(ArgToken::IntListValue(list)) => Ok(Some(Self::Ints(list))),
            Some(ArgToken::IntRangeValue { start, end }) => {
                if start == end {
                    Ok(Some(Self::Ints(vec![start])))
                } else if start < end {
                    Ok(Some(Self::Ints((start..=end).into_iter().collect())))
                } else {
                    Ok(Some(Self::Ints((end..=start).into_iter().rev().collect())))
                }
            }
            Some(_) => unreachable!("invalid value token"),
        }
    }
}

/// A single instance of a received argument and its associated value.
///
/// These are supposed to be obtained primarily using [`parse_arguments`](super::parse_arguments).
// TODO implement unified value validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentInstance {
    /// The argument represented by this instance.
    pub arg: ArgumentType,
    /// The value given to the argument, if any.
    pub value: ArgumentValue,
}

impl AsRef<ArgumentInstance> for ArgumentInstance {
    fn as_ref(&self) -> &ArgumentInstance {
        self
    }
}

impl AsMut<ArgumentInstance> for ArgumentInstance {
    fn as_mut(&mut self) -> &mut ArgumentInstance {
        self
    }
}

impl FromTokenStream for ArgumentInstance {
    type Error = ArgumentError;

    fn from_stream(tokens: &mut TokenStream) -> Result<Option<Self>, Self::Error> {
        let arg_str: Option<String> = if tokens.next_is_argument() {
            match tokens.peek() {
                Some(ArgToken::LongArg(arg)) => Some(arg.clone()),
                Some(ArgToken::ShortArg(arg)) => Some(arg.to_string()),
                Some(_) => unreachable!("checked as part of next_is_argument"),
                None => unreachable!("checked as part of next_is_argument"),
            }
        } else {
            None
        };

        let arg_t: Option<ArgumentType> = tokens.parse()?;
        if let Some(arg_t) = arg_t {
            let arg_v: Option<ArgumentValue> = tokens.parse()?;
            if let Some(arg_v) = arg_v {
                if arg_t.get_value_type() == arg_v.get_type() {
                    Ok(Some(ArgumentInstance {
                        arg: arg_t,
                        value: arg_v,
                    }))
                } else {
                    // TODO add argument value to error?
                    Err(ArgumentError::InvalidValue { val: None })
                }
            } else if arg_t.get_value_type() == ArgumentValueType::None {
                Ok(Some(ArgumentInstance {
                    arg: arg_t,
                    value: ArgumentValue::None,
                }))
            } else {
                Err(ArgumentError::MissingRequiredValue {
                    arg: arg_str.expect("should be Some if arg_t is Some"),
                })
            }
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod test {
    use super::{ArgumentInstance, ArgumentValue};

    use super::super::tokens::{FromTokenStream, TokenStream};
    use super::super::{ArgumentError, ArgumentType};

    #[test]
    fn parse_int_val() -> Result<(), ArgumentError> {
        let mut stream = TokenStream::parse_strings(vec!["9"])?;
        let value = ArgumentValue::from_stream(&mut stream)?;
        assert_eq!(value, Some(ArgumentValue::Ints(vec![9])));
        Ok(())
    }

    #[test]
    fn parse_int_list_val() -> Result<(), ArgumentError> {
        let mut stream = TokenStream::parse_strings(vec!["15,3,91"])?;
        let value = ArgumentValue::from_stream(&mut stream)?;
        assert_eq!(value, Some(ArgumentValue::Ints(vec![15, 3, 91])));
        Ok(())
    }

    #[test]
    fn parse_int_range_val() -> Result<(), ArgumentError> {
        let mut stream = TokenStream::parse_strings(vec!["9-13"])?;
        let value = ArgumentValue::from_stream(&mut stream)?;
        assert_eq!(value, Some(ArgumentValue::Ints(vec![9, 10, 11, 12, 13])));
        Ok(())
    }

    #[test]
    fn parse_empty_list_val() -> Result<(), ArgumentError> {
        let mut stream = TokenStream::parse_strings(vec![","])?;
        let value = ArgumentValue::from_stream(&mut stream)?;
        assert_eq!(value, Some(ArgumentValue::Ints(Vec::new())));
        Ok(())
    }

    #[test]
    fn parse_single_int_range_val() -> Result<(), ArgumentError> {
        let mut stream = TokenStream::parse_strings(vec!["8-8"])?;
        let value = ArgumentValue::from_stream(&mut stream)?;
        assert_eq!(value, Some(ArgumentValue::Ints(vec![8])));
        Ok(())
    }

    #[test]
    fn parse_reverse_int_range_val() -> Result<(), ArgumentError> {
        let mut stream = TokenStream::parse_strings(vec!["51-48"])?;
        let value = ArgumentValue::from_stream(&mut stream)?;
        assert_eq!(value, Some(ArgumentValue::Ints(vec![51, 50, 49, 48])));
        Ok(())
    }

    #[test]
    fn parse_basic_argument_instance() -> Result<(), ArgumentError> {
        let mut stream = TokenStream::parse_strings(vec!["-d", "6"])?;
        let inst: Option<ArgumentInstance> = stream.parse()?;
        assert_eq!(
            inst,
            Some(ArgumentInstance {
                arg: ArgumentType::Day,
                value: ArgumentValue::Ints(vec![6])
            })
        );
        Ok(())
    }

    #[test]
    fn parse_instance_missing_value() -> Result<(), ArgumentError> {
        let mut stream = TokenStream::parse_strings(vec!["--day"])?;
        let inst = stream.parse::<ArgumentInstance>();
        assert_eq!(
            inst,
            Err(ArgumentError::MissingRequiredValue {
                arg: "--day".to_owned()
            })
        );
        Ok(())
    }
}
