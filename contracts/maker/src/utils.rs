use std::str::FromStr;

use cosmwasm_std::{Decimal256 as Decimal, Deps, Fraction, Uint256};
use injective_bindings::InjectiveQueryWrapper;

use crate::state::{config_read, State};

pub fn wrap(unwrapped_num: &String, deps: Deps<InjectiveQueryWrapper>) -> Decimal {
    let state = config_read(deps.storage).load().unwrap();
    Decimal::from_str(unwrapped_num).unwrap() / state.decimal_shift
}

pub fn wrap_from_state(unwrapped_num: &String, state: &State) -> Decimal {
    Decimal::from_str(unwrapped_num).unwrap()
        * Decimal::from_ratio(Uint256::from_str("1").unwrap(), state.decimal_shift)
}

pub fn div_int(num: Decimal, denom: Uint256) -> Decimal {
    num * Decimal::from_ratio(Uint256::from_str("1").unwrap(), denom)
}

pub fn div_dec(num: Decimal, denom: Decimal) -> Decimal {
    num * denom.inv().unwrap()
}

pub fn sub_abs(lhs: Decimal, rhs: Decimal) -> Decimal {
    if lhs > rhs {
        lhs - rhs
    } else {
        rhs - lhs
    }
}

pub fn round_to_precision(num: Decimal, precision_shift: Uint256) -> Decimal {
    let shifted = num * precision_shift;
    let shifted = Decimal::from_str(&shifted.to_string()).unwrap();
    shifted
        * Decimal::from_str(&precision_shift.to_string())
            .unwrap()
            .inv()
            .unwrap()
}
