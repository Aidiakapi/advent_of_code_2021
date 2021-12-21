use crate::prelude::*;

day!(3, parse => pt1::<12>, pt2::<12>);

fn pt1<const BIT_COUNT: usize>(input: &[usize]) -> MulSubmission<usize> {
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
    MulSubmission(gamma, epsilon)
}

fn pt2<const BIT_COUNT: usize>(input: &[usize]) -> MulSubmission<usize> {
    let mut temp = input.to_vec();
    let oxygen_generator_rating =
        pt2_compute_rating::<BIT_COUNT>(Rating::OxygenGenerator, &mut temp);
    temp.clear();
    temp.extend_from_slice(input);
    let co2_scrubber_rating = pt2_compute_rating::<BIT_COUNT>(Rating::CO2Scrubber, &mut temp);

    MulSubmission(oxygen_generator_rating, co2_scrubber_rating)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rating {
    OxygenGenerator,
    CO2Scrubber,
}

fn pt2_compute_rating<const BIT_COUNT: usize>(rating: Rating, input: &mut Vec<usize>) -> usize {
    for bit in (0..BIT_COUNT).rev() {
        let mut ones_count = 0;
        for &nr in input.iter() {
            ones_count += (nr >> bit) & 1;
        }
        let zeroes_count = input.len() - ones_count;
        let kept_if = (ones_count >= zeroes_count) == (rating == Rating::OxygenGenerator);
        let kept_if = if kept_if { 1 } else { 0 };
        input.retain(|&nr| (nr >> bit) & 1 == kept_if);
        if input.len() == 1 {
            break;
        }
    }
    assert_eq!(1, input.len());
    input[0]
}

fn parse(input: &[u8]) -> ParseResult<Vec<usize>> {
    use parsers::*;
    let bit = token((b'1', 1)).or(token((b'0', 0)));
    let word = bit.fold(0, |nr, bit| (nr << 1) ^ bit);
    word.sep_by(token(b'\n')).parse(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
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

    simple_tests!(parse, pt1::<5>, pt1_tests, EXAMPLE => MulSubmission(22, 9));
    simple_tests!(parse, pt2::<5>, pt2_tests, EXAMPLE => MulSubmission(23, 10));
}
