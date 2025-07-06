use std::borrow::Borrow;

use icicle_core::traits::{Arithmetic, FieldImpl};

use icicle_babybear::field::ScalarField as Fr;
//constraints

use icicle_trace::{air::*, symbolic_builder::*, symbolic_expression::*};

use p3_matrix::dense::RowMajorMatrix;
use p3_matrix::Matrix;

//comment out lib to run this file

pub struct FibonacciAir {}

impl<F: FieldImpl> BaseAir<F> for FibonacciAir {
    fn width(&self) -> usize {
        NUM_FIBONACCI_COLS
    }
}

impl<AB: AirBuilderWithPublicValues> Air<AB> for FibonacciAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let pis = builder.public_values();

        let a = pis[0].clone();
        let b = pis[1].clone();
        let x = pis[2].clone();

        let local_option = main.row_slice(0);
        let next_option = main.row_slice(1);
        let local_slice = local_option.as_ref().expect("row_slice returned None");
        let next_slice = next_option.as_ref().expect("row_slice returned None");
        let local: &FibonacciRow<AB::Var> = (**local_slice).borrow();
        let next: &FibonacciRow<AB::Var> = (**next_slice).borrow();

        let mut when_first_row = builder.when_first_row();

        when_first_row.assert_eq(local.left.clone(), a);
        when_first_row.assert_eq(local.right.clone(), b);

        let mut when_transition = builder.when_transition();

        // a' <- b
        when_transition.assert_eq(local.right.clone(), next.left.clone());

        // b' <- a + b
        when_transition.assert_eq(local.left.clone() + local.right.clone(), next.right.clone());

        builder.when_last_row().assert_eq(local.right.clone(), x);
    }
}

pub fn generate_trace_rows<F: FieldImpl + Arithmetic>(
    a: u32,
    b: u32,
    n: usize,
) -> RowMajorMatrix<F> {
    assert!(n.is_power_of_two());
    let mut trace =
        RowMajorMatrix::new(vec![F::zero(); n * NUM_FIBONACCI_COLS], NUM_FIBONACCI_COLS);

    let (prefix, rows, suffix) = unsafe { trace.values.align_to_mut::<FibonacciRow<F>>() };
    assert!(prefix.is_empty(), "Alignment should match");
    assert!(suffix.is_empty(), "Alignment should match");
    assert_eq!(rows.len(), n);

    rows[0] = FibonacciRow::new(F::from_u32(a), F::from_u32(b));

    for i in 1..n {
        rows[i].left = rows[i - 1].right.clone();
        rows[i].right = rows[i - 1].left.clone() + rows[i - 1].right.clone();
    }
    trace
}

const NUM_FIBONACCI_COLS: usize = 2;

pub struct FibonacciRow<F> {
    pub left: F,
    pub right: F,
}

impl<F> FibonacciRow<F> {
    const fn new(left: F, right: F) -> FibonacciRow<F> {
        FibonacciRow { left, right }
    }
}
impl<F> Borrow<FibonacciRow<F>> for [F] {
    fn borrow(&self) -> &FibonacciRow<F> {
        debug_assert_eq!(self.len(), NUM_FIBONACCI_COLS);
        let (prefix, shorts, suffix) = unsafe { self.align_to::<FibonacciRow<F>>() };
        debug_assert!(prefix.is_empty(), "Alignment should match");
        debug_assert!(suffix.is_empty(), "Alignment should match");
        debug_assert_eq!(shorts.len(), 1);
        &shorts[0]
    }
}

fn test_trace(n: usize, x: u32) {
    //generate trace
    let trace = generate_trace_rows::<Fr>(0, 1, n);
    //input public values

    let pis = vec![Fr::from_u32(0), Fr::from_u32(1), Fr::from_u32(x)];
    println!("Trace {:#?}", trace);
    let symbolic_constraints =
        get_symbolic_constraints::<Fr, FibonacciAir>(&FibonacciAir {}, 0, pis.len());
    println!("symbolic constraints {:#?}", symbolic_constraints);

    let constraint_degree = symbolic_constraints
        .iter()
        .map(SymbolicExpression::degree_multiple)
        .max()
        .unwrap_or(0);
    println!("Trace dimensions {:?}", trace.dimensions());
    println!("Constraint count: {:#?}", symbolic_constraints.len());
    println!("Constraint degree: {:#?}", constraint_degree);
    println!("Trace eval domain: {:#?}",trace.height());
}

fn main() {
    test_trace(1 << 3, 21);
}
