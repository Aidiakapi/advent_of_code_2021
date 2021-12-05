use crate::parsers::{error::Finish, ParseResult};
use anyhow::Result;
use colored::Colorize;
use std::{borrow::Borrow, fmt::Display, marker::PhantomData};

#[macro_export]
macro_rules! day {
    ($nr:literal, $parser:expr => $pt1:expr, $pt2:expr) => {
        pub fn day() -> impl Day {
            $crate::day::DayCommon {
                nr: $nr,
                parser: $parser,
                pt1: $pt1,
                pt2: $pt2,
                phantom1: ::std::marker::PhantomData,
                phantom2: ::std::marker::PhantomData,
            }
        }
    };
}

#[macro_export]
macro_rules! tests {
    ($($x:tt)*) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use $crate::simple_tests;

            $($x)*
        }
    };
}

#[macro_export]
macro_rules! simple_tests {
    ($parse:expr, $pt:expr, $pt_name:ident, $($input:expr => $expected:expr),+$(,)*) => {
        #[test]
        fn $pt_name() -> ::anyhow::Result<()> {
            $({
                let input = $crate::parsers::error::Finish::finish($parse($input))?;
                let result = $crate::day::ToResult::to_result($pt(&input))?;
                let expected = $expected;
                if result != expected {
                    return Err(anyhow::anyhow!("Expected: {expected}, but got: {result}"));
                }
            })+
            Ok(())
        }
    };
}

pub enum DayResult {
    NoInput(anyhow::Error),
    ParseFailed(anyhow::Error),
    Ran {
        pt1: Result<ColoredOutput>,
        pt2: Result<ColoredOutput>,
    },
}

pub trait Day {
    fn nr(&self) -> u32;
    fn exec(&self, input: &str) -> DayResult;
}

pub auto trait IsNotResult {}
impl<T, E> !IsNotResult for std::result::Result<T, E> {}
pub trait ToResult {
    type Output;
    fn to_result(self) -> Result<Self::Output, anyhow::Error>;
}

impl<T> ToResult for Result<T, anyhow::Error> {
    type Output = T;
    fn to_result(self) -> Result<T, anyhow::Error> {
        self
    }
}

impl<T: IsNotResult> ToResult for T {
    type Output = T;
    fn to_result(self) -> Result<T, anyhow::Error> {
        Ok(self)
    }
}

pub auto trait AutoImplementToColoredString {}

pub trait ToColoredString {
    fn to_colored(self) -> ColoredOutput;
}

impl<T: Display + AutoImplementToColoredString> ToColoredString for T {
    fn to_colored(self) -> ColoredOutput {
        ColoredOutput {
            str: self.to_string().white().bold().to_string(),
            control_char_count: 11,
        }
    }
}

pub struct ColoredOutput {
    pub str: String,
    pub control_char_count: usize,
}

pub struct DayCommon<P, P1, P2, I, I1, I2, O1, O2>
where
    P: for<'s> Fn(&'s str) -> ParseResult<'s, I>,
    P1: Fn(&I1) -> O1,
    P2: Fn(&I2) -> O2,
    I: Borrow<I1> + Borrow<I2>,
    I1: ?Sized,
    I2: ?Sized,
    O1: ToResult<Output: ToColoredString>,
    O2: ToResult<Output: ToColoredString>,
{
    pub nr: u32,
    pub parser: P,
    pub pt1: P1,
    pub pt2: P2,
    pub phantom1: PhantomData<I1>,
    pub phantom2: PhantomData<I2>,
}

impl<P, P1, P2, I, I1, I2, O1, O2> Day for DayCommon<P, P1, P2, I, I1, I2, O1, O2>
where
    P: for<'s> Fn(&'s str) -> ParseResult<'s, I>,
    P1: Fn(&I1) -> O1,
    P2: Fn(&I2) -> O2,
    I: Borrow<I1> + Borrow<I2>,
    I1: ?Sized,
    I2: ?Sized,
    O1: ToResult<Output: ToColoredString>,
    O2: ToResult<Output: ToColoredString>,
{
    fn nr(&self) -> u32 {
        self.nr
    }

    fn exec(&self, input: &str) -> DayResult {
        let input = match (self.parser)(input).finish() {
            Ok(x) => x,
            Err(e) => return DayResult::ParseFailed(e),
        };
        let pt1 = (self.pt1)(input.borrow());
        let pt2 = (self.pt2)(input.borrow());
        DayResult::Ran {
            pt1: pt1.to_result().map(|x| x.to_colored()),
            pt2: pt2.to_result().map(|x| x.to_colored()),
        }
    }
}
