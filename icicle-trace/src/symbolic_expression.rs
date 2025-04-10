// This file is based on Plonky3 (https://github.com/Plonky3/Plonky3.git)
// Original authors: Plonky3 authors, 2022
// Modifications by Ingonyama, 2025

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

    pub fn zero() -> Self {
        Self::Constant(F::zero())
    }

    pub fn one() -> Self {
        Self::Constant(F::one())
    }

    pub fn from_u32(val: u32) -> Self {
        Self::Constant(F::from_u32(val))
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

impl<F: FieldImpl + Arithmetic, T> Add<T> for SymbolicExpression<F>
where
    T: Into<Self>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self {
        let rhs = rhs.into();
        if let Self::Constant(c) = &self {
            if c == &F::zero() {
                return rhs;
            }
        }
        if let Self::Constant(c) = &rhs {
            if c == &F::zero() {
                return self;
            }
        }

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
            .fold(Self::zero(), |x, y| x + y)
    }
}

impl<F: FieldImpl + Arithmetic, T> Sub<T> for SymbolicExpression<F>
where
    T: Into<Self>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self {
        let rhs = rhs.into();
        if let Self::Constant(c) = &rhs {
            if c == &F::zero() {
                return self;
            }
        }

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
        if let Self::Constant(c) = &self {
             if c == &F::zero() {
                 return self;
             }
        }
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

        if let Self::Constant(c) = &self {
            if c == &F::one() {
                return rhs;
            }
            if c == &F::zero() {
                return Self::zero();
            }
        }
         if let Self::Constant(c) = &rhs {
            if c == &F::one() {
                return self;
            }
             if c == &F::zero() {
                return Self::zero();
            }
        }

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
            .fold(Self::one(), |x, y| x * y)
    }
}
