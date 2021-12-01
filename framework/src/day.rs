use crate::parsers::ParseResult;
use anyhow::{anyhow, Result};
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
    fn into_result(self) -> Result<String, anyhow::Error>;
}

impl<T: Display> IntoResult for Result<T, anyhow::Error> {
    fn into_result(self) -> Result<String, anyhow::Error> {
        self.map(|x| x.to_string())
    }
}

impl<T: IsNotResult + Display> IntoResult for T {
    fn into_result(self) -> Result<String, anyhow::Error> {
        Ok(self.to_string())
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
    O1: IntoResult,
    O2: IntoResult,
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
    O1: IntoResult,
    O2: IntoResult,
{
    fn nr(&self) -> u32 {
        self.nr
    }

    fn exec(&self, input: &str) -> DayResult {
        let input = match (self.parser)(input) {
            Ok((x, "" | "\n")) => x,
            Ok((_, remainder)) => {
                return DayResult::ParseFailed(anyhow!("Incomplete, remainder: {remainder}"))
            }
            Err((e, remainder)) => {
                return DayResult::ParseFailed(anyhow!("Error: {e}, remainder: {remainder}"))
            }
        };
        let pt1 = (self.pt1)(input.as_ref());
        let pt2 = (self.pt2)(input.as_ref());
        DayResult::Ran {
            pt1: pt1.into_result(),
            pt2: pt2.into_result(),
        }
    }
}
