use anyhow::{anyhow, Result};
use thiserror::Error;

pub type ParseResult<'s, T> = Result<(T, &'s str), (ParseError, &'s str)>;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseError {
    #[error("empty input")]
    EmptyInput,
    #[error("expected a digit")]
    ExpectedDigit,
    #[error("overflow")]
    Overflow,
    #[error("token does not match")]
    TokenDoesNotMatch,
    #[error("unexpected char")]
    UnexpectedChar,
    #[error("grid cell out of range, x: {0}, y: {0}")]
    GridCellOutOfRange(usize, usize),
    #[error("expected a grid cell")]
    ExpectedGridCell,
    #[error("{0}")]
    Custom(&'static str),
}

pub trait Finish<T> {
    fn finish(self) -> Result<T>;
}

impl<T> Finish<T> for ParseResult<'_, T> {
    fn finish(self) -> Result<T> {
        match self {
            Ok((x, "" | "\n")) => Ok(x),
            Ok((_, remainder)) => Err(anyhow!("incomplete, remainder: \"{remainder}\"")),
            Err((e, remainder)) => Err(anyhow!("{e}, remainder: \"{remainder}\"")),
        }
    }
}
