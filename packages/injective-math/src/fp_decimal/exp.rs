/// Exponential functions for FPDecimal
use crate::fp_decimal::{FPDecimal, U256};
use num::pow::Pow;

impl FPDecimal {
    // a^b
    pub fn _pow(a: FPDecimal, b: FPDecimal) -> FPDecimal {
        if a == FPDecimal::zero() {
            return FPDecimal::zero();
        }
        FPDecimal::_exp(FPDecimal::_mul(FPDecimal::_ln(a), b))
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
    pub fn _root(a: FPDecimal) -> Option<FPDecimal> {
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
        while l != r {
            l = r;
            r = (r + a / r) / FPDecimal::TWO;
        }

        Some(r)
    }
}

impl Pow<FPDecimal> for FPDecimal {
    type Output = Self;
    fn pow(self, rhs: FPDecimal) -> Self::Output {
        Self::_pow(self, rhs)
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
        // FPDecimal::_ln(FPDecimal::zero());
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
            let result = FPDecimal::_root(FPDecimal::from(*el));

            assert_eq!(result, expected[ix]);
        }
    }
}
