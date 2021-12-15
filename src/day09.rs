use crate::prelude::*;
use ahash::AHashSet;
use arrayvec::ArrayVec;

day!(9, parse => pt1, pt2);

fn neighbors(p: Vec2u, input: &[Vec<u8>]) -> ArrayVec<Vec2u, 4> {
    let mut n = ArrayVec::new();
    if p.x != 0 {
        n.push(Vec2u { x: p.x - 1, y: p.y });
    }
    if p.x + 1 != input[p.y].len() {
        n.push(Vec2u { x: p.x + 1, y: p.y });
    }
    if p.y != 0 {
        n.push(Vec2u { x: p.x, y: p.y - 1 });
    }
    if p.y + 1 != input.len() {
        n.push(Vec2u { x: p.x, y: p.y + 1 });
    }
    n
}

fn cells<'a>(input: &'a [Vec<u8>]) -> impl Iterator<Item = (Vec2u, u8)> + 'a {
    input.iter().enumerate().flat_map(|(y, row)| {
        row.iter()
            .enumerate()
            .map(move |(x, &value)| (Vec2 { x, y }, value))
    })
}

fn pt1(input: &[Vec<u8>]) -> u32 {
    cells(input)
        // low points
        .filter(|&(p, nr)| neighbors(p, input).iter().all(|&np| input[np.y][np.x] > nr))
        // risk level
        .map(|(_, nr)| nr as u32 + 1)
        .sum()
}

fn pt2(input: &[Vec<u8>]) -> u32 {
    let mut visited = AHashSet::new();
    let mut basin_sizes = cells(input)
        .filter(|&(p, nr)| neighbors(p, input).iter().all(|&np| input[np.y][np.x] > nr))
        .map(|(p, _)| get_basin_size(p, input, &mut visited))
        .collect::<Vec<_>>();
    basin_sizes.sort_unstable();
    basin_sizes[basin_sizes.len() - 3..].iter().product()
}

fn get_basin_size(p: Vec2u, input: &[Vec<u8>], visited: &mut AHashSet<Vec2u>) -> u32 {
    fn visit(p: Vec2u, input: &[Vec<u8>], visited: &mut AHashSet<Vec2u>, size: &mut u32) {
        if !visited.insert(p) {
            return;
        }
        if input[p.y][p.x] == 9 {
            return;
        }
        *size += 1;
        for neighbor in neighbors(p, input) {
            visit(neighbor, input, visited, size);
        }
    }
    visited.clear();
    let mut size = 0;
    visit(p, input, visited, &mut size);
    size
}

fn parse(input: &str) -> ParseResult<Vec<Vec<u8>>> {
    use parsers::*;
    let input = input.trim_end();
    let row = digit().fold_mut(Vec::new(), |row, nr| row.push(nr));
    row.sep_by(token('\n')).parse(input)
}

tests! {
    const EXAMPLE: &'static str = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

    simple_tests!(parse, pt1, pt1_tests, EXAMPLE => 15);
    simple_tests!(parse, pt2, pt2_tests, EXAMPLE => 1134);
}
