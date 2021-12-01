use thiserror::Error;

#[derive(Error, Debug)]
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
