use super::*;

#[derive(Debug, Clone, Copy)]
pub struct SepBy<S, P> {
    separator: S,
    parser: P,
}

impl<S: Parser, P: Parser> Parser for SepBy<S, P> {
    type Output = Vec<P::Output>;

    fn parse<'s>(&self, input: &'s str) -> ParseResult<'s, Self::Output> {
        let (element, mut remainder) = self.parser.parse(input)?;
        let mut elements = Vec::new();
        elements.push(element);
        loop {
            let after_sep = match self.separator.parse(remainder) {
                Ok((_, after_sep)) => after_sep,
                Err(_) => return Ok((elements, remainder)),
            };
            match self.parser.parse(after_sep) {
                Ok((element, after_value)) => {
                    remainder = after_value;
                    elements.push(element);
                }
                Err(_) => return Ok((elements, remainder)),
            };
        }
    }
}

pub fn sep_by<S: Parser, P: Parser>(separator: S, parser: P) -> SepBy<S, P> {
    SepBy { separator, parser }
}
