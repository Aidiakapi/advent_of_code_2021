#![allow(incomplete_features)]
#![feature(
    allocator_api,
    associated_type_bounds,
    auto_traits,
    derive_default_enum,
    fn_traits,
    generic_associated_types,
    generic_const_exprs,
    negative_impls,
    never_type,
    trait_alias
)]

pub mod array;
pub mod day;
mod inputs;
pub mod parsers;
pub mod prelude;
pub mod submissions;
pub mod vec;

use std::{collections::HashSet, time::Duration};

use anyhow::Result;
use colored::{ColoredString, Colorize};
use inputs::Inputs;

use day::{Day, DayResult};

use crate::day::ColoredOutput;

#[macro_export]
macro_rules! main {
    ($($day:tt,)*) => {
        $(mod $day;)*

        pub fn main() {
            framework::run(&[
                $(&$day::day(),)*
            ])
        }
    };
}

pub fn run(days: &[&dyn Day]) {
    println!(
        "\n🎄 {} {} {} {} 🎄\n",
        "Advent".bright_red().bold(),
        "of".bright_green(),
        "Code".blue().bold(),
        "2021".white().bold()
    );

    let args = std::env::args().collect::<Vec<String>>();
    let is_bench = args.iter().any(|x| x == "--bench");
    let specific_days = args
        .iter()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<HashSet<u32>>();

    let mut inputs = Inputs::new();
    for &day in days {
        if !specific_days.is_empty() && !specific_days.contains(&day.nr()) {
            continue;
        }
        if is_bench {
            bench_day(&mut inputs, day);
        } else {
            exec_day(&mut inputs, day);
        }
    }
    println!();
}

fn bench_day(inputs: &mut Inputs, day: &dyn Day) {
    let day_nr = day.nr();
    print!(
        "{} {}",
        "Day".bright_blue(),
        day_nr.to_string().bright_red().bold()
    );
    let day_nr = day.nr();
    let result = inputs.get(day_nr).and_then(|input| day.exec_bench(&input));
    let timings = match result {
        Ok(x) => x,
        Err(e) => {
            println!(" :: {}", format!("error: {}", e).bright_red());
            return;
        }
    };

    fn print_timing(label: &'static str, duration: Duration) {
        print!(" :: {} {: >14}", label.bright_green(), format!("{duration:?}").white().bold());
    }

    print_timing("parse", timings.parse);
    print_timing("pt1", timings.pt1);
    print_timing("pt2", timings.pt2);
    println!();
}

fn exec_day(inputs: &mut Inputs, day: &dyn Day) {
    let day_nr = day.nr();
    print!(
        "{} {}",
        "Day".bright_blue(),
        day_nr.to_string().bright_red().bold()
    );

    let result = match inputs.get(day_nr) {
        Ok(input) => day.exec(&input),
        Err(e) => DayResult::NoInput(e),
    };
    fn err_to_str(e: anyhow::Error) -> ColoredOutput {
        ColoredOutput {
            str: e.to_string().red().bold().to_string(),
            control_char_count: 11,
        }
    }
    fn fmt_output(result: Result<ColoredOutput>) -> ColoredOutput {
        result.unwrap_or_else(err_to_str)
    }
    let (pt1_key, pt1_value, pt2_value) = match result {
        DayResult::NoInput(e) => ("no input".bright_red(), err_to_str(e), None),
        DayResult::ParseFailed(e) => ("parse error".bright_red(), err_to_str(e), None),
        DayResult::Ran { pt1, pt2 } => {
            ("pt1".bright_green(), fmt_output(pt1), Some(fmt_output(pt2)))
        }
    };
    let contains_newlines = pt1_value.str.contains('\n')
        || if let Some(v) = &pt2_value {
            v.str.contains('\n')
        } else {
            false
        };
    const COLUMN_WIDTH: usize = 85;
    const OVERHEAD_WIDTH: usize = 21;
    const PT_WIDHT: usize = (COLUMN_WIDTH - OVERHEAD_WIDTH) / 2;
    if contains_newlines {
        fn print_key(key: &ColoredString) {
            let remaining_space = COLUMN_WIDTH - key.len() - 2;
            println!(
                "{:-<before$} {key} {:-<after$}",
                "",
                "",
                before = (remaining_space + 1) / 2,
                after = remaining_space / 2,
            );
        }
        println!();
        print_key(&pt1_key);
        println!("{}", pt1_value.str);
        if let Some(pt2_value) = pt2_value {
            print_key(&"pt2".bright_green());
            println!("{}", pt2_value.str);
        }
        println!("{:-<width$}", "", width = COLUMN_WIDTH);
    } else {
        print!(
            " :: {} {: >width$}",
            pt1_key,
            pt1_value.str,
            width = PT_WIDHT + pt1_value.control_char_count,
        );
        if let Some(pt2_value) = pt2_value {
            print!(
                " :: {} {: >width$}",
                "pt2".bright_green(),
                pt2_value.str,
                width = PT_WIDHT + pt2_value.control_char_count,
            );
        }
        println!();
    }
}

pub fn get_input(day_nr: u32) -> Result<String> {
    Inputs::new().get(day_nr)
}
