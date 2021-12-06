use crate::prelude::*;

day!(6, parse => pt1, pt2);

fn pts(input: &[u64], days: usize) -> u64 {
    const CYCLE_LENGTH: usize = 7;
    const BUFFER_SIZE: usize = 9;
    let mut buffer = [0; BUFFER_SIZE];

    for &nr in input {
        buffer[nr as usize] += 1;
    }

    for i in 0..days {
        let newly_spawned = buffer[i % BUFFER_SIZE];
        buffer[(i + CYCLE_LENGTH) % BUFFER_SIZE] += newly_spawned;
    }

    buffer.iter().sum()
}

fn pt1(input: &[u64]) -> u64 {
    pts(input, 80)
}

fn pt2(input: &[u64]) -> u64 {
    pts(input, 256)
}

fn parse(input: &str) -> ParseResult<Vec<u64>> {
    use parsers::*;
    number_u64.sep_by(token(',')).parse(input)
}

tests! {
    const EXAMPLE: &'static str = "3,4,3,1,2";

    simple_tests!(parse, pt1, pt1_tests, EXAMPLE => 5934);
    simple_tests!(parse, pt2, pt2_tests, EXAMPLE => 26984457539);
}
