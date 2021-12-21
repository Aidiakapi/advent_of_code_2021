use crate::prelude::*;

day!(1, parse => pt1, pt2);

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

fn parse(input: &[u8]) -> ParseResult<Vec<u32>> {
    use parsers::*;
    number::<u32>().sep_by(token(b'\n')).parse(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
199
200
208
210
200
207
240
269
260
263";

    simple_tests!(parse, pt1, pt1_tests, EXAMPLE => 7);
    simple_tests!(parse, pt2, pt2_tests, EXAMPLE => 5);
}
