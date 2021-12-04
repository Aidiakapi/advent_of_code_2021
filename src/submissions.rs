use colored::Colorize;
use framework::day::{AutoImplementToColoredString, ColoredOutput, ToColoredString};
use std::{
    fmt::Display,
    ops::{Add, Mul},
};

macro_rules! impl_submission {
    ($name:ident, $op_trait:ident, $op_fn:ident, $fmt:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name<T: Clone + Display + $op_trait<Output = T>>(pub T, pub T);
        impl<T: Clone + Display + $op_trait<Output = T>> ToColoredString for $name<T> {
            fn to_colored(self) -> ColoredOutput {
                let result = self.0.clone().$op_fn(self.1.clone()).to_string().bold().white();
                ColoredOutput {
                    str: format!($fmt, self.0, self.1, result),
                    control_char_count: 11,
                }
            }
        }

        impl<T: Clone + Display + $op_trait<Output = T>> Display for $name<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    $fmt,
                    self.0,
                    self.1,
                    self.0.clone().$op_fn(self.1.clone())
                )
            }
        }

        impl<T: Clone + Display + $op_trait<Output = T>> !AutoImplementToColoredString
            for $name<T>
        {
        }
    };
}

impl_submission!(AddSubmission, Add, add, "{} + {} = {}");
impl_submission!(MulSubmission, Mul, mul, "{} * {} = {}");
