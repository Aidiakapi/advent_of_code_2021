use crate::prelude::*;
use ahash::AHashSet;
use std::mem::swap;

day!(13, parse => pt1, pt2);

#[derive(Debug, Clone)]
struct Input {
    points: Vec<Vec2u>,
    folds: Vec<Fold>,
}

type Fold = (Direction, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    X,
    Y,
}

fn fold_paper<I>(points: I, folded: &mut AHashSet<Vec2u>, &(direction, line): &Fold)
where
    I: IntoIterator<Item = Vec2u>,
{
    folded.clear();
    match direction {
        Direction::X => folded.extend(points.into_iter().map(move |p| {
            if p.x < line {
                p
            } else {
                Vec2u {
                    x: line - (p.x - line),
                    y: p.y,
                }
            }
        })),
        Direction::Y => folded.extend(points.into_iter().map(move |p| {
            if p.y < line {
                p
            } else {
                Vec2u {
                    x: p.x,
                    y: line - (p.y - line),
                }
            }
        })),
    }
}

fn paper_to_string(points: &AHashSet<Vec2u>) -> String {
    let w = points.iter().map(|p| p.x).max().unwrap() + 1;
    let h = points.iter().map(|p| p.y).max().unwrap() + 1;
    let mut res = String::with_capacity((w + 1) * h);
    for y in 0..h {
        for x in 0..w {
            res.push(if points.contains(&Vec2u { x, y }) {
                '█'
            } else {
                ' '
            });
        }
        res.push('\n');
    }
    res.pop();
    res
}

fn pt1(input: &Input) -> usize {
    let mut points = AHashSet::with_capacity(input.points.len());
    fold_paper(input.points.iter().cloned(), &mut points, &input.folds[0]);
    points.len()
}

fn pt2(input: &Input) -> String {
    let mut points = AHashSet::from_iter(input.points.iter().cloned());
    let mut temp = AHashSet::with_capacity(input.points.len());
    for fold in &input.folds {
        fold_paper(points.iter().cloned(), &mut temp, fold);
        swap(&mut temp, &mut points);
    }
    crate::ocr::recognize_from_contains(|x, y| points.contains(&Vec2 { x, y }))
        .unwrap_or_else(|| paper_to_string(&points))
}

fn parse(input: &[u8]) -> ParseResult<Input> {
    use parsers::*;
    let point = number::<usize>().and(token(b',').then(number::<usize>()));
    let points = point.map(|(x, y)| Vec2u { x, y }).sep_by(token(b'\n'));

    let direction = token((b'x', Direction::X)).or(token((b'y', Direction::Y)));
    let fold = token(b"fold along ")
        .then(direction)
        .and(token(b'=').then(number::<usize>()));
    let folds = fold.sep_by(token(b'\n'));

    points
        .and(token(b"\n\n").then(folds))
        .map(|(points, folds)| Input { points, folds })
        .parse(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    simple_tests!(parse, pt1, pt1_tests, EXAMPLE => 17);
    // simple_tests!(parse, pt2, pt2_tests, EXAMPLE => 5);
}
