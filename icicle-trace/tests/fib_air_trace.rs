use p3_air::{Air, AirBuilder, AirBuilderWithPublicValues, BaseAir};
use std::borrow::Borrow;

use p3_baby_bear::BabyBear;
use p3_field::{integers::QuotientMap, PrimeField64};
use p3_matrix::dense::RowMajorMatrix;
use p3_matrix::Matrix;

use p3_uni_stark::{get_symbolic_constraints, SymbolicExpression};

/// For testing the public values feature
pub struct FibonacciAir {}

impl<F> BaseAir<F> for FibonacciAir {
    fn width(&self) -> usize {
        NUM_FIBONACCI_COLS
    }
}

impl<AB: AirBuilderWithPublicValues> Air<AB> for FibonacciAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let pis = builder.public_values();

        let a = pis[0];
        let b = pis[1];
        let x = pis[2];

        let (local, next) = (main.row_slice(0), main.row_slice(1));
        let local: &FibonacciRow<AB::Var> = (*local).borrow();
        let next: &FibonacciRow<AB::Var> = (*next).borrow();

        let mut when_first_row = builder.when_first_row();

        when_first_row.assert_eq(local.left, a);
        when_first_row.assert_eq(local.right, b);

        let mut when_transition = builder.when_transition();

        // a' <- b
        when_transition.assert_eq(local.right, next.left);

        // b' <- a + b
        when_transition.assert_eq(local.left + local.right, next.right);

        builder.when_last_row().assert_eq(local.right, x);
    }
}

pub fn generate_trace_rows<F: PrimeField64>(a: u64, b: u64, n: usize) -> RowMajorMatrix<F> {
    assert!(n.is_power_of_two());

    let mut trace = RowMajorMatrix::new(F::zero_vec(n * NUM_FIBONACCI_COLS), NUM_FIBONACCI_COLS);

    let (prefix, rows, suffix) = unsafe { trace.values.align_to_mut::<FibonacciRow<F>>() };
    assert!(prefix.is_empty(), "Alignment should match");
    assert!(suffix.is_empty(), "Alignment should match");
    assert_eq!(rows.len(), n);

    rows[0] = FibonacciRow::new(unsafe { F::from_canonical_unchecked(a) }, unsafe {
        F::from_canonical_unchecked(b)
    });

    for i in 1..n {
        rows[i].left = rows[i - 1].right;
        rows[i].right = rows[i - 1].left + rows[i - 1].right;
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
type Val = BabyBear;

/// n-th Fibonacci number expected to be x
fn test_trace(n: usize, x: u64) {
    //generate trace
    let trace = generate_trace_rows::<Val>(0, 1, n);
    //input public values
    let pis = vec![
        unsafe { BabyBear::from_canonical_unchecked(0) },
        BabyBear::from_canonical_checked(1).unwrap(),
        BabyBear::from_canonical_checked(x).unwrap(),
    ];
    println!("Trace {:#?}", trace);
    let symbolic_constraints =
        get_symbolic_constraints::<BabyBear, FibonacciAir>(&FibonacciAir {}, 0, pis.len());
    println!("symbolic constraints {:#?}", symbolic_constraints);

    let constraint_count = symbolic_constraints.len();
    let constraint_degree = symbolic_constraints
        .iter()
        .map(SymbolicExpression::degree_multiple)
        .max()
        .unwrap_or(0);
    println!("Constraint count: {:#?}", constraint_count);
    println!("Constraint degree: {:#?}", constraint_degree);
    let degree = trace.height();
    println!("Trace eval domain: {:#?}", degree);
}

//run after commenting out lib

#[test]
fn test_public_value() {
    test_trace(1 << 3, 21);
}
