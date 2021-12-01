mod combi;
mod common;
mod error;
mod multi;

pub use combi::*;
pub use common::*;
pub use error::ParseError;
pub use multi::*;

pub type ParseResult<'s, T> = Result<(T, &'s str), (ParseError, &'s str)>;
// pub trait Parser<'s, T> = Fn(&'s str) -> ParseResult<'s, T>;

pub trait Parser {
    type Output;
    fn parse<'s>(&self, input: &'s str) -> ParseResult<'s, Self::Output>;
}
