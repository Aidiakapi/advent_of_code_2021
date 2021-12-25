use super::*;
use crate::array::init_boxed_array;
use bitvec::prelude::*;
use std::{fmt::Display, marker::PhantomData};

pub trait GridSpec<T> {
    type Intermediate;
    fn initialize() -> Self::Intermediate;
    fn set(data: &mut Self::Intermediate, x: usize, y: usize, value: T) -> Result<(), ()>;
    fn finalize(data: Self::Intermediate) -> Self;
}

impl<T, const W: usize, const H: usize> GridSpec<T> for Box<[[T; H]; W]>
where
    T: Default + Sized,
    [T; W * H]: Sized,
{
    type Intermediate = Self;

    fn initialize() -> Self {
        let boxed = init_boxed_array::<T, { W * H }>();
        unsafe { Box::from_raw(Box::into_raw(boxed).cast()) }
    }

    fn set(data: &mut Self, x: usize, y: usize, value: T) -> Result<(), ()> {
        *data.get_mut(x).and_then(|x| x.get_mut(y)).ok_or(())? = value;
        Ok(())
    }

    fn finalize(data: Self::Intermediate) -> Self {
        data
    }
}

impl GridSpec<bool> for (usize, BitVec) {
    type Intermediate = (usize, bool, BitVec);

    fn initialize() -> Self::Intermediate {
        (0, false, BitVec::new())
    }

    fn set(
        (width, locked_width, data): &mut Self::Intermediate,
        x: usize,
        y: usize,
        value: bool,
    ) -> Result<(), ()> {
        if *locked_width {
            if x >= *width {
                return Err(());
            }
        } else if y == 0 {
            if x >= *width {
                data.resize(x + 1, false);
                *width = x + 1;
            }
        } else {
            *locked_width = true;
        }
        let min_size = *width * (y + 1);
        if data.len() < min_size {
            data.resize(min_size, false);
        }
        data.set(y * *width + x, value);
        Ok(())
    }

    fn finalize(data: Self::Intermediate) -> Self {
        (data.0, data.2)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DynGrid<T: Default> {
    pub data: Vec<T>,
    pub width: usize,
}

impl<T: Display + Default> Display for DynGrid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let height = self.data.len() / self.width;
        for y in 0..height {
            let base = y * self.width;
            for x in 0..self.width {
                self.data[base + x].fmt(f)?;
            }
            if y != height - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl<T: Default> GridSpec<T> for DynGrid<T> {
    type Intermediate = (usize, bool, Vec<T>);

    fn initialize() -> Self::Intermediate {
        (0, false, Vec::new())
    }

    fn set(
        (width, locked_width, data): &mut Self::Intermediate,
        x: usize,
        y: usize,
        value: T,
    ) -> Result<(), ()> {
        if *locked_width {
            if x >= *width {
                return Err(());
            }
        } else if y == 0 {
            if x >= *width {
                data.resize_with(x + 1, Default::default);
                *width = x + 1;
            }
        } else {
            *locked_width = true;
        }
        let min_size = *width * (y + 1);
        if data.len() < min_size {
            data.resize_with(min_size, Default::default);
        }
        data[y * *width + x] = value;
        Ok(())
    }

    fn finalize(data: Self::Intermediate) -> Self {
        DynGrid {
            data: data.2,
            width: data.0,
        }
    }
}

pub struct Grid<GS, S, P, FT> {
    line_sep: S,
    value: P,
    transform: FT,
    _spec: PhantomData<GS>,
}

pub fn grid<T, I, GS, S, P, FT>(line_sep: S, value: P, transform: FT) -> Grid<GS, S, P, FT>
where
    T: Default,
    GS: GridSpec<T>,
    S: Parser,
    P: for<'s> Parser<Output<'s> = I>,
    FT: Fn(usize, usize, I) -> Option<(usize, usize, T)>,
{
    Grid {
        line_sep,
        value,
        transform,
        _spec: PhantomData,
    }
}

impl<T, I, GS, S, P, FT> Parser for Grid<GS, S, P, FT>
where
    T: Default,
    GS: GridSpec<T>,
    S: Parser,
    P: for<'s> Parser<Output<'s> = I>,
    FT: Fn(usize, usize, I) -> Option<(usize, usize, T)>,
{
    type Output<'s> = GS;

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
        let mut grid = GS::initialize();

        let mut set = {
            let grid = &mut grid;
            move |x, y, value| {
                if let Some((x, y, value)) = (self.transform)(x, y, value) {
                    GS::set(grid, x, y, value)
                        .ok()
                        .ok_or((ParseError::GridCellOutOfRange(x, y), input))
                } else {
                    Ok(())
                }
            }
        };

        let mut remainder = input;
        let mut width = 0;
        while let Ok((value, new_remainder)) = self.value.parse(remainder) {
            remainder = new_remainder;
            set(width, 0, value)?;
            width += 1;
        }

        if width == 0 {
            return Err((ParseError::ExpectedGridCell, input));
        }

        let mut height = 1;
        'outer: while let Ok((_, new_remainder)) = self.line_sep.parse(remainder) {
            remainder = new_remainder;
            for x in 0..width {
                let (value, new_remainder) = match self.value.parse(remainder) {
                    Ok(x) => x,
                    Err(_) if x == 0 => break 'outer,
                    Err(e) => return Err(e),
                };
                set(x, height, value)?;
                remainder = new_remainder;
            }
            height += 1;
        }

        Ok((GS::finalize(grid), remainder))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_grid() {
        const EXAMPLE: &'static [u8] = b"\
#######
#O....#
###.###
#....X#
#######";

        #[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
        enum Cell {
            #[default]
            Empty,
            Wall,
            Object,
            Target,
        }
        let cell = token((b'.', Cell::Empty))
            .or(token((b'#', Cell::Wall)))
            .or(token((b'O', Cell::Object)))
            .or(token((b'X', Cell::Target)));
        let grid: Box<[[Cell; 3]; 5]> = grid(token(b'\n'), cell, |x, y, cell| {
            if matches!((x, y), (1..=5, 1..=3)) {
                Some((x - 1, y - 1, cell))
            } else {
                None
            }
        })
        .parse(EXAMPLE)
        .unwrap()
        .0;

        assert!(grid.eq(&Box::new([
            [Cell::Object, Cell::Wall, Cell::Empty],
            [Cell::Empty, Cell::Wall, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Wall, Cell::Empty],
            [Cell::Empty, Cell::Wall, Cell::Target],
        ])));
    }
}
