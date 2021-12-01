use crate::prelude::*;
day!(1, parse => pt1, pt2);

fn parse(input: &str) -> ParseResult<Vec<u32>> {
    use parsers::*;
    sep_by(token('\n'), parse_u32).parse(input)
}

fn pt1(input: &[u32]) -> usize {
    input
        .iter()
        .tuple_windows()
        .filter(|&(&a, &b)| b > a)
        .count()
}

fn pt2(input: &[u32]) -> usize {
    input
        .iter()
        .tuple_windows()
        .map(|(&a, &b, &c)| a + b + c)
        .tuple_windows()
        .filter(|&(a, b)| b > a)
        .count()
}
