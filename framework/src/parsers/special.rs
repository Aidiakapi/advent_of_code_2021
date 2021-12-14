use super::*;
use crate::array::init_boxed_array;
use std::marker::PhantomData;

pub trait GridSpec<T> {
    fn initialize() -> Self;
    fn set(&mut self, x: usize, y: usize, value: T) -> Result<(), ()>;
}

impl<T, const W: usize, const H: usize> GridSpec<T> for Box<[[T; H]; W]>
where
    T: Default + Sized,
    [T; W * H]: Sized,
{
    fn initialize() -> Self {
        let boxed = init_boxed_array::<T, { W * H }>();
        unsafe { Box::from_raw(Box::into_raw(boxed).cast()) }
    }

    fn set(&mut self, x: usize, y: usize, value: T) -> Result<(), ()> {
        *self.get_mut(x).and_then(|x| x.get_mut(y)).ok_or(())? = value;
        Ok(())
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

    fn parse<'s>(&self, input: &'s str) -> ParseResult<'s, Self::Output<'s>> {
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
        while let Ok((_, new_remainder)) = self.line_sep.parse(remainder) {
            remainder = new_remainder;
            for x in 0..width {
                let (value, new_remainder) = match self.value.parse(remainder) {
                    Ok(x) => x,
                    Err(_) if x == 0 => return Ok((grid, remainder)),
                    Err(e) => return Err(e),
                };
                set(x, height, value)?;
                remainder = new_remainder;
            }
            height += 1;
        }

        Ok((grid, remainder))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_grid() {
        const EXAMPLE: &'static str = "\
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
        let cell = token(('.', Cell::Empty))
            .or(token(('#', Cell::Wall)))
            .or(token(('O', Cell::Object)))
            .or(token(('X', Cell::Target)));
        let grid: Box<[[Cell; 3]; 5]> = grid(token('\n'), cell, |x, y, cell| {
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
