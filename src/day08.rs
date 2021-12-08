use crate::prelude::*;

day!(8, parse => pt1, pt2);

#[derive(Debug, Clone, Copy, Default, Hash)]
struct Segments(u8);

#[derive(Debug, Clone)]
struct Record {
    measurements: [Segments; 10],
    digits: [Segments; 4],
}

fn pt1(input: &[Record]) -> usize {
    input
        .iter()
        .flat_map(|x| x.digits.iter())
        .filter(|segment| matches!(segment.0.count_ones(), 2 | 3 | 4 | 7))
        .count()
}

fn pt2(input: &[Record]) -> u32 {
    input.iter().map(get_record_value).sum()
}

fn get_record_value(record: &Record) -> u32 {
    let mut found = [0u8; 10];
    for &Segments(measurement) in record.measurements.iter() {
        match measurement.count_ones() {
            2 => found[1] = measurement,
            3 => found[7] = measurement,
            4 => found[4] = measurement,
            7 => found[8] = measurement,
            _ => (),
        }
    }

    for &Segments(measurement) in record.measurements.iter() {
        let and_one = (measurement & found[1]).count_ones();
        let xor_four = (measurement ^ found[4]).count_ones();

        let index = match (measurement.count_ones(), and_one, xor_four) {
            (5, 2, _) => 3,
            (5, _, 5) => 2,
            (5, _, 3) => 5,
            (6, 1, _) => 6,
            (6, _, 4) => 0,
            (6, _, 2) => 9,
            _ => continue,
        };

        found[index] = measurement;
    }

    record.digits
        .iter()
        .map(|&Segments(x)| found.iter().position(|&y| x == y).unwrap())
        .fold(0u32, |res, nr| res * 10 + (nr as u32))
}

fn parse(input: &str) -> ParseResult<Vec<Record>> {
    use parsers::*;
    let segment_digit = #[rustfmt::skip] {
            token(('a', 1_u8 << 0))
        .or(token(('b', 1_u8 << 1)))
        .or(token(('c', 1_u8 << 2)))
        .or(token(('d', 1_u8 << 3)))
        .or(token(('e', 1_u8 << 4)))
        .or(token(('f', 1_u8 << 5)))
        .or(token(('g', 1_u8 << 6)))
    };
    let segment = segment_digit.fold(0, |acc, v| acc | v).map(|x| Segments(x));
    let measurements = segment.clone().trailed(token(' ')).many_n();
    let digits = token(' ').then(segment).many_n();
    let record = measurements
        .trailed(token('|'))
        .and(digits)
        .map(|(measurements, digits)| Record {
            measurements,
            digits,
        });
    record.sep_by(token('\n')).parse(input)
}

tests! {
    const EXAMPLE: &'static str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    simple_tests!(parse, pt1, pt1_tests, EXAMPLE => 26);
    simple_tests!(parse, pt2, pt2_tests, EXAMPLE => 61229);
}
