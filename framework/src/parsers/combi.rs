use super::*;

pub trait ParserCombiExt: Sized + Parser {
    /// Evaluates two parsers sequentially, and returns a tuple of their outputs
    fn and<P2: Parser>(self, parser: P2) -> And<Self, P2> {
        And(self, parser)
    }
    /// Evaluates two parsers sequentially, returns the output of the second
    fn then<P2: Parser>(self, parser: P2) -> Then<Self, P2> {
        Then(self, parser)
    }
    /// Evaluates two parsers sequentially, returns the output of the first
    fn trailed<P2: Parser>(self, parser: P2) -> Trailed<Self, P2> {
        Trailed(self, parser)
    }

    /// Attempts the first parser, and upon failure attempts the second parser
    fn or<'s, P2: Parser<Output<'s> = Self::Output<'s>>>(self, parser: P2) -> Or<Self, P2> {
        Or(self, parser)
    }

    /// Takes the output of one parser, and transforms it into another type
    fn map<T, F: Fn(Self::Output<'_>) -> T>(self, f: F) -> Map<Self, F> {
        Map(self, f)
    }
    /// Takes the output of one parser, and transforms it into a `Result` of another type
    fn map_res<T, F: Fn(Self::Output<'_>) -> Result<T, ParseError>>(self, f: F) -> MapRes<Self, F> {
        MapRes(self, f)
    }

    /// Attempts to apply this parser, upon success, wraps the value in Some,
    /// upon failure, succeeds with value None and no input consumed.
    fn opt(self) -> Opt<Self> {
        Opt(self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct And<P1, P2>(P1, P2);
#[derive(Debug, Clone, Copy)]
pub struct Then<P1, P2>(P1, P2);
#[derive(Debug, Clone, Copy)]
pub struct Trailed<P1, P2>(P1, P2);

#[derive(Debug, Clone, Copy)]
pub struct Or<P1, P2>(P1, P2);

#[derive(Debug, Clone, Copy)]
pub struct Map<P, F>(P, F);
#[derive(Debug, Clone, Copy)]
pub struct MapRes<P, F>(P, F);

#[derive(Debug, Clone, Copy)]
pub struct Opt<P>(P);

impl<P1: Parser> ParserCombiExt for P1 {}

impl<P1: Parser, P2: Parser> Parser for And<P1, P2> {
    type Output<'s> = (P1::Output<'s>, P2::Output<'s>);

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
        let (o1, remainder) = self.0.parse(input)?;
        let (o2, remainder) = self.1.parse(remainder)?;
        Ok(((o1, o2), remainder))
    }
}

impl<P1: Parser, P2: Parser> Parser for Then<P1, P2> {
    type Output<'s> = P2::Output<'s>;

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
        let (_, remainder) = self.0.parse(input)?;
        self.1.parse(remainder)
    }
}

impl<P1: Parser, P2: Parser> Parser for Trailed<P1, P2> {
    type Output<'s> = P1::Output<'s>;

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
        let (output, remainder) = self.0.parse(input)?;
        let (_, remainder) = self.1.parse(remainder)?;
        Ok((output, remainder))
    }
}

impl<P1: Parser, P2: for<'s> Parser<Output<'s> = P1::Output<'s>>> Parser for Or<P1, P2> {
    type Output<'s> = P1::Output<'s>;

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
        self.0.parse(input).or_else(|_| self.1.parse(input))
    }
}

impl<P: Parser, T, F: for<'s> Fn(P::Output<'s>) -> T> Parser for Map<P, F> {
    type Output<'s> = T;

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, T> {
        self.0
            .parse(input)
            .map(|(value, remainder)| ((self.1)(value), remainder))
    }
}

impl<P: Parser, T, F: for<'s> Fn(P::Output<'s>) -> Result<T, ParseError>> Parser for MapRes<P, F> {
    type Output<'s> = T;

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, T> {
        self.0
            .parse(input)
            .and_then(|(value, remainder)| match (self.1)(value) {
                Ok(value) => Ok((value, remainder)),
                Err(err) => Err((err, input)),
            })
    }
}

impl<P: Parser> Parser for Opt<P> {
    type Output<'s> = Option<P::Output<'s>>;

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
        Ok(match self.0.parse(input) {
            Ok((value, remainder)) => (Some(value), remainder),
            _ => (None, input),
        })
    }
}
