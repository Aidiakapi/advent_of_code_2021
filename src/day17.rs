use ahash::AHashSet;

use crate::prelude::*;
use std::{cmp::Ordering, ops::ControlFlow};

day!(17, parse => pt1, pt2);

type Int = i32;
type Range = std::ops::Range<Int>;
type Vec2 = framework::vec::Vec2<Int>;

const MAX_MISSES: usize = 1000;

#[derive(Debug, Clone)]
struct Area {
    xs: Range,
    ys: Range,
}

fn sum_all_numbers_up_to(n: Int) -> Int {
    (n * (n + 1)) / 2
}

fn x_after(vel: Int, steps: Int) -> Int {
    let base = sum_all_numbers_up_to(vel);
    if steps >= vel {
        base
    } else {
        base - sum_all_numbers_up_to(vel - steps)
    }
}

fn y_after(vel: Int, steps: Int) -> Int {
    vel * steps - sum_all_numbers_up_to(steps - 1)
}

fn binary_search<F>(range: Range, mut f: F) -> Result<Int, Int>
where
    F: FnMut(Int) -> Ordering,
{
    if range.is_empty() {
        return Err(range.start);
    }
    let mid = (range.end - range.start) / 2 + range.start;
    match f(mid) {
        Ordering::Less => binary_search(range.start..mid, f),
        Ordering::Equal => Ok(mid),
        Ordering::Greater => binary_search((mid + 1)..range.end, f),
    }
}

fn get_minimum_x_velocity(xs: &Range) -> Int {
    let order = (1..20)
        .filter(|&order| sum_all_numbers_up_to(1 << order) >= xs.start)
        .next()
        .unwrap();
    binary_search((1 << (order - 1))..((1 << order) + 1), |probe| {
        xs.start.cmp(&sum_all_numbers_up_to(probe))
    })
    .unwrap_either()
}

fn get_top(vel_y: Int) -> Int {
    sum_all_numbers_up_to(vel_y)
}

fn get_y_intercept(y: Int, ys: &Range) -> Option<Int> {
    let ys_max = (ys.end - 1) as f64;
    let ys_min = ys.start as f64;
    let ys_mid = (ys_max - ys_min) / 2.0 + ys_min;

    // Calculate the positive root of y_after(x, y, steps), or in other
    // words the (non-integral) number of steps after which you reach the
    // mid-point of the target area, on the Y axis.
    let fy = y as f64;
    let y_mid_steps = fy + 0.5 + (fy * fy + fy + ys_mid * -2.0 + 0.25).sqrt();
    let steps = y_mid_steps.floor() as Int;
    if ys.contains(&y_after(y, steps)) {
        Some(steps)
    } else if ys.contains(&y_after(y, steps + 1)) {
        Some(steps + 1)
    } else {
        None
    }
}

fn get_y_step_range(y: Int, ys: &Range, base_step: Int) -> Range {
    let mut min = base_step;
    while ys.contains(&y_after(y, min - 1)) {
        min -= 1;
    }
    let mut max = base_step;
    while ys.contains(&y_after(y, max + 1)) {
        max += 1;
    }
    min..max + 1
}

fn visit_target_points<F>(y: Int, xs: &Range, ys: &Range, base_step: Int, min_x: Int, mut f: F)
where
    F: FnMut(Vec2) -> ControlFlow<()>,
{
    let step_range = get_y_step_range(y, ys, base_step);
    for steps in step_range.clone() {
        for x in min_x.. {
            let x_pos = x_after(x, steps);
            if x_pos < xs.start {
                continue;
            }
            if x_pos >= xs.end {
                break;
            }
            if let ControlFlow::Break(_) = f(Vec2 { x, y }) {
                return;
            }
        }
    }
}

fn pt1(area: &Area) -> Int {
    let min_x_vel = get_minimum_x_velocity(&area.xs);

    let mut y_miss_chain = 0;
    let mut x_miss_chain = 0;
    let mut last_y = 0;
    for y in 1.. {
        let base_step = match get_y_intercept(y, &area.ys) {
            Some(steps) => {
                y_miss_chain = 0;
                steps
            }
            None => {
                y_miss_chain += 1;
                if y_miss_chain >= MAX_MISSES {
                    break;
                }
                continue;
            }
        };

        let mut any = false;
        visit_target_points(y, &area.xs, &area.ys, base_step, min_x_vel, |_| {
            any = true;
            ControlFlow::Break(())
        });
        if any {
            x_miss_chain = 0;
            last_y = y;
        } else {
            x_miss_chain += 1;
            if x_miss_chain >= MAX_MISSES {
                break;
            }
        }
    }

    get_top(last_y)
}

fn pt2(area: &Area) -> usize {
    let min_x_vel = get_minimum_x_velocity(&area.xs);

    let mut y_miss_chain = 0;
    let mut x_miss_chain = 0;

    let mut velocity_set = AHashSet::new();
    for y in -1000.. {
        let base_step = match get_y_intercept(y, &area.ys) {
            Some(steps) => {
                y_miss_chain = 0;
                steps
            }
            None => {
                y_miss_chain += 1;
                if y_miss_chain >= MAX_MISSES {
                    break;
                }
                continue;
            }
        };

        let mut any = false;
        visit_target_points(y, &area.xs, &area.ys, base_step, min_x_vel, |velocity| {
            velocity_set.insert(velocity);
            any = true;
            ControlFlow::Continue(())
        });
        if any {
            x_miss_chain = 0;
        } else {
            x_miss_chain += 1;
            if x_miss_chain >= MAX_MISSES {
                break;
            }
        }
    }

    velocity_set.len()
}

fn parse(input: &str) -> ParseResult<Area> {
    use parsers::*;
    let range = number_i32
        .trailed(token(".."))
        .and(number_i32)
        .map(|(x, y)| x..y + 1);
    token("target area: x=")
        .then(range)
        .trailed(token(", y="))
        .and(range)
        .map(|(xs, ys)| Area { xs, ys })
        .parse(input)
}

tests! {
    const EXAMPLE: &'static str = "target area: x=20..30, y=-10..-5";

    fn position_after(vel: Vec2, steps: Int) -> Vec2 {
        Vec2 {
            x: x_after(vel.x, steps),
            y: y_after(vel.y, steps),
        }
    }

    #[test]
    fn positions() {
        let dir = Vec2 { x: 7, y: 2 };
        assert_eq!(position_after(dir, 0), Vec2 { x: 0, y: 0 });
        assert_eq!(position_after(dir, 1), Vec2 { x: 7, y: 2 });
        assert_eq!(position_after(dir, 2), Vec2 { x: 13, y: 3 });
        assert_eq!(position_after(dir, 3), Vec2 { x: 18, y: 3 });
        assert_eq!(position_after(dir, 4), Vec2 { x: 22, y: 2 });
        assert_eq!(position_after(dir, 5), Vec2 { x: 25, y: 0 });
        assert_eq!(position_after(dir, 6), Vec2 { x: 27, y: -3 });
        assert_eq!(position_after(dir, 7), Vec2 { x: 28, y: -7 });
        assert_eq!(position_after(dir, 8), Vec2 { x: 28, y: -12 });
        assert_eq!(position_after(dir, 9), Vec2 { x: 28, y: -18 });
    }

    simple_tests!(parse, pt1, pt1_tests, EXAMPLE => 45);
    simple_tests!(parse, pt2, pt2_tests, EXAMPLE => 112);
}
