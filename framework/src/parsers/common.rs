pub use super::*;

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
        assert_eq!( Ok((0,                         ""    )), number::<u8>().parse("0"    ));
        assert_eq!( Ok((128,                       ""    )), number::<u8>().parse("128"  ));
        assert_eq!( Ok((255,                       ""    )), number::<u8>().parse("255"  ));
        assert_eq!( Ok((10,                        "abc" )), number::<u8>().parse("10abc"));
        assert_eq!(Err((ParseError::Overflow,      "300" )), number::<u8>().parse("300"  ));
        assert_eq!(Err((ParseError::Overflow,      "256a")), number::<u8>().parse("256a" ));
        assert_eq!(Err((ParseError::EmptyInput,    ""    )), number::<u8>().parse(""     ));
        assert_eq!(Err((ParseError::ExpectedDigit, "-1"  )), number::<u8>().parse("-1"   ));
    }

    #[test]
    #[rustfmt::skip]
    fn signed_numbers() {
        assert_eq!( Ok((0,                         ""    )), number::<i8>().parse("0"    ));
        assert_eq!( Ok((127,                       ""    )), number::<i8>().parse("127"  ));
        assert_eq!( Ok((127,                       ""    )), number::<i8>().parse("+127" ));
        assert_eq!( Ok((-128,                      ""    )), number::<i8>().parse("-128" ));
        assert_eq!( Ok((10,                        "abc" )), number::<i8>().parse("10abc"));
        assert_eq!(Err((ParseError::Overflow,      "+128")), number::<i8>().parse("+128" ));
        assert_eq!(Err((ParseError::Overflow,      "-129")), number::<i8>().parse("-129" ));
        assert_eq!(Err((ParseError::EmptyInput,    ""    )), number::<i8>().parse(""     ));
    }
}
