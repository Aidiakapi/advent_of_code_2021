use crate::prelude::*;

day!(17, parse => pt1, pt2);

type Int = i32;
type Range = std::ops::Range<Int>;

// Experimentally, for my input, this is actually 64, but I've opted to
// make it 128 just to have a safety margin... Ideally I'd find a way to
// compute a theoretical upper bound.
const MAX_MISSES_IN_A_ROW: usize = 128;

#[derive(Debug, Clone)]
struct Area {
    xs: Range,
    ys: Range,
}

#[derive(Debug, Clone)]
struct XRange {
    // velocity: Int,
    minimum_step_count: Int,
    // exclusive upper bound
    maximum_step_count: Option<Int>,
}

#[derive(Debug, Clone)]
struct YRange {
    velocity: Int,
    step_range: Range,
}

fn sum_all_numbers_up_to(n: Int) -> Int {
    (n * (n + 1)) / 2
}

fn inverse_sum_all_numbers_up_to(sum: Int) -> f64 {
    ((8.0 * sum as f64 + 1.0).sqrt() - 1.0) / 2.0
}

fn inverse_x_pos_after(velocity: Int, x_pos: Int) -> Option<f64> {
    let v = velocity as f64;
    let d = 4.0 * (v * v + v) - 8.0 * x_pos as f64 + 1.0;
    if d <= 0.0 {
        None
    } else {
        Some(d.sqrt() * -0.5 + v + 0.5)
    }
}

fn inverse_y_pos_after(velocity: Int, y_pos: Int) -> Option<f64> {
    let v = velocity as f64;
    let d = 4.0 * (v * v + v) - 8.0 * y_pos as f64 + 1.0;
    if d <= 0.0 {
        None
    } else {
        Some(d.sqrt() * 0.5 + v + 0.5)
    }
}

fn get_valid_x_ranges(target: Range) -> Vec<XRange> {
    assert!(target.start > 0 && target.end > 0);
    let mut ranges = Vec::new();

    // Minimum velocity is where the sum_of_all_numbers_up_to(velocity) is >= target.start
    let minimum_velocity = inverse_sum_all_numbers_up_to(target.start);
    let minimum_velocity = minimum_velocity.ceil() as Int;
    // The maximum velocity can be anything less than target.end, because
    // because (target.end - 1) would reach the box in 1 step.
    for velocity in minimum_velocity..target.end {
        let min_step = inverse_x_pos_after(velocity, target.start).unwrap().ceil() as Int;
        let max_step = inverse_x_pos_after(velocity, target.end - 1).map(|n| n.floor() as Int + 1);
        match max_step {
            Some(n) if n <= min_step => continue,
            _ => {}
        }
        ranges.push(XRange {
            // velocity,
            minimum_step_count: min_step,
            maximum_step_count: max_step,
        })
    }
    ranges
}

fn get_valid_y_ranges(target: Range) -> Vec<YRange> {
    assert!(target.start < 0 && target.end < 0);
    let mut ranges = Vec::new();

    let mut misses_in_a_row = 0;
    let max_y = target.end - 1;
    let min_y = target.start;

    // Anything below the starting position, would mean that in a single step
    // we end up already below where the range starts, after which it just keeps
    // dropping further, so there's no point in checking.
    for velocity in min_y.. {
        let min_steps = inverse_y_pos_after(velocity, max_y).unwrap().ceil() as Int;
        let max_steps = inverse_y_pos_after(velocity, min_y).unwrap().floor() as Int;
        if min_steps <= max_steps {
            misses_in_a_row = 0;
            ranges.push(YRange {
                velocity,
                step_range: min_steps..max_steps + 1,
            });
        } else {
            misses_in_a_row += 1;
            if misses_in_a_row > MAX_MISSES_IN_A_ROW {
                break;
            }
        }
    }

    ranges
}

fn get_valid_ranges(area: &Area) -> (Vec<XRange>, Vec<YRange>) {
    (
        get_valid_x_ranges(area.xs.clone()),
        get_valid_y_ranges(area.ys.clone()),
    )
}

fn is_overlapping(x_range: &XRange, y_range: &YRange) -> bool {
    if x_range.minimum_step_count >= y_range.step_range.end {
        false
    } else if let Some(maximum) = x_range.maximum_step_count {
        y_range.step_range.start < maximum
    } else {
        true
    }
}

fn pt1(area: &Area) -> Int {
    let (x_ranges, y_ranges) = get_valid_ranges(area);
    let max_y_velocity = y_ranges
        .iter()
        .rev()
        .filter(|y_range| {
            x_ranges
                .iter()
                .any(|x_range| is_overlapping(x_range, y_range))
        })
        .next()
        .unwrap()
        .velocity;

    sum_all_numbers_up_to(max_y_velocity)
}

fn pt2(area: &Area) -> usize {
    let (x_ranges, y_ranges) = get_valid_ranges(area);
    x_ranges
        .iter()
        .map(|x_range| {
            y_ranges
                .iter()
                .filter(|&y_range| is_overlapping(x_range, y_range))
                .count()
        })
        .sum::<usize>()
}

fn parse(input: &[u8]) -> ParseResult<Area> {
    use parsers::*;
    let range = number_i32
        .trailed(token(b".."))
        .and(number_i32)
        .map(|(x, y)| x..y + 1);
    token(b"target area: x=")
        .then(range)
        .trailed(token(b", y="))
        .and(range)
        .map(|(xs, ys)| Area { xs, ys })
        .parse(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"target area: x=20..30, y=-10..-5";

    simple_tests!(parse, pt1, pt1_tests, EXAMPLE => 45);
    simple_tests!(parse, pt2, pt2_tests, EXAMPLE => 112);
}
