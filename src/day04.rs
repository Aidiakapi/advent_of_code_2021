use std::collections::HashSet;

use crate::prelude::*;
day!(4, parse => pt1, pt2);

const WIDTH: usize = 5;
const HEIGHT: usize = 5;
type BingoBoard = Box<[[u32; HEIGHT]; WIDTH]>;

#[derive(Debug, Clone)]
struct Input {
    numbers: Vec<u32>,
    bingo_boards: Vec<BingoBoard>,
}

fn is_bingo(board: &BingoBoard, seen_digits: &HashSet<u32>) -> bool {
    let any_row = || {
        (0..HEIGHT).any(|y| {
            (0..WIDTH)
                .map(|x| board[x][y])
                .all(|nr| seen_digits.contains(&nr))
        })
    };
    let any_column = || {
        (0..WIDTH).any(|x| {
            (0..HEIGHT)
                .map(|y| board[x][y])
                .all(|nr| seen_digits.contains(&nr))
        })
    };
    any_row() || any_column()
}

fn get_unmarked_sum(board: &BingoBoard, seen_digits: &HashSet<u32>) -> u32 {
    board
        .iter()
        .flat_map(|column| column.iter())
        .filter(|nr| !seen_digits.contains(nr))
        .sum()
}

fn pt1(input: &Input) -> Result<MulSubmission<u32>> {
    let mut seen_digits = HashSet::with_capacity(input.numbers.len());
    for &number in &input.numbers {
        seen_digits.insert(number);

        let board = match input
            .bingo_boards
            .iter()
            .filter(|board| is_bingo(board, &seen_digits))
            .next()
        {
            Some(x) => x,
            None => continue,
        };

        return Ok(MulSubmission(get_unmarked_sum(board, &seen_digits), number));
    }

    Err(anyhow!("no bingo"))
}

fn pt2(input: &Input) -> Result<MulSubmission<u32>> {
    let mut remaining_boards = input.bingo_boards.clone();

    let mut seen_digits = HashSet::with_capacity(input.numbers.len());
    for &number in &input.numbers {
        seen_digits.insert(number);

        if let [last_board] = remaining_boards.as_slice() {
            return Ok(MulSubmission(get_unmarked_sum(last_board, &seen_digits), number));
        } else {
            remaining_boards.retain(|board| !is_bingo(board, &seen_digits));
        }
    }

    Err(anyhow!(
        "no solution, remaining boards: {}",
        remaining_boards.len()
    ))
}

fn parse(input: &str) -> ParseResult<Input> {
    use parsers::{special::grid, *};
    let numbers = number_u32.sep_by(token(',')).trailed(token("\n\n"));
    let bingo_digit = token("  ").or(token(' ')).opt().then(number_u32);
    let bingo_board = grid(token('\n'), bingo_digit, |x, y, value| Some((x, y, value)));
    let bingo_boards = bingo_board.sep_by(token('\n'));
    let parser = numbers
        .and(bingo_boards)
        .map(|(numbers, bingo_boards)| Input {
            numbers,
            bingo_boards,
        });
    parser.parse(input)
}

tests! {
    const EXAMPLE: &'static str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
    simple_tests!(parse, pt1, pt1_tests, EXAMPLE => MulSubmission(188, 24));
    simple_tests!(parse, pt2, pt2_tests, EXAMPLE => MulSubmission(148, 13));
}
