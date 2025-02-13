// This file is based on Plonky3 (https://github.com/Plonky3/Plonky3.git)
// Original authors: Plonky3 authors, 2022
// Modifications by Ingonyama, 2025

use core::marker::PhantomData;
use core::ops::{Add, Mul, Sub};

use icicle_core::traits::{Arithmetic, FieldImpl};

use crate::symbolic_expression::SymbolicExpression;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Entry {
    Preprocessed { offset: usize },
    Main { offset: usize },
    Permutation { offset: usize },
    Public,
    Challenge,
}

/// A variable within the evaluation window, i.e. a column in either the local or next row.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SymbolicVariable<F: FieldImpl + Arithmetic> {
    pub entry: Entry,
    pub index: usize,
    pub(crate) _phantom: PhantomData<F>,
}

impl<F: FieldImpl + Arithmetic> SymbolicVariable<F> {
    pub const fn new(entry: Entry, index: usize) -> Self {
        Self {
            entry,
            index,
            _phantom: PhantomData,
        }
    }

    pub const fn degree_multiple(&self) -> usize {
        match self.entry {
            Entry::Preprocessed { .. } | Entry::Main { .. } | Entry::Permutation { .. } => 1,
            Entry::Public | Entry::Challenge => 0,
        }
    }
}

impl<F: FieldImpl + Arithmetic> From<SymbolicVariable<F>> for SymbolicExpression<F> {
    fn from(value: SymbolicVariable<F>) -> Self {
        SymbolicExpression::Variable(value)
    }
}

impl<F: FieldImpl + Arithmetic, T> Add<T> for SymbolicVariable<F>
where
    T: Into<SymbolicExpression<F>>,
{
    type Output = SymbolicExpression<F>;

    fn add(self, rhs: T) -> Self::Output {
        SymbolicExpression::from(self) + rhs.into()
    }
}

impl<F: FieldImpl + Arithmetic, T> Sub<T> for SymbolicVariable<F>
where
    T: Into<SymbolicExpression<F>>,
{
    type Output = SymbolicExpression<F>;

    fn sub(self, rhs: T) -> Self::Output {
        SymbolicExpression::from(self) - rhs.into()
    }
}

impl<F: FieldImpl + Arithmetic, T> Mul<T> for SymbolicVariable<F>
where
    T: Into<SymbolicExpression<F>>,
{
    type Output = SymbolicExpression<F>;

    fn mul(self, rhs: T) -> Self::Output {
        SymbolicExpression::from(self) * rhs.into()
    }
}
