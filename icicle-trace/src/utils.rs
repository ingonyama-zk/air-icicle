//! A collection of utility functions helpful in defining AIR's.

// This file is based on Plonky3 (https://github.com/Plonky3/Plonky3.git)
// Original authors: Plonky3 authors, 2022
// Modifications by Ingonyama, 2025

use core::array;

use icicle_core::traits::{Arithmetic, FieldImpl};

use crate::AirBuilder;

/// Convert a 32-bit integer into an array of 32 0 or 1 field elements.
///
/// The output array is in little-endian order.
#[inline]
pub fn u32_to_bits_le<FA: FieldImpl + Arithmetic>(val: u32) -> [FA; 32] {
    // We do this over F::from_canonical_u32 as from_canonical_u32 can be slow
    // like in the case of monty field.
    array::from_fn(|i| {
        if val & (1 << i) != 0 {
            FA::one()
        } else {
            FA::zero()
        }
    })
}

/// Verify that `a = b + c + d mod 2^32`
///
/// We assume that a, b, c, d are all given as `2, 16` bit limbs (e.g. `a = a[0] + 2^16 a[1]`) and
/// each `16` bit limb has been range checked to ensure it contains a value in `[0, 2^16)`.
///
/// This function assumes we are working over a field with characteristic `P > 3*2^16`.
#[inline]
pub fn add3<AB: AirBuilder>(
    builder: &mut AB,
    a: &[AB::Var; 2],
    b: &[AB::Var; 2],
    c: &[AB::Expr; 2],
    d: &[AB::Expr; 2],
) {
    // Define:
    //  acc    = a - b - c - d (mod P)
    //  acc_16 = a[0] - b[0] - c[0] - d[0] (mod P)
    //
    // We perform 2 checks:
    //
    // (1) acc*(acc + 2^32)*(acc + 2*2^32) = 0.
    // (2) acc_16*(acc_16 + 2^16)*(acc_16 + 2*2^16) = 0.
    //
    // We give a short proof for why this lets us conclude that a = b + c + d mod 2^32:
    //
    // As all 16 bit limbs have been range checked, we know that a, b, c, d lie in [0, 2^32) and hence
    // a = b + c + d mod 2^32 if and only if, over the integers, a - b - c - d = 0, -2^32 or -2*2^32.
    //
    // Equation (1) verifies that a - b - c - d mod P = 0, -2^32 or -2*2^32.
    //
    // Field overflow cannot occur when computing acc_16 as our characteristic is larger than 3*2^16.
    // Hence, equation (2) verifies that, over the integers, a[0] - b[0] - c[0] - d[0] = 0, -2^16 or -2*2^16.
    // Either way we can immediately conclude that a - b - c - d = 0 mod 2^16.
    //
    // Now we can use the chinese remainder theorem to combine these results to conclude that
    // a - b - c - d mod 2^16P = 0, -2^32 or -2*2^32.
    //
    // No overflow can occur mod 2^16 P as 2^16 P > 3*2^32 and a, b, c, d < 2^32. Hence we conclude that
    // over the integers a - b - c - d = 0, -2^32 or -2*2^32 which implies a = b + c + d mod 2^32.

    // By assumption P > 3*2^16 so we can safely use from_canonical here.
    let two_16 = builder.from_u32(1 << 16);
    let two_32 = two_16.clone() * two_16.clone();

    let acc_16 = a[0].into() - b[0].into() - c[0].clone() - d[0].clone();
    let acc_32 = a[1].into() - b[1].into() - c[1].clone() - d[1].clone();
    let acc = acc_16.clone() + two_16.clone() * acc_32;

    builder.assert_zero(
        acc.clone() * (acc.clone() + two_32.clone()) * (acc + two_32.clone() + two_32.clone()),
    );
    builder.assert_zero(
        acc_16.clone()
            * (acc_16.clone() + two_16.clone())
            * (acc_16 + two_16.clone() + two_16.clone()),
    );
}

/// Verify that `a = b + c mod 2^32`
///
/// We assume that a, b, c are all given as `2, 16` bit limbs (e.g. `a = a[0] + 2^16 a[1]`) and
/// each `16` bit limb has been range checked to ensure it contains a value in `[0, 2^16)`.
///
/// This function assumes we are working over a field with characteristic `P > 2^17`.
#[inline]
pub fn add2<AB: AirBuilder>(
    builder: &mut AB,
    a: &[AB::Var; 2],
    b: &[AB::Var; 2],
    c: &[AB::Expr; 2],
) {
    // Define:
    //  acc    = a - b - c (mod P)
    //  acc_16 = a[0] - b[0] - c[0] (mod P)
    //
    // We perform 2 checks:
    //
    // (1) acc*(acc + 2^32) = 0.
    // (2) acc_16*(acc_16 + 2^16) = 0.
    //
    // We give a short proof for why this lets us conclude that a = b + c mod 2^32:
    //
    // As all 16 bit limbs have been range checked, we know that a, b, c lie in [0, 2^32) and hence
    // a = b + c mod 2^32 if and only if, over the integers, a - b - c = 0 or -2^32.
    //
    // Equation (1) verifies that either a - b - c = 0 mod P or a - b - c = -2^32 mod P.
    //
    // Field overflow cannot occur when computing acc_16 as our characteristic is larger than 2^17.
    // Hence, equation (2) verifies that, over the integers, a[0] - b[0] - c[0] = 0 or -2^16.
    // Either way we can immediately conclude that a - b - c = 0 mod 2^16.
    //
    // Now we can use the chinese remainder theorem to combine these results to conclude that
    // either a - b - c = 0 mod 2^16 P or a - b - c = -2^32 mod 2^16 P.
    //
    // No overflow can occur mod 2^16 P as 2^16 P > 2^33 and a, b, c < 2^32. Hence we conclude that
    // over the integers a - b - c = 0 or a - b - c = -2^32 which is equivalent to a = b + c mod 2^32.

    // By assumption P > 2^17 so we can safely use from_canonical here.
    let two_16 = builder.from_u32(1 << 16);
    let two_32 = two_16.clone() * two_16.clone();

    let acc_16 = a[0].into() - b[0].into() - c[0].clone();
    let acc_32 = a[1].into() - b[1].into() - c[1].clone();
    let acc = acc_16.clone() + two_16.clone() * acc_32;

    builder.assert_zero(acc.clone() * (acc + two_32));
    builder.assert_zero(acc_16.clone() * (acc_16 + two_16));
}

// Verify that `a = (b ^ (c << shift))`
// We assume that a is given as `2 16` bit limbs and both b and c are unpacked into 32 individual bits.
// We assume that the bits of b have been range checked but not the inputs in c or a. Both of these are
// range checked as part of this function.

#[inline]
pub fn xor_32_shift<AB: AirBuilder>(
    builder: &mut AB,
    a: &[AB::Var; 2],
    b: &[AB::Var; 32],
    c: &[AB::Var; 32],
    shift: usize,
) {
    // First we range check all elements of c.
    c.iter().for_each(|elem| builder.assert_bool(elem.clone()));

    // Next we compute (b ^ (c << shift)) and pack the result into two 16-bit integers.
    let xor_shift_c_0_16 = b[..16]
        .iter()
        .enumerate()
        .map(|(i, elem)| builder.xor(elem.clone(), c[(32 + i - shift) % 32].clone()));
    let sum_0_16 = builder.pack_bits_le(xor_shift_c_0_16);

    let xor_shift_c_16_32 = b[16..]
        .iter()
        .enumerate()
        .map(|(i, elem)| {
            builder.xor(
                elem.clone(),
                c[(32 + (i + 16) - shift) % 32].clone(),
            )
        });
    let sum_16_32 = builder.pack_bits_le(xor_shift_c_16_32);

    // As both b and c have been range checked to be boolean, all the (b ^ (c << shift))
    // are also boolean and so this final check additionally has the effect of range checking a[0], a[1].
    builder.assert_eq(a[0].into(), sum_0_16);
    builder.assert_eq(a[1].into(), sum_16_32);
}

/// Returns `[0, ..., N - 1]`.
#[must_use]
pub const fn indices_arr<const N: usize>() -> [usize; N] {
    let mut indices_arr = [0; N];
    let mut i = 0;
    while i < N {
        indices_arr[i] = i;
        i += 1;
    }
    indices_arr
}

/// Convert a 64-bit integer into an array of four field elements representing the 16 bit limb decomposition.
///
/// The output array is in little-endian order.
#[inline]
pub fn u64_to_16_bit_limbs<R: FieldImpl + Arithmetic>(val: u64) -> [R; 4] {
    array::from_fn(|i| R::from_u32((val >> (16 * i)) as u16 as u32))
}

/// Convert a 64-bit integer into an array of 64 0 or 1 field elements.
///
/// The output array is in little-endian order.
#[inline]
pub fn u64_to_bits_le<R: FieldImpl + Arithmetic>(val: u64) -> [R; 64] {
    array::from_fn(|i| R::from_u32((val & (1 << i) != 0) as u32))
}
