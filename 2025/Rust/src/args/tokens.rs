//! The token/token stream handling for the argument parser.
//!
//! This module contains the [`TokenStream`] implementation for this argument parse, as well as the token implementation.

use std::fmt::{self, Display, Formatter};
use std::iter::{FusedIterator, Peekable};
use std::vec::IntoIter;

use super::ArgumentError;

/// A single token from the command line arguments.
///
/// Can be either an argument, or an argument value.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(super) enum ArgToken {
    /// The long argument representation of an argument.
    LongArg(String),
    /// The single character representation of an argument.
    ShortArg(char),
    /// A single integer value.
    IntValue(u8),
    /// A list of integers.
    IntListValue(Vec<u8>),
    /// A range of integers.
    IntRangeValue { start: u8, end: u8 },
}

impl Display for ArgToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            &Self::LongArg(ref arg) => write!(f, "--{arg}"),
            &Self::ShortArg(arg) => write!(f, "-{arg}"),
            &Self::IntValue(val) => write!(f, "{val}"),
            &Self::IntListValue(ref list) => write!(
                f,
                "{}",
                list.iter()
                    .map(u8::to_string)
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            &Self::IntRangeValue { start, end } => write!(f, "{start}-{end}"),
        }
    }
}

impl ArgToken {
    /// Checks if this token is an argument token.
    pub fn is_arg(&self) -> bool {
        match self {
            &Self::LongArg(_) | &Self::ShortArg(_) => true,
            &Self::IntValue(_) | &Self::IntListValue(_) | &Self::IntRangeValue { .. } => false,
        }
    }

    /// Checks if this token is a value token.
    pub fn is_val(&self) -> bool {
        match self {
            &Self::LongArg(_) | &Self::ShortArg(_) => false,
            &Self::IntValue(_) | &Self::IntListValue(_) | &Self::IntRangeValue { .. } => true,
        }
    }
}

/// An iterator containing [`ArgTokens`](ArgToken) representing the arguments given to this program.
///
/// Created using [`parse_strings`](Self::parse_strings).
#[derive(Clone, Debug)]
pub(super) struct TokenStream {
    /// The inner iterator containing the actual tokens.
    tokens: Peekable<IntoIter<ArgToken>>,
}

impl TokenStream {
    /// Parse an iterator of strings to a TokenStream.
    ///
    /// # Errors
    ///
    /// This function fails if a string is not a valid token.  
    /// Since this program can't currently handle string values, these are considered invalid arguments.
    pub fn parse_strings<Iter>(strings: Iter) -> Result<TokenStream, ArgumentError>
    where
        Iter: IntoIterator,
        Iter::Item: AsRef<str>,
    {
        let vals = strings.into_iter().collect::<Vec<Iter::Item>>();
        let mut iter = vals.iter().map(|item| item.as_ref());
        let mut current = iter.next();
        let mut tokens: Vec<ArgToken> = Vec::new();
        let mut last_list_ends_comma: bool = false;
        while let Some(string) = current {
            if string.trim_start().starts_with("--") {
                let split: Vec<&str> = string.trim_start().splitn(2, char::is_whitespace).collect();
                if !split.is_empty() {
                    tokens.push(ArgToken::LongArg(split[0].trim_end().to_owned()));
                    if split.len() > 1 {
                        current = Some(split[1]);
                        continue;
                    }
                }
            } else if string.trim_start().starts_with("-") {
                let split: Vec<&str> = string.trim_start().splitn(2, char::is_whitespace).collect();
                if !split.is_empty() {
                    tokens.extend(
                        split[0]
                            .trim_end()
                            .chars()
                            .skip(1)
                            .map(|c| ArgToken::ShortArg(c)),
                    );
                    if split.len() > 1 {
                        current = Some(split[1]);
                        continue;
                    }
                }
            } else if !string.is_empty() {
				// FIXME: Handle commans after a space
                let list_split: Vec<&str> = string.trim().split(",").collect();
                if list_split.len() > 1 || last_list_ends_comma && !string.contains("-") {
                    let mut vals: Vec<u8> = Vec::with_capacity(list_split.len());
                    let iter = list_split
                        .iter()
                        .map(|&val| val.trim())
                        .filter(|&val| !val.is_empty());
                    for val in iter {
                        vals.push(Self::parse_int(val)?);
                    }
                    match tokens.last_mut() {
                        Some(ArgToken::IntListValue(list)) if last_list_ends_comma => {
                            list.append(&mut vals);
                        }
                        _ => {
                            tokens.push(ArgToken::IntListValue(vals));
                        }
                    }
                    if string.trim_end().ends_with(",") {
                        last_list_ends_comma = true;
                    } else {
                        last_list_ends_comma = false;
                    }
                } else {
                    let range_split: Vec<&str> = string.split("-").map(|val| val.trim()).collect();
                    if range_split.len() == 2 {
                        if range_split[0].is_empty() || range_split[1].is_empty() {
                            return Err(ArgumentError::OpenRangeVal);
                        }
                        let start = Self::parse_int(range_split[0])?;
                        let end = Self::parse_int(range_split[1])?;
                        tokens.push(ArgToken::IntRangeValue { start, end });
                    } else if range_split.len() > 2 {
                        return Err(ArgumentError::OpenRangeVal);
                    } else {
                        tokens.push(ArgToken::IntValue(Self::parse_int(string.trim())?));
                    }
                }
            }
            current = iter.next();
        }
        Ok(TokenStream {
            tokens: tokens.into_iter().peekable(),
        })
    }

    /// Parses a single integer from a string.
    fn parse_int(string: &str) -> Result<u8, ArgumentError> {
        string
            .trim()
            .parse()
            .map_err(|error| ArgumentError::InvalidIntVal {
                val: Some(string.to_owned()),
                error,
            })
    }

    /// Checks whether the next [`token`](ArgToken) in this stream is an argument token.
    pub fn next_is_argument(&mut self) -> bool {
        self.tokens.peek().is_some_and(|tok| tok.is_arg())
    }

    /// Checks whether the next [`token`](ArgToken) in this stream is a value token.
    pub fn next_is_value(&mut self) -> bool {
        self.tokens.peek().is_some_and(|tok| tok.is_val())
    }

    /// Consume and return the next token if it is an argument token.
    ///
    /// Otherwise returns ['None'].
    pub fn next_if_argument(&mut self) -> Option<ArgToken> {
        if self.next_is_argument() {
            self.next()
        } else {
            None
        }
    }

    /// Consume and return the next token if it is a value token.
    ///
    /// Otherwise returns ['None'].
    pub fn next_if_value(&mut self) -> Option<ArgToken> {
        if self.next_is_value() {
            self.next()
        } else {
            None
        }
    }

    ///	Returns a reference to the next() value without advancing the iterator.
    ///
    /// Like [`next`](Self::next), if there is a value, it is wrapped in a `Some(T)`. But if the iteration is over, `None` is returned.
    pub fn peek(&mut self) -> Option<&ArgToken> {
        self.tokens.peek()
    }

    /// Parses the next value from this token stream.
    ///
    /// `parse` can parse into any type that implements [`FromTokenStream`].
    ///
    /// # Errors
    ///
    /// Will return the error returned by [`from_stream`](FromTokenStream::from_stream) if parsing a value fails.
    pub fn parse<F: FromTokenStream>(&mut self) -> Result<Option<F>, F::Error> {
        FromTokenStream::from_stream(self)
    }
}

impl Iterator for TokenStream {
    type Item = ArgToken;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.tokens.size_hint()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        // Implement this because advance_by is unstable
        self.tokens.nth(n)
    }
}

impl ExactSizeIterator for TokenStream {}

impl FusedIterator for TokenStream {}

// Don't implement FromIterator because parsing arguments to a TokenStream is fallible.

/// Parse an argument (component) from a [`TokenStream`].
pub(super) trait FromTokenStream: Sized {
    /// The type of errors [`Self::parse`] may return.
    type Error;

    /// Attempt to parse an object from a [`TokenStream`].
    ///
    /// An implementation **SHOULD NOT** consume an [`ArgToken`] if it isn't the correct type.
    ///
    /// Implementations are required to return [`Err`] if parsing fails.  
    /// `Ok(None)` may only be returned if the given [`TokenStream`] is empty.
    fn from_stream(tokens: &mut TokenStream) -> Result<Option<Self>, Self::Error>;
}

#[cfg(test)]
mod test {
    use std::num::IntErrorKind;

    use super::super::ArgumentError;
    use super::{ArgToken, TokenStream};

    /// Asserts that the right expression returns an InvalidIntValue error with a specific IntErrorKind and value string.
    ///
    /// The value string has to be an Option.
    ///
    /// Also accepts an error message.
    macro_rules! assert_int_parse_err {
        ($left:expr, $right_val_str:expr, $right_err_kind:expr $(,)?) => {
            {
                let left_val = $left;
                let right_val_str_val = $right_val_str;
                let right_err_kind_val = $right_err_kind;
                assert!(matches!(
                    &left_val,
                    &Err(ArgumentError::InvalidIntVal { ref val, ref error }) if val.as_ref().map(|val| val.as_str()) == (&right_val_str_val).as_ref().map(|val| val.as_ref()) && error.kind() == &right_err_kind_val,
                ),
"assertion `left == right` failed
  left: [{:?}]
 right: [Err(InvalidIntVal {{ val: {:?}, error: ParseIntError {{ kind: {:?} }} }})]",
                left_val, right_val_str_val, right_err_kind_val);
            }
        };
        ($left:expr, $right_val_str:expr, $right_err_kind:expr, $($arg:tt)+) => {
            {
                assert!(matches!(
                    $left,
                    Err(ArgumentError::InvalidIntVal { ref val, ref error }) if val.as_ref().map(|val| val.as_str()) == (&$right_val_str).as_ref().map(|val| val.as_ref()) && error.kind() == &$right_err_kind
                ), $($arg)+);
            }
        };
    }

    #[test]
    fn long_arg_to_str() {
        assert_eq!(
            ArgToken::LongArg(String::from("test")).to_string(),
            "--test"
        );
    }

    #[test]
    fn short_arg_to_str() {
        assert_eq!(ArgToken::ShortArg('c').to_string(), "-c");
    }

    #[test]
    fn int_val_to_str() {
        assert_eq!(ArgToken::IntValue(17).to_string(), "17");
    }

    #[test]
    fn int_list_val_to_str() {
        assert_eq!(
            ArgToken::IntListValue(vec![1, 6, 4, 7]).to_string(),
            "1,6,4,7"
        );
    }

    #[test]
    fn int_range_val_to_str() {
        assert_eq!(
            ArgToken::IntRangeValue { start: 5, end: 3 }.to_string(),
            "5-3"
        );
    }

    #[test]
    fn parse_long_args_to_token_stream() -> Result<(), ArgumentError> {
        let stream = TokenStream::parse_strings(vec!["--long", "--arg"])?;
        assert_eq!(
            stream.collect::<Vec<ArgToken>>(),
            vec![
                ArgToken::LongArg(String::from("--long")),
                ArgToken::LongArg(String::from("--arg")),
            ]
        );
        Ok(())
    }

    #[test]
    fn parse_short_args_to_token_stream() -> Result<(), ArgumentError> {
        let stream = TokenStream::parse_strings(vec!["-abc", "-d"])?;
        assert_eq!(
            stream.collect::<Vec<ArgToken>>(),
            vec![
                ArgToken::ShortArg('a'),
                ArgToken::ShortArg('b'),
                ArgToken::ShortArg('c'),
                ArgToken::ShortArg('d'),
            ]
        );
        Ok(())
    }

    #[test]
    fn parse_multi_arg_string_to_token_stream() -> Result<(), ArgumentError> {
        let stream = TokenStream::parse_strings(vec!["--long -dc"])?;
        assert_eq!(
            stream.collect::<Vec<ArgToken>>(),
            vec![
                ArgToken::LongArg(String::from("--long")),
                ArgToken::ShortArg('d'),
                ArgToken::ShortArg('c'),
            ]
        );
        Ok(())
    }

    #[test]
    fn parse_int_val_string_to_token_stream() -> Result<(), ArgumentError> {
        let stream = TokenStream::parse_strings(vec!["12"])?;
        assert_eq!(
            stream.collect::<Vec<ArgToken>>(),
            vec![ArgToken::IntValue(12)]
        );
        Ok(())
    }

    #[test]
    fn parse_int_list_val_string_to_token_stream() -> Result<(), ArgumentError> {
        let stream = TokenStream::parse_strings(vec!["123,32, 7,15"])?;
        assert_eq!(
            stream.collect::<Vec<ArgToken>>(),
            vec![ArgToken::IntListValue(vec![123, 32, 7, 15])]
        );
        Ok(())
    }

    #[test]
    fn parse_int_empty() {
        assert_int_parse_err!(TokenStream::parse_int(""), Some(""), IntErrorKind::Empty);
    }

    #[test]
    fn parse_int_negative() {
        assert_int_parse_err!(
            TokenStream::parse_int("-12"),
            Some("-12"),
            IntErrorKind::InvalidDigit
        );
    }

    #[test]
    fn parse_int_invalid() {
        assert_int_parse_err!(
            TokenStream::parse_int("abc"),
            Some("abc"),
            IntErrorKind::InvalidDigit
        );
    }

    #[test]
    fn parse_int_list_one_val_string_to_token_stream() -> Result<(), ArgumentError> {
        let stream = TokenStream::parse_strings(vec!["123,"])?;
        assert_eq!(
            stream.collect::<Vec<ArgToken>>(),
            vec![ArgToken::IntListValue(vec![123])]
        );
        Ok(())
    }

    #[test]
    fn parse_int_range_val_string_to_token_stream() -> Result<(), ArgumentError> {
        let stream = TokenStream::parse_strings(vec!["15-27"])?;
        assert_eq!(
            stream.collect::<Vec<ArgToken>>(),
            vec![ArgToken::IntRangeValue { start: 15, end: 27 }]
        );
        Ok(())
    }

    #[test]
    fn parse_int_range_open_string_to_token_stream() {
        assert_eq!(
            TokenStream::parse_strings(vec!["29-"]).unwrap_err(),
            ArgumentError::OpenRangeVal
        );
    }

    #[test]
    fn parse_arg_val_string_to_token_stream() -> Result<(), ArgumentError> {
        let stream = TokenStream::parse_strings(vec!["--arg 1, 7"])?;
        assert_eq!(
            stream.collect::<Vec<ArgToken>>(),
            vec![
                ArgToken::LongArg(String::from("--arg")),
                ArgToken::IntListValue(vec![1, 7])
            ]
        );
        Ok(())
    }

    #[test]
    fn parse_arg_val_empty_list_to_token_stream() -> Result<(), ArgumentError> {
        let stream = TokenStream::parse_strings(vec![","])?;
        assert_eq!(
            stream.collect::<Vec<ArgToken>>(),
            vec![ArgToken::IntListValue(Vec::new())]
        );
        Ok(())
    }

    #[test]
    fn parse_arg_val_multi_string_list_to_token_stream() -> Result<(), ArgumentError> {
        let stream = TokenStream::parse_strings(vec!["15,", "7,", "31"])?;
        assert_eq!(
            stream.collect::<Vec<ArgToken>>(),
            vec![ArgToken::IntListValue(vec![15, 7, 31])]
        );
        Ok(())
    }

    #[test]
    fn parse_arg_val_int_list_int_to_token_stream() -> Result<(), ArgumentError> {
        let stream = TokenStream::parse_strings(vec!["123,", "9", "53"])?;
        assert_eq!(
            stream.collect::<Vec<ArgToken>>(),
            vec![ArgToken::IntListValue(vec![123, 9]), ArgToken::IntValue(53)]
        );
        Ok(())
    }
}
