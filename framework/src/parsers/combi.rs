use super::*;

pub trait ParserExt: Sized + Parser {
    fn and<P2: Parser>(self, parser: P2) -> And<Self, P2>;
    fn or<P2: Parser<Output = Self::Output>>(self, parser: P2) -> Or<Self, P2>;
}

pub struct And<P1, P2>(P1, P2);
pub struct Or<P1, P2>(P1, P2);

impl<P1: Parser> ParserExt for P1 {
    fn and<P2: Parser>(self, parser: P2) -> And<P1, P2> {
        And(self, parser)
    }

    fn or<P2: Parser<Output = P1::Output>>(self, parser: P2) -> Or<P1, P2> {
        Or(self, parser)
    }
}

impl<P1: Parser, P2: Parser> Parser for And<P1, P2> {
    type Output = (P1::Output, P2::Output);

    fn parse<'s>(&self, input: &'s str) -> ParseResult<'s, Self::Output> {
        let (o1, remainder) = self.0.parse(input)?;
        let (o2, remainder) = self.1.parse(remainder)?;
        Ok(((o1, o2), remainder))
    }
}

impl<P1: Parser, P2: Parser<Output = P1::Output>> Parser for Or<P1, P2> {
    type Output = P1::Output;

    fn parse<'s>(&self, input: &'s str) -> ParseResult<'s, Self::Output> {
        self.0.parse(input).or_else(|_| self.1.parse(input))
    }
}
