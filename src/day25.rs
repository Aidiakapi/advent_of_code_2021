use crate::prelude::*;

day!(25, parse => pt1, pt2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Cell {
    #[default]
    Empty,
    Right,
    Down,
}
type DynGrid = parsers::special::DynGrid<Cell>;

fn pt1(input: &DynGrid) -> usize {
    let width = input.width;
    let height = input.data.len() / width;
    let mut current = input.data.clone();
    let mut pending_moves = Vec::new();

    let mut step_count = 0;
    loop {
        for y in 0..height {
            let base = y * width;
            for x in 0..width {
                if current[base + x] == Cell::Right
                    && current[base + (x + 1) % width] == Cell::Empty
                {
                    pending_moves.push((x, y));
                }
            }
        }

        let horizontal_moves = pending_moves.len();
        for (x, y) in pending_moves.drain(..) {
            current[y * width + x] = Cell::Empty;
            current[y * width + (x + 1) % width] = Cell::Right;
        }

        for y in 0..height {
            let base = y * width;
            let next = (y + 1) % height * width;
            for x in 0..width {
                if current[base + x] == Cell::Down && current[next + x] == Cell::Empty {
                    pending_moves.push((x, y));
                }
            }
        }

        let vertical_moves = pending_moves.len();
        for (x, y) in pending_moves.drain(..) {
            current[y * width + x] = Cell::Empty;
            current[(y + 1) % height * width + x] = Cell::Down;
        }

        step_count += 1;
        if vertical_moves + horizontal_moves == 0 {
            break step_count;
        }
    }
}

fn pt2(_: &DynGrid) -> &'static str {
    "gg"
}

fn parse(input: &[u8]) -> ParseResult<DynGrid> {
    use parsers::{special::grid, *};
    let cell = any().map_res(|c| match c {
        b'.' => Ok(Cell::Empty),
        b'>' => Ok(Cell::Right),
        b'v' => Ok(Cell::Down),
        _ => Err(ParseError::UnexpectedChar),
    });
    grid(token(b'\n'), cell, |x, y, v| Some((x, y, v))).parse(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    simple_tests!(parse, pt1, pt1_tests, EXAMPLE => 58);
    // simple_tests!(parse, pt2, pt2_tests, EXAMPLE => 5);
}
