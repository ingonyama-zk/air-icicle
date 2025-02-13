use icicle_blake3_air::Blake3Air;

use icicle_babybear::field::ScalarField as Fr;

use icicle_trace::{symbolic_builder::*,symbolic_expression::*};
use p3_matrix::Matrix;

fn test_trace(n: usize) {
    let blake3_air = Blake3Air {};
    let trace = blake3_air.generate_trace_rows::<Fr>(n);
    let symbolic_constraints = get_symbolic_constraints::<Fr, Blake3Air>(&Blake3Air {}, 0, 0);

    let constraint_degree = symbolic_constraints
    .iter()
    .map(SymbolicExpression::degree_multiple)
    .max()
    .unwrap_or(0);

    println!("Trace dimensions {:?}", trace.dimensions());
    println!("constraint count {:?}", symbolic_constraints.len());
    println!("constraint degree {:?}",constraint_degree);
    println!("Trace eval domain: {:#?}", trace.height());
}

fn main() {
    test_trace(1 << 8);
}
