//! APIs for AIRs, and generalizations like PAIRs.
//!
// This file is based on Plonky3 (https://github.com/Plonky3/Plonky3.git)
// Original authors: Plonky3 authors, 2022
// Modifications by Ingonyama, 2025

#![no_std]

extern crate alloc;

pub mod air;
pub mod check_constraints;
pub mod utils;
mod virtual_column;

pub mod symbolic_builder;
pub mod symbolic_expression;
pub mod symbolic_variable;

pub use air::*;
pub use symbolic_builder::*;
pub use symbolic_expression::*;
pub use symbolic_variable::*;
pub use virtual_column::*;

#[cfg(debug_assertions)]
pub use check_constraints::*;
