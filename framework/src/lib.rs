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

use anyhow::Result;
use colored::Colorize;
use inputs::Inputs;

use day::{Day, DayResult};

#[macro_export]
macro_rules! main {
    ($($day:tt,)*) => {
        $(mod $day;)*

        pub fn main() {
            framework::run(&[
                $(
                    &$day::day(),
                )*
            ])
        }
    };
}

pub fn run(days: &[&dyn Day]) {
    println!(
        "{} of {} {}\n",
        "Advent".red().bold(),
        "Code".blue().bold(),
        "2021".bold()
    );

    let mut inputs = Inputs::new();
    for &day in days {
        let day_nr = day.nr();
        print!(
            "{} {}",
            "Day".bright_green(),
            day_nr.to_string().bright_red().bold()
        );
        fn print_result(prefix: &str, result: Result<String>) {
            print!(
                " :: {prefix} {: >40}",
                match result {
                    Ok(x) => x,
                    Err(e) => e.to_string().red().bold().to_string(),
                }
            );
        }

        let result = match inputs.get(day_nr) {
            Ok(input) => day.exec(&input),
            Err(e) => DayResult::NoInput(e),
        };
        match result {
            DayResult::NoInput(e) => print_result("no input", Err(e)),
            DayResult::ParseFailed(e) => print_result("parse error", Err(e)),
            DayResult::Ran { pt1, pt2 } => {
                print_result("pt1", pt1);
                print_result("pt2", pt2);
            }
        }
        println!();
    }
}

pub fn get_input(day_nr: u32) -> Result<String> {
    Inputs::new().get(day_nr)
}
