use crate::prelude::*;
day!(2, parse => pt1, pt2);

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

type Instruction = (Direction, u32);

fn parse(input: &str) -> ParseResult<Vec<Instruction>> {
    use parsers::*;
    let direction = token(("forward ", Direction::Forward))
        .or(token(("down ", Direction::Down)))
        .or(token(("up ", Direction::Up)));
    let instruction = direction.and(parse_u32);
    sep_by(token('\n'), instruction).parse(input)
}

fn pt1(input: &[Instruction]) -> i32 {
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
    hpos * depth
}

fn pt2(input: &[Instruction]) -> i32 {
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
    hpos * depth
}
