use crate::prelude::*;

day!(5, parse => pt1, pt2);

#[derive(Debug, Clone, Copy)]
struct Line {
    from: Vec2i,
    to: Vec2i,
}

#[derive(Debug, Clone)]
struct LineIter {
    current: Vec2i,
    offset: Vec2i,
    remainder: usize,
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }

    fn is_horizontal(&self) -> bool {
        self.from.y == self.to.y
    }
}

impl IntoIterator for &'_ Line {
    type Item = Vec2i;
    type IntoIter = LineIter;

    fn into_iter(self) -> Self::IntoIter {
        let delta = self.to - self.from;
        let remainder = delta.x.abs().max(delta.y.abs());
        LineIter {
            current: self.from,
            offset: (delta.x.signum(), delta.y.signum()).into(),
            remainder: remainder as usize + 1,
        }
    }
}

impl Iterator for LineIter {
    type Item = Vec2i;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remainder > 0 {
            self.remainder -= 1;
            let value = self.current;
            self.current += self.offset;
            Some(value)
        } else {
            None
        }
    }
}

fn count_overlapping_points<'i, I: Iterator<Item = &'i Line> + Clone + 'i>(lines: I) -> usize {
    let board_size: Vec2u = lines
        .clone()
        .flat_map(|line| [line.from, line.to])
        .fold((0, 0), |(x, y), coord| {
            (x.max(coord.x as usize), y.max(coord.y as usize))
        })
        .into();
    let board_size = board_size + 1;
    let mut board = vec![0; board_size.x * board_size.y];

    for line in lines {
        for point in line {
            board[point.x as usize * board_size.y + point.y as usize] += 1;
        }
    }
    board.iter().filter(|&&count| count >= 2).count()
}

fn pt1(input: &[Line]) -> usize {
    count_overlapping_points(
        input
            .iter()
            .filter(|x| x.is_horizontal() || x.is_vertical()),
    )
}

fn pt2(input: &[Line]) -> usize {
    count_overlapping_points(input.iter())
}

fn parse(input: &[u8]) -> ParseResult<Vec<Line>> {
    use parsers::*;
    let coord = number_usize.trailed(token(b',')).and(number_usize);
    let coord = coord.map(|(x, y)| Vec2i::from((x as isize, y as isize)));
    let line = coord.trailed(token(b" -> ")).and(coord);
    let line = line.map(|(from, to)| Line { from, to });
    line.sep_by(token(b'\n')).parse(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    simple_tests!(parse, pt1, pt1_tests, EXAMPLE => 5);
    simple_tests!(parse, pt2, pt2_tests, EXAMPLE => 12);
}
