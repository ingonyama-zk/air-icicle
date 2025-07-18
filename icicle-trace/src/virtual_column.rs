// This file is based on Plonky3 (https://github.com/Plonky3/Plonky3.git)
// Original authors: Plonky3 authors, 2022
// Modifications by Ingonyama, 2025

use alloc::vec;
use alloc::vec::Vec;
use core::ops::Mul;

use icicle_core::traits::Arithmetic;
use icicle_core::field::Field;
use icicle_core::bignum::BigNum;

/// An affine function over columns in a PAIR.
#[derive(Clone, Debug)]
pub struct VirtualPairCol<F: Field + Arithmetic> {
    column_weights: Vec<(PairCol, F)>,
    constant: F,
}

/// A column in a PAIR, i.e. either a preprocessed column or a main trace column.
#[derive(Clone, Copy, Debug)]
pub enum PairCol {
    Preprocessed(usize),
    Main(usize),
}

impl PairCol {
    pub const fn get<T: Copy>(&self, preprocessed: &[T], main: &[T]) -> T {
        match self {
            PairCol::Preprocessed(i) => preprocessed[*i],
            PairCol::Main(i) => main[*i],
        }
    }
}

impl<F: Field + Arithmetic> VirtualPairCol<F> {
    pub const fn new(column_weights: Vec<(PairCol, F)>, constant: F) -> Self {
        Self {
            column_weights,
            constant,
        }
    }

    pub fn new_preprocessed(column_weights: Vec<(usize, F)>, constant: F) -> Self {
        Self::new(
            column_weights
                .into_iter()
                .map(|(i, w)| (PairCol::Preprocessed(i), w))
                .collect(),
            constant,
        )
    }

    pub fn new_main(column_weights: Vec<(usize, F)>, constant: F) -> Self {
        Self::new(
            column_weights
                .into_iter()
                .map(|(i, w)| (PairCol::Main(i), w))
                .collect(),
            constant,
        )
    }

    //   pub const ONE: Self = Self::constant(F::one());

    #[must_use]
    pub const fn constant(x: F) -> Self {
        Self {
            column_weights: vec![],
            constant: x,
        }
    }

    #[must_use]
    pub fn single(column: PairCol) -> Self {
        Self {
            column_weights: vec![(column, F::one())],
            constant: F::zero(),
        }
    }

    #[must_use]
    pub fn single_preprocessed(column: usize) -> Self {
        Self::single(PairCol::Preprocessed(column))
    }

    #[must_use]
    pub fn single_main(column: usize) -> Self {
        Self::single(PairCol::Main(column))
    }

    #[must_use]
    pub fn sum_main(columns: Vec<usize>) -> Self {
        let column_weights = columns.into_iter().map(|col| (col, F::one())).collect();
        Self::new_main(column_weights, F::zero())
    }

    #[must_use]
    pub fn sum_preprocessed(columns: Vec<usize>) -> Self {
        let column_weights = columns.into_iter().map(|col| (col, F::one())).collect();
        Self::new_preprocessed(column_weights, F::zero())
    }

    /// `a - b`, where `a` and `b` are columns in the preprocessed trace.
    #[must_use]
    pub fn diff_preprocessed(a_col: usize, b_col: usize) -> Self {
        Self::new_preprocessed(
            vec![(a_col, F::one()), (b_col, F::zero() - F::one())],
            F::zero(),
        )
    }

    /// `a - b`, where `a` and `b` are columns in the main trace.
    #[must_use]
    pub fn diff_main(a_col: usize, b_col: usize) -> Self {
        Self::new_main(
            vec![(a_col, F::one()), (b_col, F::zero() - F::one())],
            F::zero(),
        )
    }

    pub fn apply<Expr, Var>(&self, preprocessed: &[Var], main: &[Var]) -> Expr
    where
        F: Into<Expr>,
        Expr: Field + Arithmetic + Mul<F, Output = Expr>,
        Var: Into<Expr> + Copy,
    {
        let mut result = self.constant.clone().into();
        for (column, weight) in self.column_weights.iter() {
            result = result + column.get(preprocessed, main).into() * (*weight).clone();
        }
        result
    }
}
