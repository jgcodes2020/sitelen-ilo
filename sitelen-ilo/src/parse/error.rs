use std::fmt::Display;

use crate::parse::Span;

/// Custom error type for this parser.
#[derive(Debug)]
pub(crate) struct ParseError<'a> {
    span: Span<'a>,
    reason: String,
    help: Option<String>,
    cause: Option<Box<ParseError<'a>>>,
}
impl<'a> ParseError<'a> {
    /// Create a new [`ParseError`] with a location and reason.
    pub(crate) fn new(span: Span<'a>, reason: impl Into<String>) -> Self {
        Self {
            span,
            reason: reason.into(),
            help: None,
            cause: None,
        }
    }

    /// Create a new [`ParseError`] caused by this error with a location and reason.
    pub(crate) fn chain_up(self, span: Span<'a>, reason: impl Into<String>) -> Self {
        Self {
            span,
            reason: reason.into(),
            help: None,
            cause: Some(Box::new(self)),
        }

    }

    /// Adds a help message to this error.
    pub(crate) fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Wraps this error with [`nom::Err::Error`].
    pub(crate) fn into_error(self) -> nom::Err<Self> {
        nom::Err::Error(self)
    }

    /// Wraps this error with [`nom::Err::Failure`].
    pub(crate) fn into_failure(self) -> nom::Err<Self> {
        nom::Err::Failure(self)
    }

    /// Convenient helper to override the default reason provided by `nom`.
    pub(crate) fn override_reason(reason: impl Into<String>) -> impl FnOnce(nom::Err<Self>) -> nom::Err<Self> {
        |err| err.map(move |mut p_err| {
            p_err.reason = reason.into();
            p_err
        })
    }
}
impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.reason)?;
        if let Some(help) = &self.help {
            f.write_fmt(format_args!(" [help: {}]", help))?;
        }
        Ok(())
    }
}
impl<'a> std::error::Error for ParseError<'a> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        panic!("Error::description() is deprecated, use core::fmt::Display instead")
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}
impl<'a> nom::error::ParseError<Span<'a>> for ParseError<'a> {
    fn from_error_kind(input: Span<'a>, kind: nom::error::ErrorKind) -> Self {
        Self::new(input, kind.description())
    }

    fn append(input: Span<'a>, kind: nom::error::ErrorKind, other: Self) -> Self {
        other.chain_up(input, kind.description())
    }
}

pub(crate) type ParseResult<'a, T> = Result<(Span<'a>, T), nom::Err<ParseError<'a>>>;

pub(crate) type NomErrorKind = nom::error::ErrorKind;

/// Forces parsing errors into failures.
pub(super) fn nom_force_failure<E>(err: nom::Err<E>) -> nom::Err<E> {
    match err {
        nom::Err::Incomplete(_) => err,
        nom::Err::Error(value) => nom::Err::Failure(value),
        nom::Err::Failure(_) => err,
    }
}