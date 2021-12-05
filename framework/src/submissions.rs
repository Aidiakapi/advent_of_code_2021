use colored::{Colorize, Color};
use crate::day::{AutoImplementToColoredString, ColoredOutput, ToColoredString};
use std::{
    fmt::Display,
    ops::{Add, Mul},
};

const SYMBOL_COLOR: Color = Color::BrightYellow;

macro_rules! impl_submission {
    ($name:ident, $op_trait:ident, $op_fn:ident, $op_str:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name<T: Clone + Display + $op_trait<Output = T>>(pub T, pub T);
        impl<T: Clone + Display + $op_trait<Output = T>> ToColoredString for $name<T> {
            fn to_colored(self) -> ColoredOutput {
                let result = self.0.clone().$op_fn(self.1.clone());
                let result = result.to_string().bold().white();
                let op = $op_str.color(SYMBOL_COLOR);
                let eq = "=".color(SYMBOL_COLOR);
                ColoredOutput {
                    str: format!("{} {} {} {} {}", self.0, op, self.1, eq, result),
                    control_char_count: 29,
                }
            }
        }

        impl<T: Clone + Display + $op_trait<Output = T>> Display for $name<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let result = self.0.clone().$op_fn(self.1.clone());
                write!(f, "{} {} {} = {}", self.0, $op_str, self.1, result)
            }
        }

        impl<T: Clone + Display + $op_trait<Output = T>> !AutoImplementToColoredString
            for $name<T>
        {
        }
    };
}

impl_submission!(AddSubmission, Add, add, "+");
impl_submission!(MulSubmission, Mul, mul, "×");
