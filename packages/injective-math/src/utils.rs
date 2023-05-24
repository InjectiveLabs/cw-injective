use crate::FPDecimal;
use bigint::U256;
use cosmwasm_std::StdError;
use std::{fmt::Display, str::FromStr};

#[derive(Default)]
pub enum RangeEnds {
    #[default]
    BothInclusive,
    MinInclusive,
    MaxInclusive,
    Exclusive,
}

pub fn parse_dec(vs: &str, min: Option<&FPDecimal>, max: Option<&FPDecimal>, range_ends: RangeEnds) -> Result<FPDecimal, StdError> {
    let v = FPDecimal::from_str(vs)?;
    ensure_band(&v, min, max, range_ends)?;
    Ok(v)
}

pub fn parse_int<T: FromStr + Ord + Display>(vs: &str, min: Option<&T>, max: Option<&T>, range_ends: RangeEnds) -> Result<T, StdError>
where
    <T as FromStr>::Err: ToString,
{
    match vs.parse::<T>() {
        Ok(v) => {
            ensure_band(&v, min, max, range_ends)?;
            Ok(v)
        }
        Err(e) => Err(StdError::generic_err(e.to_string())),
    }
}

pub fn ensure_band<T: Ord + Display>(v: &T, min: Option<&T>, max: Option<&T>, range_ends: RangeEnds) -> Result<(), StdError> {
    if let Some(minv) = min {
        match range_ends {
            RangeEnds::BothInclusive | RangeEnds::MinInclusive => {
                if v < minv {
                    return Err(StdError::generic_err(format!("value {v} must be >= {minv}")));
                }
            }
            RangeEnds::MaxInclusive | RangeEnds::Exclusive => {
                if v <= minv {
                    return Err(StdError::generic_err(format!("value {v} must be > {minv}")));
                }
            }
        }
    }
    if let Some(maxv) = max {
        match range_ends {
            RangeEnds::BothInclusive | RangeEnds::MaxInclusive => {
                if v > maxv {
                    return Err(StdError::generic_err(format!("value {v} must be <= {maxv}")));
                }
            }
            RangeEnds::MinInclusive | RangeEnds::Exclusive => {
                if v >= maxv {
                    return Err(StdError::generic_err(format!("value {v} must be < {maxv}")));
                }
            }
        }
    }
    Ok(())
}

pub fn band_error_to_human(err: StdError, value_name: &str) -> StdError {
    StdError::generic_err(format!("Value '{value_name}' failed validation due to: '{err}'"))
}

pub fn div_dec(num: FPDecimal, denom: FPDecimal) -> FPDecimal {
    if denom == FPDecimal::zero() {
        denom
    } else {
        num / denom
    }
}

pub fn round_to_min_tick(num: FPDecimal, min_tick: FPDecimal) -> FPDecimal {
    if num < min_tick {
        FPDecimal::zero()
    } else {
        let shifted = div_dec(num, min_tick).int();
        shifted * min_tick
    }
}

pub fn round_to_nearest_tick(num: FPDecimal, min_tick: FPDecimal) -> FPDecimal {
    if num < min_tick {
        return FPDecimal::zero();
    }

    let remainder = FPDecimal::from(num.num % min_tick.num);
    if remainder.num > min_tick.num / U256::from(2u64) {
        FPDecimal::from(num.num - remainder.num + min_tick.num)
    } else {
        FPDecimal::from(num.num - remainder.num)
    }
}
