// This file is based on Plonky3 (https://github.com/Plonky3/Plonky3.git)
// Original authors: Plonky3 authors, 2022
// Modifications by Ingonyama, 2025

use core::ops::{Add, Mul, Sub};

use icicle_core::bignum;
use icicle_core::traits::Arithmetic;
use icicle_core::{field::Field,bignum::BigNum};
use p3_matrix::dense::RowMajorMatrix;
use p3_matrix::Matrix;

/// An AIR (algebraic intermediate representation).
pub trait BaseAir<F>: Sync {
    /// The number of columns (a.k.a. registers) in this AIR.
    fn width(&self) -> usize;

    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<F>> {
        None
    }
}

///  An AIR with 0 or more public values.
pub trait BaseAirWithPublicValues<F: Field + Arithmetic>: BaseAir<F> {
    fn num_public_values(&self) -> usize {
        0
    }
}

/// An AIR that works with a particular `AirBuilder`.
pub trait Air<AB: AirBuilder>: BaseAir<AB::F> {
    fn eval(&self, builder: &mut AB);
}

pub trait AirBuilder: Sized {
    type F: Field;

    type Expr: Clone
        + Send
        + Sync
        + From<Self::F>
        + From<Self::Var>
        + Add<Self::Expr, Output = Self::Expr>
        + Add<Self::F, Output = Self::Expr>
        + Add<Self::Var, Output = Self::Expr>
        + Sub<Self::Expr, Output = Self::Expr>
        + Sub<Self::F, Output = Self::Expr>
        + Sub<Self::Var, Output = Self::Expr>
        + Mul<Self::Expr, Output = Self::Expr>
        + Mul<Self::F, Output = Self::Expr>
        + Mul<Self::Var, Output = Self::Expr>;

    type Var: Into<Self::Expr>
        + Clone
        + Copy
        + Send
        + Sync
        + Add<Self::F, Output = Self::Expr>
        + Add<Self::Var, Output = Self::Expr>
        + Add<Self::Expr, Output = Self::Expr>
        + Sub<Self::F, Output = Self::Expr>
        + Sub<Self::Var, Output = Self::Expr>
        + Sub<Self::Expr, Output = Self::Expr>
        + Mul<Self::F, Output = Self::Expr>
        + Mul<Self::Var, Output = Self::Expr>
        + Mul<Self::Expr, Output = Self::Expr>;

    type M: Matrix<Self::Var>;

    fn main(&self) -> Self::M;

    fn is_first_row(&self) -> Self::Expr;
    fn is_last_row(&self) -> Self::Expr;
    fn is_transition(&self) -> Self::Expr {
        self.is_transition_window(2)
    }
    fn is_transition_window(&self, size: usize) -> Self::Expr;

    /// Returns a sub-builder whose constraints are enforced only when `condition` is nonzero.
    fn when<I: Into<Self::Expr>>(&mut self, condition: I) -> FilteredAirBuilder<'_, Self> {
        FilteredAirBuilder {
            inner: self,
            condition: condition.into(),
        }
    }

    /// Returns a sub-builder whose constraints are enforced only when `x != y`.
    fn when_ne<I1: Into<Self::Expr>, I2: Into<Self::Expr>>(
        &mut self,
        x: I1,
        y: I2,
    ) -> FilteredAirBuilder<'_, Self> {
        let x_expr = x.into();
        let y_expr = y.into();
        self.when(x_expr - y_expr)
    }

    /// Returns a sub-builder whose constraints are enforced only on the first row.
    fn when_first_row(&mut self) -> FilteredAirBuilder<'_, Self> {
        self.when(self.is_first_row())
    }

    /// Returns a sub-builder whose constraints are enforced only on the last row.
    fn when_last_row(&mut self) -> FilteredAirBuilder<'_, Self> {
        self.when(self.is_last_row())
    }

    /// Returns a sub-builder whose constraints are enforced on all rows except the last.
    fn when_transition(&mut self) -> FilteredAirBuilder<'_, Self> {
        self.when(self.is_transition())
    }

    /// Returns a sub-builder whose constraints are enforced on all rows except the last `size - 1`.
    fn when_transition_window(&mut self, size: usize) -> FilteredAirBuilder<'_, Self> {
        self.when(self.is_transition_window(size))
    }

    /// Get a constant expression representing zero.
    fn zero(&self) -> Self::Expr;
    /// Get a constant expression representing one.
    fn one(&self) -> Self::Expr;
    /// Get a constant expression representing two.
    fn two(&self) -> Self::Expr {
        self.one() + self.one()
    }
    /// Get a constant expression from a u32.
    fn from_u32(&self, val: u32) -> Self::Expr;

    fn assert_zero<I: Into<Self::Expr>>(&mut self, x: I);

    fn assert_one<I: Into<Self::Expr>>(&mut self, x: I) {
        self.assert_zero(x.into() - self.one());
    }

    fn assert_eq<I1: Into<Self::Expr>, I2: Into<Self::Expr>>(&mut self, x: I1, y: I2) {
        self.assert_zero(x.into() - y.into());
    }

    /// Assert that `x` is a boolean, i.e. either 0 or 1.
    fn assert_bool<I: Into<Self::Expr>>(&mut self, x: I) {
        let x = x.into();
        self.assert_zero(x.clone() * (x - self.one()));
    }

    /// Assert that `x` is ternary, i.e. either 0, 1 or 2.
    fn assert_tern<I: Into<Self::Expr>>(&mut self, x: I) {
        let x = x.into();
        let one = self.one();
        let two = self.two();
        self.assert_zero(x.clone() * (x.clone() - one) * (x - two));
    }

    /// Pack a collection of bits into a number.
    ///
    /// Given vec = [v0, v1, ..., v_n] returns v0 + 2v_1 + ... + 2^n v_n
    #[inline]
    fn pack_bits_le<I>(&self, iter: I) -> Self::Expr
    where
        I: DoubleEndedIterator,
        I::Item: Into<Self::Expr>,
    {
        let mut output = self.zero();
        let two = self.two();
        for elem in iter.rev() {
            output = output.clone() * two.clone(); // Use clone if Expr doesn't impl Copy
            output = output + elem.into();
        }
        output
    }

    /// Computes the arithmetic generalization of boolean `xor`.
    ///
    /// For boolean inputs, `x ^ y = x + y - 2 xy`.
    #[inline(always)]
    fn xor<X, Y>(&self, x: X, y: Y) -> Self::Expr
    where
        X: Into<Self::Expr>,
        Y: Into<Self::Expr>,
    {
        let x = x.into();
        let y = y.into();
        let two = self.two();
        x.clone() + y.clone() - two * x * y
    }

    /// Computes the arithmetic generalization of a triple `xor`.
    ///
    /// For boolean inputs `x ^ y ^ z = x + y + z - 2(xy + xz + yz) + 4xyz`.
    #[inline(always)]
    fn xor3<X, Y, Z>(&self, x: X, y: Y, z: Z) -> Self::Expr
    where
        X: Into<Self::Expr>,
        Y: Into<Self::Expr>,
        Z: Into<Self::Expr>,
    {
        // The cheapest way to implement this polynomial is to simply apply xor twice.
        // This costs 2 adds, 2 subs, 2 muls and 2 doubles.
        self.xor(x, self.xor(y, z))
    }

    /// Computes the arithmetic generalization of `andnot`.
    ///
    /// For boolean inputs `(!x) & y = (1 - x)y`
    #[inline(always)]
    fn andn<X, Y>(&self, x: X, y: Y) -> Self::Expr
    where
        X: Into<Self::Expr>,
        Y: Into<Self::Expr>,
    {
        let one = self.one();
        let x = x.into();
        let y = y.into();
        (one - x) * y
    }
}

pub trait AirBuilderWithPublicValues: AirBuilder {
    type PublicVar: Into<Self::Expr> + Clone + Send + Sync;

    fn public_values(&self) -> &[Self::PublicVar];
}

pub trait PairBuilder: AirBuilder {
    fn preprocessed(&self) -> Self::M;
}

#[derive(Debug)]
pub struct FilteredAirBuilder<'a, AB: AirBuilder> {
    pub inner: &'a mut AB,
    condition: AB::Expr,
}

impl<AB: AirBuilder> FilteredAirBuilder<'_, AB> {
    pub fn condition(&self) -> AB::Expr {
        self.condition.clone()
    }

    pub fn main(&self) -> AB::M {
        self.inner.main()
    }

    pub fn is_first_row(&self) -> AB::Expr {
        self.inner.is_first_row()
    }

    pub fn is_last_row(&self) -> AB::Expr {
        self.inner.is_last_row()
    }

    pub fn is_transition_window(&self, size: usize) -> AB::Expr {
        self.inner.is_transition_window(size)
    }

    pub fn zero(&self) -> AB::Expr {
        self.inner.zero()
    }
    pub fn one(&self) -> AB::Expr {
        self.inner.one()
    }
    pub fn two(&self) -> AB::Expr {
        self.inner.two()
    }
    pub fn from_u32(&self, val: u32) -> AB::Expr {
        self.inner.from_u32(val)
    }

    pub fn assert_zero<I: Into<AB::Expr>>(&mut self, x: I) {
        self.inner.assert_zero(self.condition() * x.into());
    }

    pub fn assert_eq<I1: Into<AB::Expr>, I2: Into<AB::Expr>>(&mut self, x: I1, y: I2) {
        self.assert_zero(x.into() - y.into());
    }

    pub fn assert_one<I: Into<AB::Expr>>(&mut self, x: I) {
        self.assert_zero(x.into() - self.one());
    }
}
