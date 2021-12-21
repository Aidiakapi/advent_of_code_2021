use std::{
    fmt::Debug,
    marker::PhantomData,
    mem::{swap, MaybeUninit},
};

use super::*;

pub trait ParserMultiExt: Sized + Parser {
    /// Repeatedly applies the parser, interspersing applications of `separator`.
    /// Fails if parser cannot be applied at least once.
    fn sep_by<'s, S, C: Default + Extend<Self::Output<'s>>>(self, separator: S) -> SepBy<Self, S, C>
    where
        S: Parser,
    {
        SepBy {
            parser: self,
            separator,
            _collection: PhantomData,
        }
    }

    /// Repeatedly applies the parser, repeatedly invoking `func` with the
    /// output value, updating the accumulator which starts out as `initial`.
    fn fold<A, F>(self, initial: A, func: F) -> Fold<Self, A, F>
    where
        A: Clone,
        F: Fn(A, Self::Output<'_>) -> A,
    {
        Fold {
            parser: self,
            initial,
            func,
        }
    }

    /// Repeatedly applies the parser, repeatedly invoking `func` with the
    /// output value, updating the accumulator which starts out as `initial`.
    fn fold_mut<A, F>(self, initial: A, func: F) -> FoldMut<Self, A, F>
    where
        A: Clone,
        F: Fn(&mut A, Self::Output<'_>),
    {
        FoldMut {
            parser: self,
            initial,
            func,
        }
    }

    /// Repeatedly applies the parser, until failure, returning the last
    /// successful output, or an error if it fails to apply even once.
    fn repeat(self) -> Repeat<Self> {
        Repeat { parser: self }
    }

    /// Repeatedly applies the parser, until failure, returning a collection
    /// of all successfully applied values.
    fn repeat_into<'s, C: Default + Extend<Self::Output<'s>>>(self) -> RepeatInto<Self, C> {
        RepeatInto {
            parser: self,
            _collection: PhantomData,
        }
    }

    fn many_n<const N: usize>(self) -> Many<Self, N> {
        Many { parser: self }
    }
}

impl<P: Parser> ParserMultiExt for P {}

#[derive(Debug, Clone, Copy)]
pub struct TakeWhile<F>(F);
impl<F> Parser for TakeWhile<F>
where
    F: Fn(u8) -> bool,
{
    type Output<'s> = &'s [u8];

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
        let mut index = 0;
        loop {
            match input.get(index) {
                Some(&c) if (self.0)(c) => index += 1,
                _ => break,
            }
        }
        if index == 0 {
            Err((ParseError::UnexpectedChar, input))
        } else {
            Ok((&input[0..index], &input[index..]))
        }
    }
}
pub fn take_while<F>(f: F) -> TakeWhile<F>
where
    F: Fn(u8) -> bool,
{
    TakeWhile(f)
}

#[derive(Debug, Clone, Copy)]
pub struct SepBy<P, S, C> {
    parser: P,
    separator: S,
    _collection: PhantomData<C>,
}

#[derive(Debug, Clone, Copy)]
pub struct Fold<P, A, F> {
    parser: P,
    initial: A,
    func: F,
}

#[derive(Debug, Clone, Copy)]
pub struct FoldMut<P, A, F> {
    parser: P,
    initial: A,
    func: F,
}

#[derive(Debug, Clone, Copy)]
pub struct Repeat<P> {
    parser: P,
}

#[derive(Debug, Clone, Copy)]
pub struct RepeatInto<P, C> {
    parser: P,
    _collection: PhantomData<C>,
}

#[derive(Debug, Clone, Copy)]
pub struct Many<P, const N: usize> {
    parser: P,
}

impl<P, S, C> Parser for SepBy<P, S, C>
where
    P: Parser,
    S: Parser,
    C: Default + for<'s> Extend<P::Output<'s>>,
{
    type Output<'s> = C;

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
        let (element, mut remainder) = self.parser.parse(input)?;
        let mut elements = C::default();
        elements.extend(Some(element));
        loop {
            let after_sep = match self.separator.parse(remainder) {
                Ok((_, after_sep)) => after_sep,
                Err(_) => return Ok((elements, remainder)),
            };
            match self.parser.parse(after_sep) {
                Ok((element, after_value)) => {
                    remainder = after_value;
                    elements.extend(Some(element));
                }
                Err(_) => return Ok((elements, remainder)),
            };
        }
    }
}

impl<P, A, F> Parser for Fold<P, A, F>
where
    P: Parser,
    A: Clone,
    F: Fn(A, P::Output<'_>) -> A,
{
    type Output<'s> = A;

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
        let mut accumulator = self.initial.clone();
        let mut remainder = input;
        while let Ok((value, new_remainder)) = self.parser.parse(remainder) {
            accumulator = (self.func)(accumulator, value);
            remainder = new_remainder;
        }
        Ok((accumulator, remainder))
    }
}

impl<P, A, F> Parser for FoldMut<P, A, F>
where
    P: Parser,
    A: Clone,
    F: Fn(&mut A, P::Output<'_>),
{
    type Output<'s> = A;

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
        let mut accumulator = self.initial.clone();
        let mut remainder = input;
        while let Ok((value, new_remainder)) = self.parser.parse(remainder) {
            (self.func)(&mut accumulator, value);
            remainder = new_remainder;
        }
        Ok((accumulator, remainder))
    }
}

impl<P> Parser for Repeat<P>
where
    P: Parser,
{
    type Output<'s> = P::Output<'s>;

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
        let (mut last_value, mut remainder) = self.parser.parse(input)?;
        while let Ok((value, new_remainder)) = self.parser.parse(remainder) {
            last_value = value;
            remainder = new_remainder;
        }
        Ok((last_value, remainder))
    }
}

impl<P: Parser, C: Default + for<'s> Extend<P::Output<'s>>> Parser for RepeatInto<P, C> {
    type Output<'s> = C;

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
        let mut c = C::default();

        let (first_value, mut remainder) = self.parser.parse(input)?;
        c.extend(Some(first_value));
        while let Ok((value, new_remainder)) = self.parser.parse(remainder) {
            c.extend(Some(value));
            remainder = new_remainder;
        }
        Ok((c, remainder))
    }
}

impl<P: Parser, const N: usize> Parser for Many<P, N> {
    type Output<'s> = [P::Output<'s>; N];

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
        struct PartiallyInit<T, const N: usize> {
            memory: [MaybeUninit<T>; N],
            count: usize,
        }

        impl<T, const N: usize> Drop for PartiallyInit<T, N> {
            fn drop(&mut self) {
                for i in (0..self.count).rev() {
                    unsafe {
                        self.memory[i].assume_init_drop();
                    }
                }
            }
        }

        let mut partially_init = PartiallyInit::<P::Output<'s>, N> {
            memory: MaybeUninit::uninit_array(),
            count: 0,
        };

        let mut remainder = input;
        while partially_init.count < N {
            let (value, new_remainder) = self.parser.parse(remainder)?;
            remainder = new_remainder;
            partially_init.memory[partially_init.count].write(value);
            partially_init.count += 1;
        }

        let result = unsafe {
            let mut memory = MaybeUninit::uninit_array();
            swap(&mut memory, &mut partially_init.memory);
            MaybeUninit::array_assume_init(memory)
        };
        Ok((result, remainder))
    }
}
