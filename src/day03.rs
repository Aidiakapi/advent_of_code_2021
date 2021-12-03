use crate::prelude::*;
day!(3, parse => pt1::<12>, pt2::<12>);

fn pt1<const BIT_COUNT: usize>(input: &[usize]) -> usize {
    let mut bit_counts = [0; BIT_COUNT];

    for &nr in input {
        for bit in 0..BIT_COUNT {
            bit_counts[bit] += (nr >> bit) & 1;
        }
    }

    let half_len = input.len() / 2;
    let gamma = bit_counts
        .into_iter()
        .map(|count| count > half_len)
        .enumerate()
        .fold(0, |gamma, (index, bit)| {
            gamma | (if bit { 1 } else { 0 } << index)
        });

    let epsilon = gamma ^ ((1 << BIT_COUNT) - 1);
    gamma * epsilon
}

fn pt2<const BIT_COUNT: usize>(input: &[usize]) -> usize {
    let mut temp = input.to_vec();
    let oxygen_generator_rating = pt2_compute_rating::<BIT_COUNT>(Rating::OxygenGenerator, &mut temp);
    temp.clear();
    temp.extend_from_slice(input);
    let co2_scrubber_rating = pt2_compute_rating::<BIT_COUNT>(Rating::CO2Scrubber, &mut temp);

    oxygen_generator_rating * co2_scrubber_rating
}

fn parse(input: &str) -> ParseResult<Vec<usize>> {
    use parsers::*;
    let bit = token(('1', 1)).or(token(('0', 0)));
    let word = bit.fold(0, |nr, bit| (nr << 1) ^ bit);
    word.sep_by(token('\n')).parse(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rating {
    OxygenGenerator,
    CO2Scrubber,
}

fn pt2_compute_rating<const BIT_COUNT: usize>(rating: Rating, input: &mut Vec<usize>) -> usize {
    for bit in (0..BIT_COUNT).rev() {
        let mut one_count = 0;
        for &nr in input.iter() {
            one_count += (nr >> bit) & 1;
        }
        let zero_count = input.len() - one_count;
        let target_bit = if (one_count >= zero_count) == (rating == Rating::OxygenGenerator) {
            1
        } else {
            0
        };
        input.drain_filter(|&mut nr| (nr >> bit) & 1 != target_bit);
        if input.len() == 1 {
            break;
        }
    }
    assert_eq!(1, input.len());
    input[0]
}

tests! {
    const EXAMPLE: &'static str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    simple_tests!(parse, pt1::<5>, pt1_tests, EXAMPLE => 198);
    simple_tests!(parse, pt2::<5>, pt2_tests, EXAMPLE => 230);
}
