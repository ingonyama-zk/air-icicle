// This file is based on Plonky3 (https://github.com/Plonky3/Plonky3.git)
// Original authors: Plonky3 authors, 2022
// Modifications by Ingonyama, 2025

use core::borrow::Borrow;

use icicle_trace::AirBuilder;
use p3_matrix::Matrix;

use crate::columns::KeccakCols;
use crate::NUM_ROUNDS;

#[inline]
pub(crate) fn eval_round_flags<AB: AirBuilder>(builder: &mut AB) {
    let main = builder.main();
    let (local, next) = (main.row_slice(0), main.row_slice(1));
    let local: &KeccakCols<AB::Var> = (*local).borrow();
    let next: &KeccakCols<AB::Var> = (*next).borrow();

    // Initially, the first step flag should be 1 while the others should be 0.
    builder
        .when_first_row()
        .assert_one(local.step_flags[0].clone());
    for i in 1..NUM_ROUNDS {
        builder
            .when_first_row()
            .assert_zero(local.step_flags[i].clone());
    }

    for i in 0..NUM_ROUNDS {
        let current_round_flag = local.step_flags[i].clone();
        let next_round_flag = next.step_flags[(i + 1) % NUM_ROUNDS].clone();
        builder
            .when_transition()
            .assert_eq(next_round_flag, current_round_flag);
    }
}
