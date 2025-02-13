use std::borrow::Borrow;

use icicle_core::traits::{Arithmetic, FieldImpl};

use icicle_babybear::field::ScalarField as Fr;
//constraints
use p3_air::BaseAir;
use p3_matrix::dense::RowMajorMatrix;

//comment out lib to run this file

pub struct FibonacciAir {}

impl<F: FieldImpl> BaseAir<F> for FibonacciAir {
    fn width(&self) -> usize {
        NUM_FIBONACCI_COLS
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

pub struct FibonacciRow<F: FieldImpl + Arithmetic> {
    pub left: F,
    pub right: F,
}

impl<F: FieldImpl + Arithmetic> FibonacciRow<F> {
    const fn new(left: F, right: F) -> FibonacciRow<F> {
        FibonacciRow { left, right }
    }
}
impl<F: FieldImpl + Arithmetic> Borrow<FibonacciRow<F>> for [F] {
    fn borrow(&self) -> &FibonacciRow<F> {
        debug_assert_eq!(self.len(), NUM_FIBONACCI_COLS);
        let (prefix, shorts, suffix) = unsafe { self.align_to::<FibonacciRow<F>>() };
        debug_assert!(prefix.is_empty(), "Alignment should match");
        debug_assert!(suffix.is_empty(), "Alignment should match");
        debug_assert_eq!(shorts.len(), 1);
        &shorts[0]
    }
}

fn main() {
    fn test_trace(n: usize, _x: u32) {
        //generate trace
        let trace = generate_trace_rows::<Fr>(0, 1, n);
        //input public values
        println!("Trace {:#?}", trace);
    }
    test_trace(1 << 3, 21);
}
