use cosmwasm_std::{OverflowError, OverflowOperation};
use std::str::FromStr;

/// Exponential functions for FPDecimal
use crate::fp_decimal::{FPDecimal, U256};
use num::pow::Pow;

impl FPDecimal {
    // a^b
    pub fn _pow(a: FPDecimal, b: FPDecimal) -> FPDecimal {
        a.checked_pow(b).unwrap()
    }
    // e^(a)
    pub fn _exp(a: FPDecimal) -> FPDecimal {
        // this throws underflow with a sufficiently large negative exponent
        // short circuit and just return 0 above a certain threshold
        // otherwise if there is a long enough delay between updates on a cluster
        // the penalty function will be bricked
        if a.sign == 0 && a.num >= FPDecimal::from(45i128).num {
            return FPDecimal::zero();
        }
        let mut x = a.num;
        let mut r = FPDecimal::ONE;
        while x >= U256([10, 0, 0, 0]) * FPDecimal::ONE.num {
            x = x - U256([10, 0, 0, 0]) * FPDecimal::ONE.num;
            r = FPDecimal::_mul(r, FPDecimal::E_10);
        }
        if x == FPDecimal::ONE.num {
            let val = FPDecimal::_mul(r, FPDecimal::E);
            if a.sign == 0 {
                return FPDecimal::reciprocal(val);
            }
            return val;
        } else if x == FPDecimal::zero().num {
            let val = r;
            if a.sign == 0 {
                return FPDecimal::reciprocal(val);
            }
            return val;
        }
        let mut tr = FPDecimal::ONE.num;
        let mut d = tr;
        for i in 1..((2 * FPDecimal::DIGITS + 1) as u64) {
            d = (d * x) / (FPDecimal::ONE.num * U256([i, 0, 0, 0]));
            tr = tr + d;
        }
        let val = FPDecimal::_mul(FPDecimal { num: tr, sign: 1 }, r);
        if a.sign == 0 {
            return FPDecimal::reciprocal(val);
        }
        val
    }

    // a^(0.5)
    pub fn sqrt(a: FPDecimal) -> Option<FPDecimal> {
        const MAX_ITERATIONS: i64 = 300;

        if a < FPDecimal::zero() {
            return None;
        }

        if a.is_zero() {
            return Some(FPDecimal::zero());
        }

        // Start with an arbitrary number as the first guess
        let mut r = a / FPDecimal::TWO;
        let mut l = r + FPDecimal::one();

        // Keep going while the difference is larger than the tolerance
        let mut c = 0i64;
        while (l != r) && (c < MAX_ITERATIONS) {
            l = r;
            r = (r + a / r) / FPDecimal::TWO;

            c += 1;
        }

        Some(r)
    }

    pub fn checked_pow(self, rhs: FPDecimal) -> Result<FPDecimal, OverflowError> {
        {
            // This uses the exponentiation by squaring algorithm:
            // https://en.wikipedia.org/wiki/Exponentiation_by_squaring#Basic_method

            if self == FPDecimal::zero() {
                return Ok(FPDecimal::zero());
            }
            if self > FPDecimal::zero() && rhs == FPDecimal::zero() {
                return Ok(FPDecimal::one());
            }
            if self.is_negative() && rhs == FPDecimal::zero() {
                return Ok(FPDecimal::NEGATIVE_ONE);
            }

            if self == FPDecimal::from(10u128) {
                if rhs == FPDecimal::one() {
                    return Ok(FPDecimal::from(10u128));
                }
                if rhs == FPDecimal::TWO {
                    return Ok(FPDecimal::from(100u128));
                }
                if rhs == FPDecimal::THREE {
                    return Ok(FPDecimal::from(1000u128));
                }
                if rhs == FPDecimal::FOUR {
                    return Ok(FPDecimal::from(10000u128));
                }
                if rhs == FPDecimal::FIVE {
                    return Ok(FPDecimal::from(100000u128));
                }
                if rhs == FPDecimal::SIX {
                    return Ok(FPDecimal::from(1000000u128));
                }
                if rhs == FPDecimal::SEVEN {
                    return Ok(FPDecimal::from(10000000u128));
                }
                if rhs == FPDecimal::EIGHT {
                    return Ok(FPDecimal::from(100000000u128));
                }
                if rhs == FPDecimal::NINE {
                    return Ok(FPDecimal::from(1000000000u128));
                }
                if rhs == FPDecimal::from(10u128) {
                    return Ok(FPDecimal::from(10000000000u128));
                }
                if rhs == FPDecimal::from(11u128) {
                    return Ok(FPDecimal::from(100000000000u128));
                }
                if rhs == FPDecimal::from(12u128) {
                    return Ok(FPDecimal::from(1000000000000u128));
                }
                if rhs == FPDecimal::from(13u128) {
                    return Ok(FPDecimal::from(10000000000000u128));
                }
                if rhs == FPDecimal::from(14u128) {
                    return Ok(FPDecimal::from(100000000000000u128));
                }
                if rhs == FPDecimal::from(15u128) {
                    return Ok(FPDecimal::from(1000000000000000u128));
                }
                if rhs == FPDecimal::from(16u128) {
                    return Ok(FPDecimal::from(10000000000000000u128));
                }
                if rhs == FPDecimal::from(17u128) {
                    return Ok(FPDecimal::from(100000000000000000u128));
                }
                if rhs == FPDecimal::from(18u128) {
                    return Ok(FPDecimal::from(1000000000000000000u128));
                }
                if rhs == FPDecimal::from(19u128) {
                    return Ok(FPDecimal::from(10000000000000000000u128));
                }
                if rhs == FPDecimal::from(20u128) {
                    return Ok(FPDecimal::from(100000000000000000000u128));
                }
                if rhs == FPDecimal::NEGATIVE_ONE {
                    return Ok(FPDecimal::from_str("0.1").unwrap());
                }
                if rhs == FPDecimal::from_str("-2").unwrap() {
                    return Ok(FPDecimal::from_str("0.01").unwrap());
                }
                if rhs == FPDecimal::from_str("-3").unwrap() {
                    return Ok(FPDecimal::from_str("0.001").unwrap());
                }
                if rhs == FPDecimal::from_str("-4").unwrap() {
                    return Ok(FPDecimal::from_str("0.0001").unwrap());
                }
                if rhs == FPDecimal::from_str("-5").unwrap() {
                    return Ok(FPDecimal::from_str("0.00001").unwrap());
                }
                if rhs == FPDecimal::from_str("-6").unwrap() {
                    return Ok(FPDecimal::from_str("0.000001").unwrap());
                }
                if rhs == FPDecimal::from_str("-7").unwrap() {
                    return Ok(FPDecimal::from_str("0.0000001").unwrap());
                }
                if rhs == FPDecimal::from_str("-8").unwrap() {
                    return Ok(FPDecimal::from_str("0.00000001").unwrap());
                }
                if rhs == FPDecimal::from_str("-9").unwrap() {
                    return Ok(FPDecimal::from_str("0.000000001").unwrap());
                }
                if rhs == FPDecimal::from_str("-10").unwrap() {
                    return Ok(FPDecimal::from_str("0.0000000001").unwrap());
                }
                if rhs == FPDecimal::from_str("-11").unwrap() {
                    return Ok(FPDecimal::from_str("0.00000000001").unwrap());
                }
                if rhs == FPDecimal::from_str("-12").unwrap() {
                    return Ok(FPDecimal::from_str("0.000000000001").unwrap());
                }
                if rhs == FPDecimal::from_str("-13").unwrap() {
                    return Ok(FPDecimal::from_str("0.0000000000001").unwrap());
                }
                if rhs == FPDecimal::from_str("-14").unwrap() {
                    return Ok(FPDecimal::from_str("0.00000000000001").unwrap());
                }
                if rhs == FPDecimal::from_str("-15").unwrap() {
                    return Ok(FPDecimal::from_str("0.000000000000001").unwrap());
                }
                if rhs == FPDecimal::from_str("-16").unwrap() {
                    return Ok(FPDecimal::from_str("0.0000000000000001").unwrap());
                }
                if rhs == FPDecimal::from_str("-17").unwrap() {
                    return Ok(FPDecimal::from_str("0.00000000000000001").unwrap());
                }
                if rhs == FPDecimal::from_str("-18").unwrap() {
                    return Ok(FPDecimal::from_str("0.000000000000000001").unwrap());
                }
                if rhs < FPDecimal::from_str("-18").unwrap() {
                    return Ok(FPDecimal::zero());
                }
                if rhs == FPDecimal::from(21u128) {
                    return Ok(FPDecimal::from(1000000000000000000000u128));
                }
                if rhs == FPDecimal::from(22u128) {
                    return Ok(FPDecimal::from(10000000000000000000000u128));
                }
                if rhs == FPDecimal::from(23u128) {
                    return Ok(FPDecimal::from(100000000000000000000000u128));
                }
                if rhs == FPDecimal::from(24u128) {
                    return Ok(FPDecimal::from(1000000000000000000000000u128));
                }
                if rhs == FPDecimal::from(25u128) {
                    return Ok(FPDecimal::from(10000000000000000000000000u128));
                }
                if rhs == FPDecimal::from(26u128) {
                    return Ok(FPDecimal::from(100000000000000000000000000u128));
                }
                if rhs == FPDecimal::from(27u128) {
                    return Ok(FPDecimal::from(1000000000000000000000000000u128));
                }
                if rhs == FPDecimal::from(28u128) {
                    return Ok(FPDecimal::from(10000000000000000000000000000u128));
                }
                if rhs == FPDecimal::from(29u128) {
                    return Ok(FPDecimal::from(100000000000000000000000000000u128));
                }
                if rhs == FPDecimal::from(30u128) {
                    return Ok(FPDecimal::from(1000000000000000000000000000000u128));
                }
                if rhs == FPDecimal::from(31u128) {
                    return Ok(FPDecimal::from(10000000000000000000000000000000u128));
                }
                if rhs == FPDecimal::from(32u128) {
                    return Ok(FPDecimal::from(100000000000000000000000000000000u128));
                }
                if rhs == FPDecimal::from(33u128) {
                    return Ok(FPDecimal::from(1000000000000000000000000000000000u128));
                }
                if rhs == FPDecimal::from(34u128) {
                    return Ok(FPDecimal::from(10000000000000000000000000000000000u128));
                }
                if rhs == FPDecimal::from(35u128) {
                    return Ok(FPDecimal::from(100000000000000000000000000000000000u128));
                }
                if rhs == FPDecimal::from(36u128) {
                    return Ok(FPDecimal::from(1000000000000000000000000000000000000u128));
                }
                if rhs == FPDecimal::from(37u128) {
                    return Ok(FPDecimal::from(10000000000000000000000000000000000000u128));
                }
                if rhs == FPDecimal::from(38u128) {
                    return Ok(FPDecimal::from(100000000000000000000000000000000000000u128));
                }
                if rhs == FPDecimal::from(39u128) {
                    return Ok(FPDecimal::from_str("1000000000000000000000000000000000000000").unwrap());
                }
                if rhs == FPDecimal::from(40u128) {
                    return Ok(FPDecimal::from_str("10000000000000000000000000000000000000000").unwrap());
                }
                if rhs == FPDecimal::from(41u128) {
                    return Ok(FPDecimal::from_str("100000000000000000000000000000000000000000").unwrap());
                }
                if rhs == FPDecimal::from(42u128) {
                    return Ok(FPDecimal::from_str("1000000000000000000000000000000000000000000").unwrap());
                }
                if rhs == FPDecimal::from(43u128) {
                    return Ok(FPDecimal::from_str("10000000000000000000000000000000000000000000").unwrap());
                }
                if rhs == FPDecimal::from(44u128) {
                    return Ok(FPDecimal::from_str("100000000000000000000000000000000000000000000").unwrap());
                }
                if rhs == FPDecimal::from(45u128) {
                    return Ok(FPDecimal::from_str("1000000000000000000000000000000000000000000000").unwrap());
                }
                if rhs == FPDecimal::from(46u128) {
                    return Ok(FPDecimal::from_str("10000000000000000000000000000000000000000000000").unwrap());
                }
                if rhs == FPDecimal::from(47u128) {
                    return Ok(FPDecimal::from_str("100000000000000000000000000000000000000000000000").unwrap());
                }
                if rhs == FPDecimal::from(48u128) {
                    return Ok(FPDecimal::from_str("1000000000000000000000000000000000000000000000000").unwrap());
                }
                if rhs == FPDecimal::from(49u128) {
                    return Ok(FPDecimal::from_str("10000000000000000000000000000000000000000000000000").unwrap());
                }
                if rhs == FPDecimal::from(50u128) {
                    return Ok(FPDecimal::from_str("100000000000000000000000000000000000000000000000000").unwrap());
                }
                if rhs == FPDecimal::from(51u128) {
                    return Ok(FPDecimal::from_str("1000000000000000000000000000000000000000000000000000").unwrap());
                }
                if rhs == FPDecimal::from(52u128) {
                    return Ok(FPDecimal::from_str("10000000000000000000000000000000000000000000000000000").unwrap());
                }
                if rhs == FPDecimal::from(53u128) {
                    return Ok(FPDecimal::from_str("100000000000000000000000000000000000000000000000000000").unwrap());
                }
                if rhs == FPDecimal::from(54u128) {
                    return Ok(FPDecimal::from_str("1000000000000000000000000000000000000000000000000000000").unwrap());
                }
                if rhs == FPDecimal::from(55u128) {
                    return Ok(FPDecimal::from_str("10000000000000000000000000000000000000000000000000000000").unwrap());
                }
                if rhs == FPDecimal::from(56u128) {
                    return Ok(FPDecimal::from_str("100000000000000000000000000000000000000000000000000000000").unwrap());
                }
                if rhs == FPDecimal::from(57u128) {
                    return Ok(FPDecimal::from_str("1000000000000000000000000000000000000000000000000000000000").unwrap());
                }
                if rhs == FPDecimal::from(58u128) {
                    return Ok(FPDecimal::from_str("10000000000000000000000000000000000000000000000000000000000").unwrap());
                }
                if rhs == FPDecimal::from(59u128) {
                    return Ok(FPDecimal::from_str("100000000000000000000000000000000000000000000000000000000000").unwrap());
                }
            }

            fn inner(mut x: FPDecimal, mut n: FPDecimal) -> Result<FPDecimal, OverflowError> {
                if n == FPDecimal::zero() {
                    return Ok(FPDecimal::one());
                }

                let mut y = FPDecimal::one();

                while n > FPDecimal::one() {
                    if n.num % FPDecimal::TWO.num == FPDecimal::zero().num {
                        x = x * x;
                        n = n / FPDecimal::TWO;
                    } else {
                        y = x * y;
                        x = x * x;
                        n = (n - FPDecimal::ONE) / FPDecimal::TWO;
                    }
                }

                Ok(x * y)
            }

            inner(self, rhs).map_err(|_| OverflowError {
                operation: OverflowOperation::Pow,
                operand1: self.to_string(),
                operand2: rhs.to_string(),
            })
        }
    }
}

impl Pow<FPDecimal> for FPDecimal {
    type Output = Self;
    // fn pow(self, rhs: FPDecimal) -> Self::Output {
    //     Self::_pow(self, rhs)
    // }

    /// Raises a value to the power of `exp`, panics if an overflow occurred.
    fn pow(self, exp: FPDecimal) -> Self {
        match self.checked_pow(exp) {
            Ok(value) => value,
            Err(_) => panic!("Multiplication overflow"),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::FPDecimal;
    use bigint::U256;
    use num::pow::Pow;
    use std::str::FromStr;

    #[test]
    fn test_exp() {
        assert_eq!(FPDecimal::_exp(FPDecimal::ONE), FPDecimal::E);
    }

    #[test]
    fn test_exp0() {
        assert_eq!(FPDecimal::_exp(FPDecimal::zero()), FPDecimal::ONE);
    }

    #[test]
    fn test_exp10() {
        assert_eq!(
            FPDecimal::_exp(FPDecimal {
                num: U256([10, 0, 0, 0]) * FPDecimal::ONE.num,
                sign: 1
            }),
            FPDecimal::E_10
        );
    }

    #[test]
    fn test_pow_zero() {
        // FPDecimal::_ln(FPDecimal::zero());
        FPDecimal::pow(FPDecimal::zero(), FPDecimal::one().div(2i128));
        assert_eq!(FPDecimal::ZERO.pow(FPDecimal::ONE), FPDecimal::ZERO);
    }

    #[test]
    fn test_pow_four() {
        // NOTE: this test is not correct, but is an example of why we need a square root
        assert!(FPDecimal::pow(FPDecimal::FOUR, FPDecimal::one().div(2i128)) != FPDecimal::TWO);
    }

    #[test]
    fn test_pow_exp() {
        assert_eq!(FPDecimal::E.pow(FPDecimal::ONE), FPDecimal::E);
    }

    #[test]
    fn test_pow_exp0() {
        assert_eq!(FPDecimal::E.pow(FPDecimal::zero()), FPDecimal::ONE);
    }

    #[test]
    fn test_pow_exp10() {
        assert_eq!(
            FPDecimal::E.pow(FPDecimal {
                num: U256([10, 0, 0, 0]) * FPDecimal::ONE.num,
                sign: 1
            }),
            FPDecimal::E_10
        );
    }

    #[test]
    fn test_pow_zero_2() {
        FPDecimal::ZERO.pow(FPDecimal::one().div(2i128));
    }

    #[test]
    fn test_square_root() {
        let inputs: Vec<i128> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 16, 25, -1];

        let expected: Vec<Option<FPDecimal>> = vec![
            Some(FPDecimal::zero()),
            Some(FPDecimal::one()),
            Some(FPDecimal::from_str("1.414213562373095048").unwrap()),
            Some(FPDecimal::from_str("1.732050807568877293").unwrap()),
            Some(FPDecimal::TWO),
            Some(FPDecimal::from_str("2.236067977499789696").unwrap()),
            Some(FPDecimal::from_str("2.449489742783178098").unwrap()),
            Some(FPDecimal::from_str("2.645751311064590590").unwrap()),
            Some(FPDecimal::from_str("2.828427124746190097").unwrap()),
            Some(FPDecimal::THREE),
            Some(FPDecimal::from_str("3.162277660168379331").unwrap()),
            Some(FPDecimal::FOUR),
            Some(FPDecimal::FIVE),
            None,
        ];

        for (ix, el) in inputs.iter().enumerate() {
            let result = FPDecimal::sqrt(FPDecimal::from(*el));

            assert_eq!(result, expected[ix]);
        }
    }

    #[test]
    fn test_pow_10_positive() {
        let base = FPDecimal::from(10u128);
        assert_eq!(base.pow(FPDecimal::from_str("6").unwrap()), FPDecimal::from_str("1000000").unwrap());
    }

    #[test]
    fn test_pow_10_max() {
        let base = FPDecimal::from(10u128);
        assert_eq!(
            base.pow(FPDecimal::from_str("59").unwrap()),
            FPDecimal::from_str("100000000000000000000000000000000000000000000000000000000000").unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn test_pow_10_overflow() {
        let base = FPDecimal::from(10u128);
        base.pow(FPDecimal::from_str("60").unwrap());
    }

    #[test]
    fn test_pow_10_negative() {
        let base = FPDecimal::from(10u128);
        assert_eq!(base.pow(FPDecimal::from_str("-3").unwrap()), FPDecimal::from_str("0.001").unwrap());
    }

    #[test]
    fn test_pow_10_min() {
        let base = FPDecimal::from(10u128);
        assert_eq!(
            base.pow(FPDecimal::from_str("-18").unwrap()),
            FPDecimal::from_str("0.000000000000000001").unwrap()
        );
    }

    #[test]
    fn test_pow_10_underflow() {
        let base = FPDecimal::from(10u128);
        assert_eq!(base.pow(FPDecimal::from_str("-19").unwrap()), FPDecimal::zero());
    }

    #[test]
    fn test_checked_pow() {
        let base = FPDecimal::from(2u128);

        let result = FPDecimal::checked_pow(base, FPDecimal::from(2u128)).unwrap();
        assert_eq!(result, FPDecimal::from(4u128));
    }
}
