use crate::FPDecimal;
use bigint::U256;
use cosmwasm_std::StdError;
use std::cmp::Ordering;
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

pub fn floor(num: FPDecimal, min_tick: FPDecimal) -> FPDecimal {
    // min_tick has to be a positive number
    assert!(min_tick >= FPDecimal::ZERO);
    if num.is_zero() {
        return num;
    }
    let remainder = num % min_tick;
    num - remainder
}

pub fn round(num: FPDecimal, min_tick: FPDecimal) -> FPDecimal {
    let num_floor = floor(num, min_tick);
    let diff = num - num_floor;
    match diff.cmp(&(min_tick / FPDecimal::TWO)) {
        Ordering::Less => num_floor,
        Ordering::Equal => {
            if num_floor / (min_tick * FPDecimal::TWO) == FPDecimal::ZERO {
                num_floor
            } else {
                num_floor + min_tick
            }
        }
        Ordering::Greater => num_floor + min_tick,
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

pub fn round_up_to_min_tick(num: FPDecimal, min_tick: FPDecimal) -> FPDecimal {
    if num < min_tick {
        return min_tick;
    }

    let remainder = FPDecimal::from(num.num % min_tick.num);

    if remainder.num.is_zero() {
        return num;
    }

    FPDecimal::from(num.num - remainder.num + min_tick.num)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fp_decimal::scale::Scaled;

    #[test]
    fn test_floor() {
        assert_eq!(floor(FPDecimal::must_from_str("0"), FPDecimal::must_from_str("0.1")), FPDecimal::ZERO);
        assert_eq!(
            floor(FPDecimal::must_from_str("0.13"), FPDecimal::must_from_str("0.1")),
            FPDecimal::must_from_str("0.1")
        );
        assert_eq!(
            floor(FPDecimal::must_from_str("0.19"), FPDecimal::must_from_str("0.1")),
            FPDecimal::must_from_str("0.1")
        );
        assert_eq!(
            floor(FPDecimal::must_from_str("1.19"), FPDecimal::must_from_str("0.1")),
            FPDecimal::must_from_str("1.1")
        );
        assert_eq!(floor(FPDecimal::must_from_str("2.19"), FPDecimal::ONE), FPDecimal::TWO);

        assert_eq!(floor(FPDecimal::must_from_str("-0"), FPDecimal::must_from_str("0.1")), FPDecimal::ZERO);
        assert_eq!(
            floor(FPDecimal::must_from_str("-0.13"), FPDecimal::must_from_str("0.1")),
            FPDecimal::must_from_str("-0.2")
        );
        assert_eq!(
            floor(FPDecimal::must_from_str("-0.19"), FPDecimal::must_from_str("0.1")),
            FPDecimal::must_from_str("-0.2")
        );
        assert_eq!(
            floor(FPDecimal::must_from_str("-1.19"), FPDecimal::must_from_str("0.1")),
            FPDecimal::must_from_str("-1.2")
        );
        assert_eq!(
            floor(FPDecimal::must_from_str("-2.19"), FPDecimal::must_from_str("0.1")),
            FPDecimal::must_from_str("-2.2")
        );

        assert_eq!(floor(FPDecimal::must_from_str("-2.19"), FPDecimal::ONE), FPDecimal::must_from_str("-3"));
    }

    #[test]
    fn test_round() {
        assert_eq!(round(FPDecimal::must_from_str("0.13"), FPDecimal::ONE), FPDecimal::ZERO);
        assert_eq!(round(FPDecimal::must_from_str("0.49"), FPDecimal::ONE), FPDecimal::ZERO);
        assert_eq!(round(FPDecimal::must_from_str("0.5"), FPDecimal::ONE), FPDecimal::ZERO);
        assert_eq!(round(FPDecimal::must_from_str("0.50009"), FPDecimal::ONE), FPDecimal::ONE);

        assert_eq!(round(FPDecimal::must_from_str("-0.13"), FPDecimal::ONE), FPDecimal::ZERO);
        assert_eq!(round(FPDecimal::must_from_str("-0.49"), FPDecimal::ONE), FPDecimal::ZERO);
        assert_eq!(round(FPDecimal::must_from_str("-0.5"), FPDecimal::ONE), FPDecimal::ZERO);
        assert_eq!(round(FPDecimal::must_from_str("-0.51"), FPDecimal::ONE), -FPDecimal::ONE);
        assert_eq!(round(FPDecimal::must_from_str("-1.50009"), FPDecimal::ONE), -FPDecimal::TWO);
    }

    #[test]
    fn test_round_with_scaled_numbers() {
        assert_eq!(round(FPDecimal::must_from_str("0"), FPDecimal::must_from_str("0.1")), FPDecimal::ZERO);
        assert_eq!(
            round(FPDecimal::must_from_str("0.13"), FPDecimal::must_from_str("0.1")),
            FPDecimal::must_from_str("0.1")
        );
        assert_eq!(
            round(FPDecimal::must_from_str("0.50009"), FPDecimal::must_from_str("0.0001")),
            FPDecimal::must_from_str("0.5001")
        );

        assert_eq!(round(FPDecimal::must_from_str("-0"), FPDecimal::must_from_str("0.1")), FPDecimal::ZERO);
        assert_eq!(
            round(FPDecimal::must_from_str("-0.13"), FPDecimal::must_from_str("0.1")),
            FPDecimal::must_from_str("-0.1")
        );
        assert_eq!(
            round(FPDecimal::must_from_str("-0.50009"), FPDecimal::must_from_str("0.0001")),
            FPDecimal::must_from_str("-0.5001")
        );

        assert_eq!(round(FPDecimal::must_from_str("-1.50009"), FPDecimal::ONE.scaled(1)), FPDecimal::ZERO);
        assert_eq!(
            round(FPDecimal::must_from_str("-1.50009").scaled(1), FPDecimal::ONE.scaled(1)),
            -FPDecimal::TWO.scaled(1)
        );
        assert_eq!(round(FPDecimal::must_from_str("-1.50009"), FPDecimal::ONE.scaled(1)), FPDecimal::ZERO);
        assert_eq!(
            round(FPDecimal::must_from_str("-1.50009").scaled(1), FPDecimal::ONE.scaled(1)),
            -FPDecimal::TWO.scaled(1)
        );
    }

    #[test]
    fn test_div_dec() {
        assert_eq!(
            div_dec(FPDecimal::must_from_str("6"), FPDecimal::must_from_str("2")),
            FPDecimal::must_from_str("3")
        );
        assert_eq!(
            div_dec(FPDecimal::must_from_str("7"), FPDecimal::must_from_str("0")),
            FPDecimal::must_from_str("0")
        );
        assert_eq!(
            div_dec(FPDecimal::must_from_str("7.5"), FPDecimal::must_from_str("2.5")),
            FPDecimal::must_from_str("3.0")
        );
    }

    #[test]
    fn test_round_to_min_tick() {
        assert_eq!(
            round_to_min_tick(FPDecimal::must_from_str("7.7"), FPDecimal::must_from_str("2.0")),
            FPDecimal::must_from_str("6.0")
        );
        assert_eq!(
            round_to_min_tick(FPDecimal::must_from_str("1.5"), FPDecimal::must_from_str("2.0")),
            FPDecimal::must_from_str("0.0")
        );
        assert_eq!(
            round_to_min_tick(FPDecimal::must_from_str("10.0"), FPDecimal::must_from_str("3.0")),
            FPDecimal::must_from_str("9.0")
        );
    }

    #[test]
    fn round_to_nearest_tick_test() {
        assert_eq!(
            round_to_nearest_tick(FPDecimal::must_from_str("7.7"), FPDecimal::must_from_str("2.0")),
            FPDecimal::must_from_str("8.0")
        );
        assert_eq!(
            round_to_nearest_tick(FPDecimal::must_from_str("1.5"), FPDecimal::must_from_str("2.0")),
            FPDecimal::must_from_str("0.0")
        );
        assert_eq!(
            round_to_nearest_tick(FPDecimal::must_from_str("2.5"), FPDecimal::must_from_str("2.0")),
            FPDecimal::must_from_str("2.0")
        );
        assert_eq!(
            round_to_nearest_tick(FPDecimal::must_from_str("10.0"), FPDecimal::must_from_str("3.0")),
            FPDecimal::must_from_str("9.0")
        );
        // input, expected
        let data = vec![
            ["1.09932", "1.1"],
            ["2.032", "2.03"],
            ["1.0009932", "1"],
            ["1.009932", "1.01"],
            ["0.9932", "0.99"],
        ];
        let precision = FPDecimal::from_str("0.01").unwrap();

        for item in &data {
            let input = FPDecimal::from_str(item[0]).unwrap();
            let expected = FPDecimal::from_str(item[1]).unwrap();

            let output = round_to_nearest_tick(input, precision);
            assert_eq!(expected, output);
        }
    }

    #[test]
    fn test_round_up_to_min_tick() {
        let num = FPDecimal::from(37u128);
        let min_tick = FPDecimal::from(10u128);

        let result = round_up_to_min_tick(num, min_tick);
        assert_eq!(result, FPDecimal::from(40u128));

        let num = FPDecimal::from_str("0.00000153").unwrap();
        let min_tick = FPDecimal::from_str("0.000001").unwrap();

        let result = round_up_to_min_tick(num, min_tick);
        assert_eq!(result, FPDecimal::from_str("0.000002").unwrap());

        let num = FPDecimal::from_str("0.000001").unwrap();
        let min_tick = FPDecimal::from_str("0.000001").unwrap();

        let result = round_up_to_min_tick(num, min_tick);
        assert_eq!(result, FPDecimal::from_str("0.000001").unwrap());

        let num = FPDecimal::from_str("0.0000001").unwrap();
        let min_tick = FPDecimal::from_str("0.000001").unwrap();

        let result = round_up_to_min_tick(num, min_tick);
        assert_eq!(result, FPDecimal::from_str("0.000001").unwrap());
    }
}
