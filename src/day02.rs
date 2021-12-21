use crate::prelude::*;

day!(2, parse => pt1, pt2);

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

type Instruction = (Direction, u32);

fn pt1(input: &[Instruction]) -> MulSubmission<i32> {
    let mut hpos = 0;
    let mut depth = 0;
    for &(direction, amount) in input {
        let amount = amount as i32;
        match direction {
            Direction::Forward => hpos += amount,
            Direction::Down => depth += amount,
            Direction::Up => depth -= amount,
        }
    }
    MulSubmission(hpos, depth)
}

fn pt2(input: &[Instruction]) -> MulSubmission<i32> {
    let mut aim = 0;
    let mut hpos = 0;
    let mut depth = 0;
    for &(direction, amount) in input {
        let amount = amount as i32;
        match direction {
            Direction::Forward => {
                hpos += amount;
                depth += aim * amount;
            }
            Direction::Down => aim += amount,
            Direction::Up => aim -= amount,
        }
    }
    MulSubmission(hpos, depth)
}

fn parse(input: &[u8]) -> ParseResult<Vec<Instruction>> {
    use parsers::*;
    let direction = token((b"forward ", Direction::Forward))
        .or(token((b"down ", Direction::Down)))
        .or(token((b"up ", Direction::Up)));
    let instruction = direction.and(number::<u32>());
    instruction.sep_by(token(b'\n')).parse(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
forward 5
down 5
forward 8
up 3
down 8
forward 2";

    simple_tests!(parse, pt1, pt1_tests, EXAMPLE => MulSubmission(15, 10));
    simple_tests!(parse, pt2, pt2_tests, EXAMPLE => MulSubmission(15, 60));
}
