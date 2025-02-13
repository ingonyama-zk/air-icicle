extern crate std;
use core::cmp;
use core::fmt::{Debug, Display, Formatter};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::sync::Arc;

use alloc::fmt;
use alloc::vec::Vec;
use icicle_core::traits::{Arithmetic, FieldImpl};

use crate::symbolic_variable::SymbolicVariable;

/// An expression over `SymbolicVariable`s.
#[derive(Clone, Debug, PartialEq)]
pub enum SymbolicExpression<F: FieldImpl + Arithmetic> {
    Variable(SymbolicVariable<F>),
    IsFirstRow,
    IsLastRow,
    IsTransition,
    Constant(F),
    Add {
        x: Arc<Self>,
        y: Arc<Self>,
        degree_multiple: usize,
    },
    Sub {
        x: Arc<Self>,
        y: Arc<Self>,
        degree_multiple: usize,
    },
    Neg {
        x: Arc<Self>,
        degree_multiple: usize,
    },
    Mul {
        x: Arc<Self>,
        y: Arc<Self>,
        degree_multiple: usize,
    },
}

impl<F: FieldImpl + Arithmetic> SymbolicExpression<F> {
    /// Returns the multiple of `n` (the trace length) in this expression's degree.
    pub const fn degree_multiple(&self) -> usize {
        match self {
            SymbolicExpression::Variable(v) => v.degree_multiple(),
            SymbolicExpression::IsFirstRow => 1,
            SymbolicExpression::IsLastRow => 1,
            SymbolicExpression::IsTransition => 0,
            SymbolicExpression::Constant(_) => 0,
            SymbolicExpression::Add {
                degree_multiple, ..
            } => *degree_multiple,
            SymbolicExpression::Sub {
                degree_multiple, ..
            } => *degree_multiple,
            SymbolicExpression::Neg {
                degree_multiple, ..
            } => *degree_multiple,
            SymbolicExpression::Mul {
                degree_multiple, ..
            } => *degree_multiple,
        }
    }
}

impl<F: FieldImpl + Arithmetic + Display> Display for SymbolicExpression<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Variable(var) => write!(f, "{:?}", var),
            Self::IsFirstRow => write!(f, "IsFirstRow"),
            Self::IsLastRow => write!(f, "IsLastRow"),
            Self::IsTransition => write!(f, "IsTransition"),
            Self::Constant(val) => write!(f, "{}", val),
            Self::Add { x, y, .. } => write!(f, "({} + {})", &**x, &**y),
            Self::Sub { x, y, .. } => write!(f, "({} - {})", &**x, &**y),
            Self::Mul { x, y, .. } => write!(f, "({} * {})", &**x, &**y),
            Self::Neg { x, .. } => write!(f, "(-{})", &**x),
        }
    }
}

impl<F: FieldImpl + Arithmetic> FieldImpl for SymbolicExpression<F> {
    type Config = F::Config;
    type Repr = F::Repr;

    fn to_bytes_le(&self) -> Vec<u8> {
        match self {
            Self::Constant(f) => f.to_bytes_le(),
            _ => panic!("Cannot convert symbolic expression to bytes"),
        }
    }

    fn from_bytes_le(bytes: &[u8]) -> Self {
        Self::Constant(F::from_bytes_le(bytes))
    }

    fn from_hex(s: &str) -> Self {
        Self::Constant(F::from_hex(s))
    }

    fn zero() -> Self {
        Self::Constant(F::zero())
    }

    fn one() -> Self {
        Self::Constant(F::one())
    }

    fn from_u32(val: u32) -> Self {
        Self::Constant(F::from_u32(val))
    }

    fn from_repr(repr: Self::Repr) -> Self {
        Self::Constant(F::from_repr(repr))
    }
}

impl<F: FieldImpl + Arithmetic> Arithmetic for SymbolicExpression<F> {
    fn sqr(self) -> Self {
        // Square is equivalent to multiplying by self
        // Degree multiple doubles because we're multiplying by self
        let degree_multiple = self.degree_multiple() * 2;
        Self::Mul {
            x: Arc::new(self.clone()),
            y: Arc::new(self),
            degree_multiple,
        }
    }

    fn inv(self) -> Self {
        // For symbolic expressions, we might need to handle this specially
        // You might want to add an Inv variant to your SymbolicExpression enum
        todo!("Implement inverse for symbolic expressions")
    }

    fn pow(self, exp: usize) -> Self {
        match exp {
            0 => Self::Constant(F::one()),
            1 => self,
            2 => self.sqr(),
            _ => {
                // For higher powers, we can use the square-and-multiply algorithm
                // but for symbolic expressions, you might want to add a Pow variant
                todo!("Implement pow for symbolic expressions")
            }
        }
    }
}

impl<F: FieldImpl + Arithmetic> Default for SymbolicExpression<F> {
    fn default() -> Self {
        Self::Constant(F::zero())
    }
}

impl<F: FieldImpl + Arithmetic> From<F> for SymbolicExpression<F> {
    fn from(value: F) -> Self {
        Self::Constant(value)
    }
}

impl<F: FieldImpl + Arithmetic> SymbolicExpression<F> {
    // type F = F;

    // const ZERO: Self = Self::Constant(F::ZERO);
    // const ONE: Self = Self::Constant(F::ONE);
    // const TWO: Self = Self::Constant(F::TWO);
    // const NEG_ONE: Self = Self::Constant(F::NEG_ONE);

    #[inline]
    // fn from_f(f: Self::F) -> Self {
    //     f.into()
    // }

    fn from_bool(b: bool) -> Self {
        if b {
            Self::Constant(F::one())
        } else {
            Self::Constant(F::zero())
        }
    }

    // fn from_canonical_u8(n: u8) -> Self {
    //     Self::Constant(F::from_canonical_u8(n))
    // }

    // fn from_canonical_u16(n: u16) -> Self {
    //     Self::Constant(F::from_canonical_u16(n))
    // }

    fn from_canonical_u32(n: u32) -> Self {
        Self::Constant(F::from_u32(n))
    }

    // fn from_canonical_u64(n: u64) -> Self {
    //     Self::Constant(F::from_canonical_u64(n))
    // }

    // fn from_canonical_usize(n: usize) -> Self {
    //     Self::Constant(F::from_canonical_usize(n))
    // }

    // fn from_wrapped_u32(n: u32) -> Self {
    //     Self::Constant(F::from_wrapped_u32(n))
    // }

    // fn from_wrapped_u64(n: u64) -> Self {
    //     Self::Constant(F::from_wrapped_u64(n))
    // }
}

impl<F: FieldImpl + Arithmetic, T> Add<T> for SymbolicExpression<F>
where
    T: Into<Self>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self {
        let rhs = rhs.into();
        match (self, rhs) {
            (Self::Constant(lhs), Self::Constant(rhs)) => Self::Constant(lhs + rhs),
            (lhs, rhs) => {
                let degree_multiple = cmp::max(lhs.degree_multiple(), rhs.degree_multiple());
                Self::Add {
                    x: Arc::new(lhs),
                    y: Arc::new(rhs),
                    degree_multiple,
                }
            }
        }
    }
}

impl<F: FieldImpl + Arithmetic, T> AddAssign<T> for SymbolicExpression<F>
where
    T: Into<Self>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = self.clone() + rhs.into();
    }
}

impl<F: FieldImpl + Arithmetic, T> Sum<T> for SymbolicExpression<F>
where
    T: Into<Self>,
{
    fn sum<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.map(Into::into)
            .reduce(|x, y| x + y)
            .unwrap_or(Self::from_canonical_u32(0))
    }
}

impl<F: FieldImpl + Arithmetic, T> Sub<T> for SymbolicExpression<F>
where
    T: Into<Self>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self {
        let rhs = rhs.into();
        match (self, rhs) {
            (Self::Constant(lhs), Self::Constant(rhs)) => Self::Constant(lhs - rhs),
            (lhs, rhs) => {
                let degree_multiple = cmp::max(lhs.degree_multiple(), rhs.degree_multiple());
                Self::Sub {
                    x: Arc::new(lhs),
                    y: Arc::new(rhs),
                    degree_multiple,
                }
            }
        }
    }
}

impl<F: FieldImpl + Arithmetic, T> SubAssign<T> for SymbolicExpression<F>
where
    T: Into<Self>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = self.clone() - rhs.into();
    }
}

impl<F: FieldImpl + Arithmetic> Neg for SymbolicExpression<F> {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Self::Constant(c) => Self::Constant(F::zero() - c),
            expr => {
                let degree_multiple = expr.degree_multiple();
                Self::Neg {
                    x: Arc::new(expr),
                    degree_multiple,
                }
            }
        }
    }
}

impl<F: FieldImpl + Arithmetic, T> Mul<T> for SymbolicExpression<F>
where
    T: Into<Self>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        let rhs = rhs.into();
        match (self, rhs) {
            (Self::Constant(lhs), Self::Constant(rhs)) => Self::Constant(lhs * rhs),
            (lhs, rhs) => {
                #[allow(clippy::suspicious_arithmetic_impl)]
                let degree_multiple = lhs.degree_multiple() + rhs.degree_multiple();
                Self::Mul {
                    x: Arc::new(lhs),
                    y: Arc::new(rhs),
                    degree_multiple,
                }
            }
        }
    }
}

impl<F: FieldImpl + Arithmetic, T> MulAssign<T> for SymbolicExpression<F>
where
    T: Into<Self>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = self.clone() * rhs.into();
    }
}

impl<F: FieldImpl + Arithmetic, T> Product<T> for SymbolicExpression<F>
where
    T: Into<Self>,
{
    fn product<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.map(Into::into)
            .reduce(|x, y| x * y)
            .unwrap_or(Self::Constant(F::from_u32(1)))
    }
}
