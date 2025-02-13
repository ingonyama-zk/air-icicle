//! An AIR for the Blake-3 permutation. Assumes the field size is between 2^20 and 2^32.

// This file is based on Plonky3 (https://github.com/Plonky3/Plonky3.git)
// Original authors: Plonky3 authors, 2022
// Modifications by Ingonyama, 2025

#![no_std]

extern crate alloc;

mod air;
mod columns;
mod constants;
mod generation;

pub use air::*;
pub use columns::*;
pub use generation::*;
