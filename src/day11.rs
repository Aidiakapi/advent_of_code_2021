use crate::prelude::*;
use arrayvec::ArrayVec;

day!(11, parse => pt1, pt2);

const FLASHED: u8 = 1 << 7;
type Grid = Box<[[u8; 10]; 10]>;

fn neighbors(p: Vec2u) -> ArrayVec<Vec2u, 8> {
    let mut n = ArrayVec::new();
    let min_x = p.x.checked_sub(1).unwrap_or(p.x);
    let min_y = p.y.checked_sub(1).unwrap_or(p.y);
    let max_x = if p.x + 1 < 10 { p.x + 1 } else { p.x };
    let max_y = if p.y + 1 < 10 { p.y + 1 } else { p.y };
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            if x != p.x || y != p.y {
                n.push(Vec2 { x, y });
            }
        }
    }
    n
}

fn flood_fill(p: Vec2u, flashed: &mut Vec<Vec2u>, grid: &mut Grid) {
    let cell = &mut grid[p.x][p.y];
    *cell += 1;
    if *cell <= 9 || *cell & FLASHED == FLASHED {
        return;
    }
    *cell |= FLASHED;
    flashed.push(p);
    for n in neighbors(p) {
        flood_fill(n, flashed, grid);
    }
}

fn step(flashed: &mut Vec<Vec2u>, grid: &mut Grid) -> usize {
    debug_assert!(flashed.is_empty());
    for x in 0..10 {
        for y in 0..10 {
            flood_fill(Vec2u { x, y }, flashed, grid);
        }
    }
    let flash_count = flashed.len();
    for cell in flashed.drain(..) {
        grid[cell.x][cell.y] = 0;
    }
    flash_count
}

fn pt1(input: &Grid) -> usize {
    let mut flashed = Vec::new();
    let mut grid = input.clone();
    let mut total_flashes = 0;
    for _ in 0..100 {
        total_flashes += step(&mut flashed, &mut grid);
    }

    total_flashes
}

fn pt2(input: &Grid) -> usize {
    let mut flashed = Vec::new();
    let mut grid = input.clone();
    for step_index in 1.. {
        if step(&mut flashed, &mut grid) == 100 {
            return step_index;
        }
    }
    unreachable!();
}

fn parse(input: &str) -> ParseResult<Grid> {
    use parsers::{special::grid, *};
    grid(token('\n'), digit(), |x, y, v| Some((x, y, v))).parse(input)
}

tests! {
    const EXAMPLE: &'static str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    simple_tests!(parse, pt1, pt1_tests, EXAMPLE => 1656);
    simple_tests!(parse, pt2, pt2_tests, EXAMPLE => 195);
}
