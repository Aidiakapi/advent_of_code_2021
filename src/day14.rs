use crate::prelude::*;
use itertools::MinMaxResult;
use std::{cell::RefCell, mem::swap};

day!(14, parse => pt1, pt2);

type Molecule = u8;

const MAX_MOLECULE_TYPE_COUNT: usize = 10;

#[derive(Debug, Clone)]
struct Input {
    template: Vec<Molecule>,
    rules: Vec<Rule>,
    molecule_count: usize,
}

#[derive(Debug, Clone, Copy)]
struct Rule {
    lhs: Molecule,
    rhs: Molecule,
    add: Molecule,
}

fn pts(input: &Input, steps: usize) -> Result<SubSubmission<u64>> {
    let overcounted_base = input.molecule_count * input.molecule_count;
    let mut counts = vec![0u64; overcounted_base + input.molecule_count];
    for &[a, b] in input.template.array_windows() {
        counts[a as usize * input.molecule_count + b as usize] += 1;
    }
    for &x in &input.template[1..input.template.len() - 1] {
        counts[overcounted_base + x as usize] += 1;
    }
    let mut new_counts = counts.clone();
    for step_index in 0..steps {
        if step_index != 0 {
            new_counts.copy_from_slice(&counts);
        }
        for rule in &input.rules {
            let i1 = rule.lhs as usize * input.molecule_count + rule.rhs as usize;
            let i2 = rule.lhs as usize * input.molecule_count + rule.add as usize;
            let i3 = rule.add as usize * input.molecule_count + rule.rhs as usize;
            let count = counts[i1];
            new_counts[i1] -= count;
            new_counts[i2] += count;
            new_counts[i3] += count;
            new_counts[overcounted_base + rule.add as usize] += count;
        }
        swap(&mut counts, &mut new_counts);
    }

    let mut total_counts = [0u64; MAX_MOLECULE_TYPE_COUNT];
    for a in 0..input.molecule_count {
        for b in 0..input.molecule_count {
            let count = counts[a as usize * input.molecule_count + b as usize];
            total_counts[a] += count;
            total_counts[b] += count;
        }
    }
    for m in 0..input.molecule_count {
        total_counts[m] -= counts[overcounted_base + m as usize];
    }
    match total_counts
        .iter()
        .take(input.molecule_count)
        .cloned()
        .minmax()
    {
        MinMaxResult::NoElements => Err(anyhow!("no molecules")),
        MinMaxResult::OneElement(_) => Err(anyhow!("min == max")),
        MinMaxResult::MinMax(min, max) => Ok(SubSubmission(max, min)),
    }
}

fn pt1(input: &Input) -> Result<SubSubmission<u64>> {
    pts(input, 10)
}

fn pt2(input: &Input) -> Result<u64> {
    pts(input, 40).map(|SubSubmission(max, min)| max - min)
}

fn parse(input: &[u8]) -> ParseResult<Input> {
    use parsers::*;
    let kind = RefCell::new(Vec::<(u8, u8)>::new());
    let molecule = any().map_res(|c| {
        if matches!(c, b'A'..=b'Z') {
            let mut map = kind.borrow_mut();
            Ok(match map.iter().find(|(x, _)| *x == c) {
                Some((_, idx)) => *idx,
                None => {
                    let len = map.len() as u8;
                    map.push((c, len));
                    len
                }
            })
        } else {
            Err(ParseError::TokenDoesNotMatch)
        }
    });
    let template = molecule.repeat_into();
    let rule = molecule.and(molecule).and(token(b" -> ").then(molecule));
    let rule = rule.map(|((lhs, rhs), add)| Rule { lhs, rhs, add });
    let rules = rule.sep_by(token(b'\n'));
    let parser = template.and(token(b"\n\n").then(rules));
    parser.parse(input).map(|((template, rules), rem)| {
        let molecule_count = kind.borrow().len();
        assert!(molecule_count <= MAX_MOLECULE_TYPE_COUNT);
        (
            Input {
                template,
                rules,
                molecule_count,
            },
            rem,
        )
    })
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    simple_tests!(parse, pt1, pt1_tests, EXAMPLE => SubSubmission(1749, 161));
    simple_tests!(parse, pt2, pt2_tests, EXAMPLE => 2188189693529);
}
