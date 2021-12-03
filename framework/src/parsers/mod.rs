mod combi;
mod common;
pub mod error;
mod multi;

pub use combi::ParserCombiExt;
pub use common::*;
pub use error::{ParseError, ParseResult};
pub use multi::ParserMultiExt;

pub trait Parser {
    type Output;
    fn parse<'s>(&self, input: &'s str) -> ParseResult<'s, Self::Output>;
}
