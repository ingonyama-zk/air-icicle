// This file is based on Plonky3 (https://github.com/Plonky3/Plonky3.git)
// Original authors: Plonky3 authors, 2022
// Modifications by Ingonyama, 2025

use crate::air::{Air, AirBuilder, AirBuilderWithPublicValues};
use alloc::vec::Vec;
use p3_matrix::dense::{RowMajorMatrix, RowMajorMatrixView};
use p3_matrix::stack::VerticalPair;
use p3_matrix::Matrix;
use tracing::instrument;

use icicle_core::traits::{Arithmetic, FieldImpl};

pub fn from_bool<F: FieldImpl + Arithmetic>(b: bool) -> F {
    if b {
        F::one()
    } else {
        F::zero()
    }
}

#[instrument(name = "check constraints", skip_all)]
pub(crate) fn check_constraints<F, A>(air: &A, main: &RowMajorMatrix<F>, public_values: &Vec<F>)
where
    F: FieldImpl + Arithmetic,
    A: for<'a> Air<DebugConstraintBuilder<'a, F>>,
{
    let height = main.height();

    (0..height).for_each(|i| {
        let i_next = (i + 1) % height;

        let local = main.row_slice(i);
        let next = main.row_slice(i_next);
        let main = VerticalPair::new(
            RowMajorMatrixView::new_row(&*local),
            RowMajorMatrixView::new_row(&*next),
        );

        let mut builder = DebugConstraintBuilder {
            row_index: i,
            main,
            public_values,
            is_first_row: from_bool::<F>(i == 0),
            is_last_row: from_bool::<F>(i == height - 1),
            is_transition: from_bool::<F>(i != height - 1),
        };

        air.eval(&mut builder);
    });
}

/// An `AirBuilder` which asserts that each constraint is zero, allowing any failed constraints to
/// be detected early.
#[derive(Debug)]
pub struct DebugConstraintBuilder<'a, F: FieldImpl + Arithmetic> {
    row_index: usize,
    main: VerticalPair<RowMajorMatrixView<'a, F>, RowMajorMatrixView<'a, F>>,
    public_values: &'a [F],
    is_first_row: F,
    is_last_row: F,
    is_transition: F,
}

impl<'a, F> AirBuilder for DebugConstraintBuilder<'a, F>
where
    F: FieldImpl + Arithmetic,
{
    type F = F;
    type Expr = F;
    type Var = F;
    type M = VerticalPair<RowMajorMatrixView<'a, F>, RowMajorMatrixView<'a, F>>;

    fn main(&self) -> Self::M {
        self.main.clone()
    }

    fn is_first_row(&self) -> Self::Expr {
        self.is_first_row.clone()
    }

    fn is_last_row(&self) -> Self::Expr {
        self.is_last_row.clone()
    }

    fn is_transition_window(&self, size: usize) -> Self::Expr {
        if size == 2 {
            self.is_transition.clone()
        } else {
            panic!("only supports a window size of 2")
        }
    }

    fn zero(&self) -> Self::Expr {
        F::zero()
    }
    fn one(&self) -> Self::Expr {
        F::one()
    }
    fn from_u32(&self, val: u32) -> Self::Expr {
        F::from_u32(val)
    }

    fn assert_zero<I: Into<Self::Expr>>(&mut self, x: I) {
        assert_eq!(
            x.into(),
            F::zero(),
            "constraints had nonzero value on row {}",
            self.row_index
        );
    }

    fn assert_eq<I1: Into<Self::Expr>, I2: Into<Self::Expr>>(&mut self, x: I1, y: I2) {
        let x = x.into();
        let y = y.into();
        assert_eq!(
            x, y,
            "values didn't match on row {}: {} != {}",
            self.row_index, x, y
        );
    }
}

impl<F: FieldImpl + Arithmetic> AirBuilderWithPublicValues for DebugConstraintBuilder<'_, F> {
    type PublicVar = Self::F;

    fn public_values(&self) -> &[Self::F] {
        self.public_values
    }
}
