use std::{
    fmt::Display,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vec2u = Vec2<usize>;
pub type Vec2i = Vec2<isize>;

impl<F, I: From<F>> From<(F, F)> for Vec2<I> {
    fn from((x, y): (F, F)) -> Self {
        Vec2 {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl<F, I: From<F>> From<Vec2<F>> for (I, I) {
    fn from(Vec2 { x, y }: Vec2<F>) -> Self {
        (x.into(), y.into())
    }
}

macro_rules! impl_vec_bin_ops {
    ($trait_name:ident, $trait_fn:ident, $assign_trait_name:ident, $assign_fn_name:ident) => {
        impl<T: $trait_name<Output = T>> $trait_name for Vec2<T> {
            type Output = Vec2<T>;

            fn $trait_fn(self, rhs: Self) -> Self::Output {
                Vec2 {
                    x: self.x.$trait_fn(rhs.x),
                    y: self.y.$trait_fn(rhs.y),
                }
            }
        }

        impl<T: $trait_name<Output = T> + Clone> $trait_name<T> for Vec2<T> {
            type Output = Vec2<T>;

            fn $trait_fn(self, rhs: T) -> Self::Output {
                Vec2 {
                    x: self.x.$trait_fn(rhs.clone()),
                    y: self.y.$trait_fn(rhs),
                }
            }
        }

        impl<T: $assign_trait_name> $assign_trait_name for Vec2<T> {
            fn $assign_fn_name(&mut self, rhs: Self) {
                self.x.$assign_fn_name(rhs.x);
                self.y.$assign_fn_name(rhs.y);
            }
        }

        impl<T: $assign_trait_name + Clone> $assign_trait_name<T> for Vec2<T> {
            fn $assign_fn_name(&mut self, rhs: T) {
                self.x.$assign_fn_name(rhs.clone());
                self.y.$assign_fn_name(rhs);
            }
        }

        impl<T: $trait_name<Output = T>> $trait_name for Vec3<T> {
            type Output = Vec3<T>;

            fn $trait_fn(self, rhs: Self) -> Self::Output {
                Vec3 {
                    x: self.x.$trait_fn(rhs.x),
                    y: self.y.$trait_fn(rhs.y),
                    z: self.z.$trait_fn(rhs.z),
                }
            }
        }

        impl<T: $trait_name<Output = T> + Clone> $trait_name<T> for Vec3<T> {
            type Output = Vec3<T>;

            fn $trait_fn(self, rhs: T) -> Self::Output {
                Vec3 {
                    x: self.x.$trait_fn(rhs.clone()),
                    y: self.y.$trait_fn(rhs.clone()),
                    z: self.z.$trait_fn(rhs),
                }
            }
        }

        impl<T: $assign_trait_name> $assign_trait_name for Vec3<T> {
            fn $assign_fn_name(&mut self, rhs: Self) {
                self.x.$assign_fn_name(rhs.x);
                self.y.$assign_fn_name(rhs.y);
                self.z.$assign_fn_name(rhs.z);
            }
        }

        impl<T: $assign_trait_name + Clone> $assign_trait_name<T> for Vec3<T> {
            fn $assign_fn_name(&mut self, rhs: T) {
                self.x.$assign_fn_name(rhs.clone());
                self.y.$assign_fn_name(rhs.clone());
                self.z.$assign_fn_name(rhs);
            }
        }
    };
}

impl_vec_bin_ops!(Add, add, AddAssign, add_assign);
impl_vec_bin_ops!(Sub, sub, SubAssign, sub_assign);
impl_vec_bin_ops!(Mul, mul, MulAssign, mul_assign);
impl_vec_bin_ops!(Div, div, DivAssign, div_assign);
impl_vec_bin_ops!(Rem, rem, RemAssign, rem_assign);
impl_vec_bin_ops!(BitAnd, bitand, BitAndAssign, bitand_assign);
impl_vec_bin_ops!(BitOr, bitor, BitOrAssign, bitor_assign);
impl_vec_bin_ops!(BitXor, bitxor, BitXorAssign, bitxor_assign);
impl_vec_bin_ops!(Shl, shl, ShlAssign, shl_assign);
impl_vec_bin_ops!(Shr, shr, ShrAssign, shr_assign);

macro_rules! impl_vec_un_ops {
    ($trait_name:ident, $trait_fn:ident) => {
        impl<T: $trait_name<Output = O>, O> $trait_name for Vec2<T> {
            type Output = Vec2<O>;

            fn $trait_fn(self) -> Self::Output {
                Vec2 {
                    x: self.x.$trait_fn(),
                    y: self.y.$trait_fn(),
                }
            }
        }

        impl<T: $trait_name<Output = O>, O> $trait_name for Vec3<T> {
            type Output = Vec3<O>;

            fn $trait_fn(self) -> Self::Output {
                Vec3 {
                    x: self.x.$trait_fn(),
                    y: self.y.$trait_fn(),
                    z: self.z.$trait_fn(),
                }
            }
        }
    };
}

impl_vec_un_ops!(Neg, neg);
impl_vec_un_ops!(Not, not);

impl<T: PartialOrd> Vec2<T> {
    pub fn max(self, other: Self) -> Self {
        Self {
            x: if self.x > other.x { self.x } else { other.x },
            y: if self.y > other.y { self.y } else { other.y },
        }
    }

    pub fn min(self, other: Self) -> Self {
        Self {
            x: if self.x < other.x { self.x } else { other.x },
            y: if self.y < other.y { self.y } else { other.y },
        }
    }
}

impl<T: PartialOrd + Add<Output = T> + Sub<Output = T> + Clone> Vec2<T> {
    pub fn manhathan_dist(self, other: Self) -> T {
        let max = Self::max(self.clone(), other.clone());
        let min = Self::min(self, other);
        (max.x - min.x) + (max.y - min.y)
    }
}

impl<T: Display> Display for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: PartialOrd> Vec3<T> {
    pub fn max(self, other: Self) -> Self {
        Self {
            x: if self.x > other.x { self.x } else { other.x },
            y: if self.y > other.y { self.y } else { other.y },
            z: if self.z > other.z { self.z } else { other.z },
        }
    }

    pub fn min(self, other: Self) -> Self {
        Self {
            x: if self.x < other.x { self.x } else { other.x },
            y: if self.y < other.y { self.y } else { other.y },
            z: if self.z < other.z { self.z } else { other.z },
        }
    }
}

impl<T: PartialOrd + Add<Output = T> + Sub<Output = T> + Clone> Vec3<T> {
    pub fn manhathan_dist(self, other: Self) -> T {
        let max = Self::max(self.clone(), other.clone());
        let min = Self::min(self, other);
        (max.x - min.x) + (max.y - min.y) + (max.z - min.z)
    }
}

impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
