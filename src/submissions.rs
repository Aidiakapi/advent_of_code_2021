use colored::Colorize;
use framework::day::ToColoredString;
use std::{
    fmt::Display,
    ops::{Add, Mul},
};

macro_rules! impl_submission {
    ($name:ident, $op_trait:ident, $op_fn:ident, $fmt:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name<T: Clone + Display + $op_trait<Output = T>>(pub T, pub T);
        impl<T: Clone + Display + $op_trait<Output = T>> ToColoredString for $name<T> {
            fn to_colored(self) -> String {
                let result = self.0.clone().$op_fn(self.1.clone()).to_string().bold();
                format!($fmt, self.0, self.1, result)
            }
        }
    };
}

impl_submission!(AddSubmission, Add, add, "{} + {} = {}");
impl_submission!(MulSubmission, Mul, mul, "{} * {} = {}");
