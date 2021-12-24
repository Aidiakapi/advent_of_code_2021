use crate::prelude::*;
use framework::astar::astar_no_path;

day!(15, parse::<100> => pt1::<100>, pt2::<100>);

type Grid<const N: usize> = Box<[[u8; N]; N]>;

type Cost = u32;
fn pt1<const N: usize>(input: &Grid<N>) -> Cost
where
    [(); N * N]:,
{
    astar_no_path(
        Vec2u::default(),
        |p: &Vec2u, neighbors| {
            let mut add_pos = |x: usize, y: usize| {
                neighbors.push((Vec2u { x, y }, input[x][y] as Cost));
            };
            if p.x != 0 {
                add_pos(p.x - 1, p.y);
            }
            if p.y != 0 {
                add_pos(p.x, p.y - 1);
            }
            if p.x != N - 1 {
                add_pos(p.x + 1, p.y);
            }
            if p.y != N - 1 {
                add_pos(p.x, p.y + 1);
            }
        },
        |p: &Vec2u| (p.x.abs_diff(N - 1) + p.y.abs_diff(N - 1)) as Cost,
        |p: &Vec2u| p.x == N - 1 && p.y == N - 1,
    )
    .unwrap()
}

fn pt2<const N: usize>(input: &Grid<N>) -> Cost
where
    [(); N * N]:,
    [(); (N * 5) * (N * 5)]:,
{
    use parsers::special::GridSpec;
    let mut full_grid = <Grid<{ N * 5 }> as GridSpec<u8>>::initialize();
    for x in 0..N {
        for y in 0..N {
            full_grid[x][y] = input[x][y];
        }
    }
    fn inc(n: u8) -> u8 {
        if n == 9 {
            1
        } else {
            n + 1
        }
    }
    for x in 0..N {
        for y in 0..N * 4 {
            full_grid[x][y + N] = inc(full_grid[x][y]);
        }
    }
    for x in 0..N * 4 {
        for y in 0..N * 5 {
            full_grid[x + N][y] = inc(full_grid[x][y]);
        }
    }

    pt1::<{ N * 5 }>(&full_grid)
}

fn parse<const N: usize>(input: &[u8]) -> ParseResult<Grid<N>>
where
    [(); N * N]:,
{
    use parsers::{special::grid, *};
    grid(token(b'\n'), digit(), |x, y, v| Some((x, y, v))).parse(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    simple_tests!(parse::<10>, pt1::<10>, pt1_tests, EXAMPLE => 40);
    simple_tests!(parse::<10>, pt2::<10>, pt2_tests, EXAMPLE => 315);
}
