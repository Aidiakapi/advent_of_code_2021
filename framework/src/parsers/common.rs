pub use super::*;

macro_rules! impl_uint_parsing {
    ($kind:tt, $name:ident) => {
        #[derive(Debug, Clone, Copy)]
        #[allow(non_camel_case_types)]
        pub struct $name;
        impl Parser for $name {
            type Output<'s> = $kind;

            fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
                let first_char = *input.first().ok_or((ParseError::EmptyInput, input))?;
                if !matches!(first_char, b'0'..=b'9') {
                    return Err((ParseError::ExpectedDigit, input));
                }

                let mut remainder = &input[1..];

                let mut x = (first_char as $kind) - ('0' as $kind);
                loop {
                    let next_digit = match remainder.first() {
                        Some(&c @ b'0'..=b'9') => (c as $kind) - ('0' as $kind),
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
    };
}

macro_rules! impl_sint_parsing {
    ($kind:tt, $name:ident, $unsigned:tt, $unsigned_name:ident) => {
        /// Parses an integer. Allows an optional + or - at the start to
        /// indicate a sign.
        #[derive(Debug, Clone, Copy)]
        #[allow(non_camel_case_types)]
        pub struct $name;
        impl Parser for $name {
            type Output<'s> = $kind;

            fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
                let (is_negative, remainder) = match input.first() {
                    Some(&b'-') => (true, &input[1..]),
                    Some(&b'+') => (false, &input[1..]),
                    _ => (false, input),
                };
                let (number, remainder) = $unsigned_name.parse(remainder)?;
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
    };
}

impl_uint_parsing!(u8, number_u8);
impl_uint_parsing!(u16, number_u16);
impl_uint_parsing!(u32, number_u32);
impl_uint_parsing!(u64, number_u64);
impl_uint_parsing!(u128, number_u128);
impl_uint_parsing!(usize, number_usize);

impl_sint_parsing!(i8, number_i8, u8, number_u8);
impl_sint_parsing!(i16, number_i16, u16, number_u16);
impl_sint_parsing!(i32, number_i32, u32, number_u32);
impl_sint_parsing!(i64, number_i64, u64, number_u64);
impl_sint_parsing!(i128, number_i128, u128, number_u128);
impl_sint_parsing!(isize, number_isize, usize, number_usize);

pub struct Digit;
pub const fn digit() -> Digit {
    Digit
}
impl Parser for Digit {
    type Output<'s> = u8;

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
        match input.first().cloned() {
            None => Err((ParseError::EmptyInput, input)),
            Some(d @ b'0'..=b'9') => Ok((d - b'0', &input[1..])),
            Some(_) => Err((ParseError::ExpectedDigit, input)),
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Token<T> {
    value: T,
}

impl Parser for Token<u8> {
    type Output<'s> = ();
    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, ()> {
        if let Some(&c) = input.first() {
            if c == self.value {
                return Ok(((), &input[1..]));
            }
        }
        Err((ParseError::TokenDoesNotMatch, input))
    }
}

impl<T: Clone> Parser for Token<(u8, T)> {
    type Output<'s> = T;
    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, T> {
        if let Some(&c) = input.first() {
            if c == self.value.0 {
                return Ok((self.value.1.clone(), &input[1..]));
            }
        }
        Err((ParseError::TokenDoesNotMatch, input))
    }
}

impl<'t> Parser for Token<&'t [u8]> {
    type Output<'s> = ();
    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, ()> {
        if input.starts_with(self.value) {
            Ok(((), &input[self.value.len()..]))
        } else {
            Err((ParseError::TokenDoesNotMatch, input))
        }
    }
}

impl<'t, T: Clone> Parser for Token<(&'t [u8], T)> {
    type Output<'s> = T;
    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, T> {
        if input.starts_with(self.value.0) {
            Ok((self.value.1.clone(), &input[self.value.0.len()..]))
        } else {
            Err((ParseError::TokenDoesNotMatch, input))
        }
    }
}

impl<'t, const N: usize> Parser for Token<&'t [u8; N]> {
    type Output<'s> = ();
    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, ()> {
        if input.starts_with(self.value) {
            Ok(((), &input[self.value.len()..]))
        } else {
            Err((ParseError::TokenDoesNotMatch, input))
        }
    }
}

impl<'t, T: Clone, const N: usize> Parser for Token<(&'t [u8; N], T)> {
    type Output<'s> = T;
    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, T> {
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

#[derive(Debug, Clone, Copy)]
pub struct Any;
impl Parser for Any {
    type Output<'s> = u8;

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
        match input.first() {
            Some(&c) => Ok((c, &input[1..])),
            None => Err((ParseError::EmptyInput, input)),
        }
    }
}
pub fn any() -> Any {
    Any
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
