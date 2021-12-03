use anyhow::{anyhow, Result};
use thiserror::Error;

pub type ParseResult<'s, T> = Result<(T, &'s str), (ParseError, &'s str)>;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseError {
    #[error("Empty input")]
    EmptyInput,
    #[error("Expected a digit")]
    ExpectedDigit,
    #[error("Overflow")]
    Overflow,
    #[error("Token does not match")]
    TokenDoesNotMatch,
}

pub trait Finish<T> {
    fn finish(self) -> Result<T>;
}

impl<T> Finish<T> for ParseResult<'_, T> {
    fn finish(self) -> Result<T> {
        match self {
            Ok((x, "" | "\n")) => Ok(x),
            Ok((_, remainder)) => Err(anyhow!("Incomplete, remainder: {remainder}")),
            Err((e, remainder)) => Err(anyhow!("Error: {e}, remainder: {remainder}")),
        }
    }
}
