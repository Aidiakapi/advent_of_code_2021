use crate::prelude::*;
use std::collections::{hash_map::Entry, HashMap};

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

fn count_overlapping_points<'i, I: Iterator<Item = &'i Line> + 'i>(lines: I) -> usize {
    let mut counts = HashMap::new();
    for line in lines {
        for point in line {
            match counts.entry(point) {
                Entry::Occupied(mut s) => *s.get_mut() += 1,
                Entry::Vacant(s) => {
                    s.insert(1);
                }
            }
        }
    }
    counts.values().filter(|&&count| count >= 2).count()
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

fn parse(input: &str) -> ParseResult<Vec<Line>> {
    use parsers::*;
    let coord = number_usize.trailed(token(',')).and(number_usize);
    let coord = coord.map(|(x, y)| (x as isize, y as isize));
    let coord = coord.map(Vec2i::from);
    let line = coord.clone().trailed(token(" -> ")).and(coord);
    let line = line.map(|(from, to)| Line { from, to });
    line.sep_by(token('\n')).parse(input)
}

tests! {
    const EXAMPLE: &'static str = "\
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
