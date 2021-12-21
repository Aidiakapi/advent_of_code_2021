use std::{
    fmt::{Display, Write},
    ops::Range,
};

use crate::prelude::*;

day!(18, parse => pt1, pt2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Part {
    OpenBracket,
    Separator,
    CloseBracket,
    Number(u8),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct SnailfishNr {
    parts: Vec<Part>,
}

impl Display for SnailfishNr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for part in &self.parts {
            match part {
                Part::OpenBracket => f.write_char('['),
                Part::Separator => f.write_char(','),
                Part::CloseBracket => f.write_char(']'),
                Part::Number(nr) => nr.fmt(f),
            }?;
        }
        Ok(())
    }
}

impl SnailfishNr {
    fn add_with(&mut self, rhs: &Self) {
        self.parts.reserve(rhs.parts.len() + 3);
        self.parts.insert(0, Part::OpenBracket);
        self.parts.push(Part::Separator);
        self.parts.extend_from_slice(&rhs.parts);
        self.parts.push(Part::CloseBracket);
    }

    fn increment(&mut self, index: usize, by: u8) {
        if let Part::Number(nr) = &mut self.parts[index] {
            *nr += by;
        } else {
            unreachable!();
        }
    }

    fn explode_apply(
        &mut self,
        lhs: u8,
        rhs: u8,
        pair_range: Range<usize>,
        left_index: Option<usize>,
        right_index: Option<usize>,
    ) {
        if let Some(index) = left_index {
            self.increment(index, lhs);
        }
        if let Some(index) = right_index {
            self.increment(index, rhs);
        }
        self.parts.drain(pair_range.start + 1..pair_range.end);
        self.parts[pair_range.start] = Part::Number(0);
    }

    /// Returns `true` if there were any reductions to make
    fn try_explode(&mut self) -> bool {
        let mut nesting_depth = 0;
        let mut left_index = None;
        let mut iter = self.parts.iter().enumerate();
        while let Some((idx, &part)) = iter.next() {
            match part {
                Part::OpenBracket => nesting_depth += 1,
                Part::Separator => {}
                Part::CloseBracket => nesting_depth -= 1,
                Part::Number(lhs) if nesting_depth > 4 => {
                    assert!(matches!(iter.next(), Some((_, Part::Separator))));
                    let rhs = if let Some((_, &Part::Number(rhs))) = iter.next() {
                        rhs
                    } else {
                        unreachable!();
                    };
                    assert!(matches!(iter.next(), Some((_, Part::CloseBracket))));
                    let right_index = iter
                        .filter(|&(_, part)| matches!(part, Part::Number(_)))
                        .next()
                        .map(|(idx, _)| idx);

                    self.explode_apply(lhs, rhs, idx - 1..idx + 4, left_index, right_index);
                    return true;
                }
                Part::Number(_) => left_index = Some(idx),
            }
        }
        false
    }

    fn try_split(&mut self) -> bool {
        for (idx, &part) in self.parts.iter().enumerate() {
            let nr = match part {
                Part::Number(nr) if nr >= 10 => nr,
                _ => continue,
            };

            let new_parts = [
                Part::OpenBracket,
                Part::Number(nr / 2),
                Part::Separator,
                Part::Number((nr + 1) / 2),
                Part::CloseBracket,
            ];
            self.parts.splice(idx..idx + 1, new_parts);
            return true;
        }
        false
    }

    fn reduce(&mut self) {
        loop {
            if self.try_explode() {
                continue;
            }
            if self.try_split() {
                continue;
            }
            break;
        }
    }

    fn magnitude(&self) -> u64 {
        fn pop_and_get(iter: &mut std::slice::Iter<Part>) -> u64 {
            match iter.next() {
                Some(Part::OpenBracket) => {
                    let lhs = pop_and_get(iter);
                    assert!(matches!(iter.next(), Some(Part::Separator)));
                    let rhs = pop_and_get(iter);
                    assert!(matches!(iter.next(), Some(Part::CloseBracket)));
                    lhs as u64 * 3 + rhs as u64 * 2
                }
                Some(Part::Number(x)) => *x as u64,
                _ => unreachable!(),
            }
        }
        pop_and_get(&mut self.parts.iter())
    }
}

fn pt1(input: &[SnailfishNr]) -> u64 {
    let mut nr = input[0].clone();
    for next in &input[1..] {
        nr.add_with(next);
        nr.reduce();
    }
    nr.magnitude()
}

fn pt2(input: &[SnailfishNr]) -> u64 {
    let mut temp = SnailfishNr::default();
    input
        .iter()
        .enumerate()
        .flat_map(|(idx_a, a)| {
            input
                .iter()
                .enumerate()
                .filter(move |(idx_b, _)| idx_a != *idx_b)
                .map(move |(_, b)| (a, b))
        })
        .map(|(a, b)| {
            temp.parts.clear();
            temp.parts.push(Part::OpenBracket);
            temp.parts.extend_from_slice(&a.parts);
            temp.parts.push(Part::Separator);
            temp.parts.extend_from_slice(&b.parts);
            temp.parts.push(Part::CloseBracket);
            temp.reduce();
            temp.magnitude()
        })
        .max()
        .unwrap()
}

fn parse(input: &[u8]) -> ParseResult<Vec<SnailfishNr>> {
    use parsers::*;
    let part = token((b'[', Part::OpenBracket))
        .or(token((b',', Part::Separator)))
        .or(token((b']', Part::CloseBracket)))
        .or(number::<u8>().map(Part::Number));
    part.repeat_into()
        .map(|parts| SnailfishNr { parts })
        .sep_by(token(b'\n'))
        .parse(input)
}

tests! {
    fn reduce_nr(input: &[SnailfishNr]) -> String {
        assert_eq!(1, input.len());
        let mut item = input[0].clone();
        item.reduce();
        item.to_string()
    }

    const EXAMPLE: &'static [u8] = b"\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    simple_tests!(parse, reduce_nr, reduce_tests,
            "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]" => "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
    );
    simple_tests!(parse, pt1, pt1_tests,
        "[[1,2],[[3,4],5]]" => 143,
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]" => 1384,
        "[[[[1,1],[2,2]],[3,3]],[4,4]]" => 445,
        "[[[[3,0],[5,3]],[4,4]],[5,5]]" => 791,
        "[[[[5,0],[7,4]],[5,5]],[6,6]]" => 1137,
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]" => 3488,
        EXAMPLE => 4140,
    );
    simple_tests!(parse, pt2, pt2_tests, EXAMPLE => 3993);
}
