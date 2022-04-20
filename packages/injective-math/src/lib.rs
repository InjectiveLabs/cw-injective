pub mod fp_decimal;
pub mod vector;

use cosmwasm_std::{StdResult, Uint128};
pub use fp_decimal::*;
use std::str::FromStr;
pub use vector::*;

/// ## Description
/// Calculates the cluster imbalance.
///
/// ## Params
/// - **i** is a reference to an array containing objects of type [`FPDecimal`] which
///     is the asset inventory.
///
/// - **p** is a reference to an array containing objects of type [`FPDecimal`] which
///     are the prices of the assets.
///
/// - **w** is a reference to an array containing objects of type [`FPDecimal`] which
///     are the target weights of the assets.
pub fn imbalance(i: &[FPDecimal], p: &[FPDecimal], w: &[FPDecimal]) -> FPDecimal {
    // Target weights with prices
    // -- u = elem_mul(targets, prices)
    let u = mul(w, p);
    // NAV with target weights instead of inventory
    // -- wp = dot(targets, prices)
    let wp = dot(w, p);

    // Suppose
    // A is the capital allocation
    // -- A = elem_mul(inventory, prices)
    // A_opt is the optimal capital allocation
    // -- A_opt = u * rescale_to_actual_NAV
    //          = u * dot(inventory, prices) / wp

    // Compute imbalance
    // -- imb = | A_opt - A |
    //        = | u * dot(inventory, prices) / wp - elem_mul(inventory, prices) |
    //        = | u * dot(inventory, prices) - elem_mul(inventory, prices) * wp | / wp
    let err_portfolio = sub(&mul_const(&u, dot(i, p)), &mul_const(&mul(i, p), wp));
    sum(&abs(&err_portfolio)) / wp
}

/// ## Description
/// Converts an int32 array to a FPDecimal array.
///
/// ## Params
/// - **arr** is a reference to an array containing objects of type [`u32`].
pub fn int32_vec_to_fpdec(arr: &[u32]) -> Vec<FPDecimal> {
    arr.iter()
        .map(|val| FPDecimal::from(*val as u128))
        .collect()
}

/// ## Description
/// Converts an Uint128 array to a FPDecimal array.
///
/// ## Params
/// - **arr** is a reference to an array containing objects of type [`Uint128`].
pub fn int_vec_to_fpdec(arr: &[Uint128]) -> Vec<FPDecimal> {
    arr.iter().map(|val| FPDecimal::from(val.u128())).collect()
}

/// ## Description
/// Converts an String array to a FPDecimal array.
///
/// ## Params
/// - **arr** is a reference to an array containing objects of type [`String`].
pub fn str_vec_to_fpdec(arr: &[String]) -> StdResult<Vec<FPDecimal>> {
    arr.iter()
        .map(|val| FPDecimal::from_str(val))
        .collect::<StdResult<Vec<FPDecimal>>>()
}
