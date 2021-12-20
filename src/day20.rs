use crate::prelude::*;
use bitvec::prelude::*;
use std::{fmt::Display, mem::swap};

day!(20, parse => pt1, pt2);

#[derive(Debug, Clone)]
struct Input {
    pattern: BitVec,
    image: Image,
}

#[derive(Debug, Clone)]
struct Image {
    width: usize,
    data: BitVec,
    infinite_value: bool,
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let height = self.data.len() / self.width;
        let mut s = String::with_capacity((self.width + 1) * height);
        for y in 0..height {
            for x in 0..self.width {
                s.push(if self.data[y * self.width + x] {
                    '#'
                } else {
                    '.'
                });
            }
            s.push('\n');
        }
        s.pop();
        f.write_str(&s)
    }
}

fn enhance_image(pattern: &BitSlice, input: &Image, output: &mut Image) {
    let width = input.width + 2;
    let height = input.data.len() / input.width + 2;

    output.width = width;
    output.data.clear();
    if pattern[0] {
        assert!(!pattern[511]);
        output.infinite_value = !input.infinite_value;
    }
    for y in 0..height {
        let y_sub1 = y.checked_sub(1);
        let y_add1 = if y == height - 1 { None } else { Some(y + 1) };
        for x in 0..width {
            let x_sub1 = x.checked_sub(1);
            let x_add1 = if x == width - 1 { None } else { Some(x + 1) };
            let mut mask = 0;
            let mut set = |idx: usize, value| {
                let is_set = if let Some((x, y)) = value {
                    if x != 0 && x != width - 1 && y != 0 && y != height - 1 {
                        input.data[(y - 1) * input.width + (x - 1)]
                    } else {
                        input.infinite_value
                    }
                } else {
                    input.infinite_value
                };
                if is_set {
                    mask |= 1 << idx;
                }
            };
            set(0, x_add1.and_then(|x| y_add1.map(|y| (x, y))));
            set(1, y_add1.map(|y| (x, y)));
            set(2, x_sub1.and_then(|x| y_add1.map(|y| (x, y))));
            set(3, x_add1.map(|x| (x, y)));
            set(4, Some((x, y)));
            set(5, x_sub1.map(|x| (x, y)));
            set(6, x_add1.and_then(|x| y_sub1.map(|y| (x, y))));
            set(7, y_sub1.map(|y| (x, y)));
            set(8, x_sub1.and_then(|x| y_sub1.map(|y| (x, y))));
            output.data.push(pattern[mask]);
        }
    }
}

// fn enhance_image(pattern: &BitSlice, input: &Image, output: &mut Image) {
//     output.clear();
//     output.extend(input.iter().flat_map(|&pos| box3x3(pos).into_iter()));
//     output.retain(|&pos| {
//         let neighbors = box3x3(pos)
//             .into_iter()
//             .enumerate()
//             .filter(|(_, box_pos)| input.contains(box_pos));
//         let mut mask = 0u16;
//         for (idx, _) in neighbors {
//             mask |= 1 << (8 - idx);
//         }
//         pattern[mask as usize]
//     });
// }

fn enhance_n_times(input: &Input, n: usize) -> usize {
    let mut current_image = input.image.clone();
    let mut new_image = current_image.clone();
    for _ in 0..n {
        enhance_image(&input.pattern, &current_image, &mut new_image);
        swap(&mut current_image, &mut new_image);
    }

    current_image.data.count_ones()
}

fn pt1(input: &Input) -> usize {
    enhance_n_times(input, 2)
}

fn pt2(input: &Input) -> usize {
    enhance_n_times(input, 50)
}

fn parse(input: &str) -> ParseResult<Input> {
    use parsers::{special::grid, *};
    let bit = token(('.', false)).or(token(('#', true)));
    let pattern = bit.clone().repeat_into();
    let image =
        grid(token('\n'), bit, |x, y, v| Some((x, y, v))).map(|(width, data): (usize, BitVec)| {
            Image {
                width,
                data,
                infinite_value: false,
            }
        });

    pattern
        .trailed(token("\n\n"))
        .and(image)
        .map(|(pattern, image)| Input { pattern, image })
        .parse(input)
}

tests! {
    const EXAMPLE: &'static str = "\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    simple_tests!(parse, pt1, pt1_tests, EXAMPLE => 35);
    simple_tests!(parse, pt2, pt2_tests, EXAMPLE => 3351);
}
