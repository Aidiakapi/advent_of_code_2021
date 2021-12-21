use crate::prelude::*;

day!(7, parse => pt1, pt2);

fn pt1(input: &[u32]) -> SubmissionContext<u32, u32> {
    let input = {
        let mut v = input.to_owned();
        v.sort_unstable();
        v
    };
    let mut current_pos = input[0];
    let mut fuel_cost = { input.iter().sum::<u32>() - current_pos * (input.len() as u32) };
    let mut index = 0u32;
    loop {
        let new_pos = current_pos + 1;
        while input[index as usize] < new_pos {
            index += 1;
        }
        let new_fuel_cost = fuel_cost + index - (input.len() as u32 - index);
        if fuel_cost < new_fuel_cost {
            return SubmissionContext(current_pos, fuel_cost);
        }
        current_pos = new_pos;
        fuel_cost = new_fuel_cost;
    }
}

fn pt2(input: &[u32]) -> SubmissionContext<u32, u32> {
    let (mut min, mut max) = input.iter().cloned().minmax().into_option().unwrap();
    max += 1;
    loop {
        let mid = (max - min) / 2 + min;
        let score_low = get_score(input, mid - 1);
        let score_curr = get_score(input, mid);
        if score_low < score_curr {
            max = mid;
            continue;
        }
        let score_high = get_score(input, mid + 1);
        if score_curr < score_high {
            return SubmissionContext(mid, score_curr);
        }
        min = mid + 1;
    }
}

fn get_score(input: &[u32], position: u32) -> u32 {
    input
        .iter()
        .map(|&point| {
            let dist = position.abs_diff(point);
            // sum range 0..dist:
            (dist * (dist + 1)) / 2
        })
        .sum()
}

fn parse(input: &[u8]) -> ParseResult<Vec<u32>> {
    use parsers::*;
    number_u32.sep_by(token(b',')).parse(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"16,1,2,0,4,2,7,1,2,14";

    simple_tests!(parse, pt1, pt1_tests, EXAMPLE => SubmissionContext(2, 37));
    simple_tests!(parse, pt2, pt2_tests, EXAMPLE => SubmissionContext(5, 168));
}
