use crate::FPDecimal;
use cosmwasm_std::StdError;
use std::{fmt::Display, str::FromStr};

pub enum RangeEnds {
    BothInclusive,
    MinInclusive,
    MaxInclusive,
    Exclusive,
}

impl Default for RangeEnds {
    fn default() -> Self {
        RangeEnds::BothInclusive
    }
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
