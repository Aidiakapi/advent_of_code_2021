mod combi;
mod common;
pub mod error;
mod multi;
pub mod special;

pub use combi::ParserCombiExt;
pub use common::*;
pub use error::{ParseError, ParseResult};
pub use multi::{ParserMultiExt, take_while};

pub trait Parser {
    type Output<'s>;
    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>>;
}
