use cosmwasm_std::{Decimal256 as Decimal, Fraction, Uint256};
use std::str::FromStr;

pub fn wrap(unwrapped_num: &String) -> Decimal {
    Decimal::from_str(unwrapped_num).unwrap()
}

pub fn div_int(num: Decimal, denom: Uint256) -> Decimal {
    if denom == Uint256::zero() {
        Decimal::zero()
    } else {
        num / denom
    }
}

pub fn div_dec(num: Decimal, denom: Decimal) -> Decimal {
    if denom == Decimal::zero() {
        denom
    } else {
        num * denom.inv().unwrap()
    }
}

pub fn sub_abs(lhs: Decimal, rhs: Decimal) -> Decimal {
    if lhs > rhs {
        lhs - rhs
    } else {
        rhs - lhs
    }
}

pub fn sub_no_overflow(lhs: Decimal, rhs: Decimal) -> Decimal {
    if lhs > rhs {
        lhs - rhs
    } else {
        Decimal::zero()
    }
}

pub fn round_to_precision(num: Decimal, precision_shift: Uint256) -> Decimal {
    let precision_shift = Decimal::from_str(&precision_shift.to_string()).unwrap();
    let shifted = (num * precision_shift) * Uint256::from_str("1").unwrap();
    let shifted = Decimal::from_str(&shifted.to_string()).unwrap();
    div_dec(shifted, precision_shift)
}

pub fn round_to_min_ticker(num: Decimal, min_ticker: Decimal) -> Decimal {
    let precision_shift = min_ticker.inv().unwrap();
    let shifted = (num * precision_shift) * Uint256::from_str("1").unwrap();
    let shifted = Decimal::from_str(&shifted.to_string()).unwrap();
    div_dec(shifted, precision_shift)
}

pub fn bp_to_dec(bp: Decimal) -> Decimal {
    div_dec(bp, Decimal::from_str("10000").unwrap())
}

#[cfg(test)]
mod tests {
    use super::sub_no_overflow;
    use crate::utils::{div_dec, div_int, round_to_precision, sub_abs, round_to_min_ticker};
    use cosmwasm_std::{Decimal256, Uint256};
    use std::str::FromStr;

    #[test]
    fn div_int_test() {
        let num = Decimal256::from_str("1").unwrap();
        let denom = Uint256::zero();
        let ans = div_int(num, denom);
        assert_eq!(Decimal256::zero(), ans);

        let num = Decimal256::from_str("3").unwrap();
        let denom = Uint256::from_str("1").unwrap();
        let ans = div_int(num, denom);
        assert_eq!(Decimal256::from_str("3").unwrap(), ans);

        let num = Decimal256::from_str("3").unwrap();
        let denom = Uint256::from_str("2").unwrap();
        let ans = div_int(num, denom);
        assert_eq!(Decimal256::from_str("1.5").unwrap(), ans);
    }

    #[test]
    fn div_dec_test() {
        let num = Decimal256::from_str("1").unwrap();
        let denom = Decimal256::zero();
        let ans = div_dec(num, denom);
        assert_eq!(Decimal256::zero(), ans);

        let num = Decimal256::from_str("3").unwrap();
        let denom = Decimal256::from_str("1").unwrap();
        let ans = div_dec(num, denom);
        assert_eq!(Decimal256::from_str("3").unwrap(), ans);

        let num = Decimal256::from_str("3").unwrap();
        let denom = Decimal256::from_str("2").unwrap();
        let ans = div_dec(num, denom);
        assert_eq!(Decimal256::from_str("1.5").unwrap(), ans);
    }

    #[test]
    fn sub_abs_test() {
        let lhs = Decimal256::from_str("2").unwrap();
        let rhs = Decimal256::from_str("3").unwrap();
        let ans = sub_abs(lhs, rhs);
        assert_eq!(Decimal256::one(), ans);

        let lhs = Decimal256::from_str("3").unwrap();
        let rhs = Decimal256::from_str("1").unwrap();
        let ans = sub_abs(lhs, rhs);
        assert_eq!(Decimal256::from_str("2").unwrap(), ans);
    }

    #[test]
    fn sub_no_overflow_test() {
        let lhs = Decimal256::from_str("2").unwrap();
        let rhs = Decimal256::from_str("3").unwrap();
        let ans = sub_no_overflow(lhs, rhs);
        assert_eq!(Decimal256::zero(), ans);

        let lhs = Decimal256::from_str("3").unwrap();
        let rhs = Decimal256::from_str("1").unwrap();
        let ans = sub_no_overflow(lhs, rhs);
        assert_eq!(Decimal256::from_str("2").unwrap(), ans);
    }

    #[test]
    fn round_to_precision_test() {
        let num = Decimal256::from_str("1.1111111111111").unwrap();
        let precision_shift = Uint256::from_str("10000").unwrap();
        let rounded_num = round_to_precision(num, precision_shift);
        assert_eq!(Decimal256::from_str("1.1111").unwrap(), rounded_num);
    }

    #[test] 
    fn round_to_min_ticker_test() {
        let num = Decimal256::from_str("1.1911111111111").unwrap();
        let precision_shift = Decimal256::from_str("0.1").unwrap();
        let rounded_num = round_to_min_ticker(num, precision_shift);
        println!("{}",rounded_num.to_string());
        assert_eq!(Decimal256::from_str("1.1").unwrap(), rounded_num);
    }
}
