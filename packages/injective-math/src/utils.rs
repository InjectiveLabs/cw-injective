use crate::FPDecimal;
use cosmwasm_std::StdError;
use std::{fmt::Display, str::FromStr};

pub fn parse_dec(vs: &str, min: Option<&FPDecimal>, max: Option<&FPDecimal>) -> Result<FPDecimal, StdError> {
    let v = FPDecimal::from_str(vs)?;
    ensure_band(&v, min, max)?;
    Ok(v)
}

pub fn parse_int<T: FromStr + Ord + Display>(vs: &str, min: Option<&T>, max: Option<&T>) -> Result<T, StdError>
where
    <T as FromStr>::Err: ToString,
{
    match vs.parse::<T>() {
        Ok(v) => {
            ensure_band(&v, min, max)?;
            Ok(v)
        }
        Err(e) => Err(StdError::generic_err(e.to_string())),
    }
}

pub fn ensure_band<T: Ord + Display>(v: &T, min: Option<&T>, max: Option<&T>) -> Result<(), StdError> {
    if let Some(minv) = min {
        if v < minv {
            return Err(StdError::generic_err(format!("value {} must be >= {}", v, minv)));
        }
    }
    if let Some(maxv) = max {
        if v > maxv {
            return Err(StdError::generic_err(format!("value {} must be <= {}", v, maxv)));
        }
    }
    Ok(())
}
