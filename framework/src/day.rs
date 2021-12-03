use crate::parsers::{error::Finish, ParseResult};
use anyhow::Result;
use std::{fmt::Display, marker::PhantomData};

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

pub use paste::paste;

#[macro_export]
macro_rules! simple_tests {
    ($parse:expr, $pt:expr, $pt_name:ident, $($input:expr => $expected:expr),+$(,)*) => {
        #[test]
        fn $pt_name() -> ::anyhow::Result<()> {
            $({
                let input = $crate::parsers::error::Finish::finish($parse($input))?;
                let result = $crate::day::IntoResult::into_result($pt(&input))?;
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
        pt1: Result<String>,
        pt2: Result<String>,
    },
}

pub trait Day {
    fn nr(&self) -> u32;
    fn exec(&self, input: &str) -> DayResult;
}

pub auto trait IsNotResult {}
impl<T, E> !IsNotResult for std::result::Result<T, E> {}
pub trait IntoResult {
    type Output;
    fn into_result(self) -> Result<Self::Output, anyhow::Error>;
}

impl<T> IntoResult for Result<T, anyhow::Error> {
    type Output = T;
    fn into_result(self) -> Result<T, anyhow::Error> {
        self
    }
}

impl<T: IsNotResult> IntoResult for T {
    type Output = T;
    fn into_result(self) -> Result<T, anyhow::Error> {
        Ok(self)
    }
}

pub trait IntoDisplayResult {
    fn into_display_result(self) -> Result<String, anyhow::Error>;
}

impl<T> IntoDisplayResult for T
where
    T: IntoResult,
    T::Output: Display,
{
    fn into_display_result(self) -> Result<String, anyhow::Error> {
        self.into_result().map(|x| x.to_string())
    }
}

pub struct DayCommon<P, P1, P2, I, I1, I2, O1, O2>
where
    P: for<'s> Fn(&'s str) -> ParseResult<'s, I>,
    P1: Fn(&I1) -> O1,
    P2: Fn(&I2) -> O2,
    I: AsRef<I1> + AsRef<I2>,
    I1: ?Sized,
    I2: ?Sized,
    O1: IntoDisplayResult,
    O2: IntoDisplayResult,
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
    I: AsRef<I1> + AsRef<I2>,
    I1: ?Sized,
    I2: ?Sized,
    O1: IntoDisplayResult,
    O2: IntoDisplayResult,
{
    fn nr(&self) -> u32 {
        self.nr
    }

    fn exec(&self, input: &str) -> DayResult {
        let input = match (self.parser)(input).finish() {
            Ok(x) => x,
            Err(e) => return DayResult::ParseFailed(e),
        };
        let pt1 = (self.pt1)(input.as_ref());
        let pt2 = (self.pt2)(input.as_ref());
        DayResult::Ran {
            pt1: pt1.into_display_result(),
            pt2: pt2.into_display_result(),
        }
    }
}
