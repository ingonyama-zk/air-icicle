//! APIs for AIRs, and generalizations like PAIRs.

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
pub use virtual_column::*;
pub use symbolic_builder::*;
pub use symbolic_expression::*;
pub use symbolic_variable::*;

#[cfg(debug_assertions)]
pub use check_constraints::*;
