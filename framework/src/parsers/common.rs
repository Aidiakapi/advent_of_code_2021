pub use super::*;

macro_rules! impl_uint_parsing {
    ($kind:tt) => {
        paste::paste! {
            #[derive(Debug, Clone, Copy)]
            #[allow(non_camel_case_types)]
            pub struct [<number_ $kind>];
            impl Parser for [<number_ $kind>] {
                type Output = $kind;

                fn parse<'s>(&self, input: &'s str) -> ParseResult<'s, Self::Output> {
                    let first_char = input
                        .chars()
                        .next()
                        .ok_or((ParseError::EmptyInput, input))?;
                    if !first_char.is_ascii_digit() {
                        return Err((ParseError::ExpectedDigit, input));
                    }

                    let mut remainder = &input[1..];

                    let mut x = (first_char as $kind) - ('0' as $kind);
                    loop {
                        let next_digit = match remainder.chars().next() {
                            Some(c @ '0'..='9') => (c as $kind) - ('0' as $kind),
                            _ => break,
                        };
                        x = x
                            .checked_mul(10)
                            .and_then(|x| x.checked_add(next_digit))
                            .ok_or((ParseError::Overflow, input))?;
                        remainder = &remainder[1..];
                    }

                    Ok((x, remainder))
                }
            }
        }
    };
}

macro_rules! impl_sint_parsing {
    ($kind:tt, $unsigned: tt) => {
        paste::paste! {
            /// Parses an integer. Allows an optional + or - at the start to
            /// indicate a sign.
            #[derive(Debug, Clone, Copy)]
            #[allow(non_camel_case_types)]
            pub struct [<number_ $kind>];
            impl Parser for [<number_ $kind>] {
                type Output = $kind;

                fn parse<'s>(&self, input: &'s str) -> ParseResult<'s, Self::Output> {
                    let (is_negative, remainder) = match input.chars().next() {
                        Some('-') => (true, &input[1..]),
                        Some('+') => (false, &input[1..]),
                        _ => (false, input),
                    };
                    let (number, remainder) =
                        [<number_ $unsigned>].parse(remainder)?;
                    const MAX: $unsigned = $kind::MAX as $unsigned;
                    const MAX_PLUS_ONE: $unsigned = MAX + 1;
                    let number = match (number, is_negative) {
                        (0..=MAX, false) => number as $kind,
                        (0..=MAX, true) => -(number as $kind),
                        (MAX_PLUS_ONE, true) => $kind::MIN,
                        _ => return Err((ParseError::Overflow, input)),
                    };
                    Ok((number, remainder))
                }
            }
        }
    };
}

impl_uint_parsing!(u8);
impl_uint_parsing!(u16);
impl_uint_parsing!(u32);
impl_uint_parsing!(u64);
impl_uint_parsing!(u128);
impl_uint_parsing!(usize);

impl_sint_parsing!(i8, u8);
impl_sint_parsing!(i16, u16);
impl_sint_parsing!(i32, u32);
impl_sint_parsing!(i64, u64);
impl_sint_parsing!(i128, u128);
impl_sint_parsing!(isize, usize);

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Token<T> {
    value: T,
}

impl Parser for Token<char> {
    type Output = ();
    fn parse<'s>(&self, input: &'s str) -> ParseResult<'s, ()> {
        if let Some(c) = input.chars().next() {
            if c == self.value {
                return Ok(((), &input[1..]));
            }
        }
        Err((ParseError::TokenDoesNotMatch, input))
    }
}

impl<T: Clone> Parser for Token<(char, T)> {
    type Output = T;
    fn parse<'s>(&self, input: &'s str) -> ParseResult<'s, T> {
        if let Some(c) = input.chars().next() {
            if c == self.value.0 {
                return Ok((self.value.1.clone(), &input[1..]));
            }
        }
        Err((ParseError::TokenDoesNotMatch, input))
    }
}

impl<'t> Parser for Token<&'t str> {
    type Output = ();
    fn parse<'s>(&self, input: &'s str) -> ParseResult<'s, ()> {
        if input.starts_with(self.value) {
            Ok(((), &input[self.value.len()..]))
        } else {
            Err((ParseError::TokenDoesNotMatch, input))
        }
    }
}

impl<'t, T: Clone> Parser for Token<(&'t str, T)> {
    type Output = T;
    fn parse<'s>(&self, input: &'s str) -> ParseResult<'s, T> {
        if input.starts_with(self.value.0) {
            Ok((self.value.1.clone(), &input[self.value.0.len()..]))
        } else {
            Err((ParseError::TokenDoesNotMatch, input))
        }
    }
}

pub fn token<T>(token: T) -> Token<T> {
    Token { value: token }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn unsigned_numbers() {
        assert_eq!( Ok((0,                         ""    )), number_u8.parse("0"    ));
        assert_eq!( Ok((128,                       ""    )), number_u8.parse("128"  ));
        assert_eq!( Ok((255,                       ""    )), number_u8.parse("255"  ));
        assert_eq!( Ok((10,                        "abc" )), number_u8.parse("10abc"));
        assert_eq!(Err((ParseError::Overflow,      "300" )), number_u8.parse("300"  ));
        assert_eq!(Err((ParseError::Overflow,      "256a")), number_u8.parse("256a" ));
        assert_eq!(Err((ParseError::EmptyInput,    ""    )), number_u8.parse(""     ));
        assert_eq!(Err((ParseError::ExpectedDigit, "-1"  )), number_u8.parse("-1"   ));
    }

    #[test]
    #[rustfmt::skip]
    fn signed_numbers() {
        assert_eq!( Ok((0,                         ""    )), number_i8.parse("0"    ));
        assert_eq!( Ok((127,                       ""    )), number_i8.parse("127"  ));
        assert_eq!( Ok((127,                       ""    )), number_i8.parse("+127" ));
        assert_eq!( Ok((-128,                      ""    )), number_i8.parse("-128" ));
        assert_eq!( Ok((10,                        "abc" )), number_i8.parse("10abc"));
        assert_eq!(Err((ParseError::Overflow,      "+128")), number_i8.parse("+128" ));
        assert_eq!(Err((ParseError::Overflow,      "-129")), number_i8.parse("-129" ));
        assert_eq!(Err((ParseError::EmptyInput,    ""    )), number_i8.parse(""     ));
    }
}
