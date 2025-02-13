use icicle_keccak_air::KeccakAir;

use icicle_babybear::field::ScalarField as Fr;

use icicle_trace::{symbolic_builder::*,symbolic_expression::*};
use p3_matrix::Matrix;

fn test_trace(n: usize) {
    let keccak_air = KeccakAir {};
    let trace = keccak_air.generate_trace_rows::<Fr>(n, 0);
  

    let symbolic_constraints = get_symbolic_constraints::<Fr, KeccakAir>(&KeccakAir {}, 0, 0);
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
