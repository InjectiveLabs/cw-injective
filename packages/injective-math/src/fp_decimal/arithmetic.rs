/// Arithmetic operators for FPDecimal
use crate::fp_decimal::{FPDecimal, U256};
use std::iter;
use std::ops;

impl FPDecimal {
    pub fn _add(x: FPDecimal, y: FPDecimal) -> FPDecimal {
        if x.sign == y.sign {
            return FPDecimal {
                num: x.num + y.num,
                sign: x.sign,
            };
        }

        if x.num > y.num {
            return FPDecimal {
                num: x.num - y.num,
                sign: x.sign,
            };
        }
        let mut sign = y.sign;
        if y.num == x.num {
            sign = 1;
        }

        FPDecimal { num: y.num - x.num, sign }
    }

    pub fn add(&self, other: i128) -> FPDecimal {
        FPDecimal::_add(*self, FPDecimal::from(other))
    }

    pub fn _sub(x: FPDecimal, y: FPDecimal) -> FPDecimal {
        let neg_y = FPDecimal {
            num: y.num,
            sign: 1 - y.sign,
        };
        FPDecimal::_add(x, neg_y)
    }

    pub fn sub(&self, other: i128) -> FPDecimal {
        FPDecimal::_sub(*self, FPDecimal::from(other))
    }

    pub fn _mul(x: FPDecimal, y: FPDecimal) -> FPDecimal {
        let mut sign = 1;
        if x.sign != y.sign {
            sign = 0;
        }
        let x1: U256 = FPDecimal::_int(x).num / FPDecimal::ONE.num;
        let x2: U256 = FPDecimal::_fraction(x).num;
        let y1: U256 = FPDecimal::_int(y).num / FPDecimal::ONE.num;
        let y2: U256 = FPDecimal::_fraction(y).num;
        let mut x1y1 = x1 * y1;
        let dec_x1y1 = x1y1 * FPDecimal::ONE.num;
        x1y1 = dec_x1y1;
        let x2y1 = x2 * y1;
        let x1y2 = x1 * y2;

        let x2y2 = x2 * y2;
        let mut result = x1y1;
        result = result + x2y1;
        result = result + x1y2;
        result = result + x2y2 / FPDecimal::MUL_PRECISION.num / FPDecimal::MUL_PRECISION.num;

        FPDecimal { num: result, sign }
    }

    pub fn mul(&self, other: i128) -> FPDecimal {
        FPDecimal::_mul(*self, FPDecimal::from(other))
    }

    pub fn _div(x: FPDecimal, y: FPDecimal) -> FPDecimal {
        if y == FPDecimal::ONE {
            return x;
        }

        assert_ne!(y.num, U256::zero());

        let num = FPDecimal::ONE.num.full_mul(x.num) / y.num.into();
        if num.is_zero() {
            return FPDecimal::zero();
        }

        FPDecimal {
            num: num.into(),
            sign: 1 ^ x.sign ^ y.sign,
        }
    }

    pub fn div(&self, other: i128) -> FPDecimal {
        FPDecimal::_div(*self, FPDecimal::from(other))
    }

    pub fn reciprocal(x: FPDecimal) -> FPDecimal {
        assert!(x.num != U256::zero());
        FPDecimal {
            num: FPDecimal::ONE.num * FPDecimal::ONE.num / x.num,
            sign: x.sign,
        }
    }

    pub fn abs(&self) -> FPDecimal {
        FPDecimal { num: self.num, sign: 1i8 }
    }

    pub fn abs_diff(&self, other: &Self) -> Self {
        if self > other {
            *self - *other
        } else {
            *other - *self
        }
    }
}

impl ops::Add for FPDecimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        FPDecimal::_add(self, rhs)
    }
}

impl ops::AddAssign for FPDecimal {
    fn add_assign(&mut self, rhs: Self) {
        *self = FPDecimal::_add(*self, rhs);
    }
}

impl ops::Sub for FPDecimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        FPDecimal::_sub(self, rhs)
    }
}

impl ops::SubAssign for FPDecimal {
    fn sub_assign(&mut self, rhs: Self) {
        *self = FPDecimal::_sub(*self, rhs);
    }
}

impl ops::Mul for FPDecimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        FPDecimal::_mul(self, rhs)
    }
}

impl ops::MulAssign for FPDecimal {
    fn mul_assign(&mut self, rhs: Self) {
        *self = FPDecimal::_mul(*self, rhs);
    }
}

impl ops::Div for FPDecimal {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        FPDecimal::_div(self, rhs)
    }
}

impl ops::DivAssign for FPDecimal {
    fn div_assign(&mut self, rhs: FPDecimal) {
        *self = *self / rhs;
    }
}

impl ops::Rem for FPDecimal {
    type Output = Self;

    fn rem(self, divisor: FPDecimal) -> Self::Output {
        assert_ne!(divisor, FPDecimal::ZERO);

        if divisor.is_negative() {
            return self.calculate_negative_remainder(divisor);
        }

        self.calculate_positive_remainder(divisor)
    }
}

impl FPDecimal {
    fn calculate_positive_remainder(&self, divisor: FPDecimal) -> FPDecimal {
        let mut remainder = *self;

        if self.is_negative() {
            while remainder < FPDecimal::ZERO {
                remainder += divisor;
            }

            return remainder;
        }

        while remainder >= divisor {
            remainder -= divisor;
        }

        remainder
    }

    fn calculate_negative_remainder(&self, divisor: FPDecimal) -> FPDecimal {
        let mut remainder = *self;

        if self.is_negative() {
            while remainder < divisor {
                remainder -= divisor;
            }

            return remainder;
        }

        while remainder >= -divisor {
            remainder += divisor;
        }

        remainder
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl ops::RemAssign for FPDecimal {
    fn rem_assign(&mut self, b: FPDecimal) {
        *self = *self % b;
    }
}

impl<'a> iter::Sum<&'a Self> for FPDecimal {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(FPDecimal::ZERO, |a, b| a + *b)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::FPDecimal;
    use bigint::U256;

    #[test]
    fn test_into_u128() {
        let first_num: u128 = FPDecimal::from(1234567890123456789u128).into();
        assert_eq!(first_num, 1234567890123456789u128);

        let num: u128 = FPDecimal::from(u128::MAX).into();
        assert_eq!(num, u128::MAX);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn panic_into_u128() {
        let _: u128 = (FPDecimal::from(u128::MAX) + FPDecimal::ONE).into();
    }

    // #[test]
    // fn test_overflow() {
    //     let num1 = FPDecimal::from(1701411834604692317316873037158841i128);
    //     assert_eq!(num1, FPDecimal::one());
    // }

    #[test]
    fn test_add() {
        let five = FPDecimal {
            num: U256([5, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let three = FPDecimal {
            num: U256([3, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let eight = FPDecimal {
            num: U256([8, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        assert_eq!(FPDecimal::_add(five, three), eight);
    }

    #[test]
    fn test_add_neg() {
        let neg_five = FPDecimal {
            num: U256([5, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 0,
        };
        let neg_three = FPDecimal {
            num: U256([3, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 0,
        };
        let neg_eight = FPDecimal {
            num: U256([8, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 0,
        };
        assert_eq!(FPDecimal::_add(neg_five, neg_three), neg_eight);
    }

    #[test]
    fn test_sub() {
        let five = FPDecimal {
            num: U256([5, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let three = FPDecimal {
            num: U256([3, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let two = FPDecimal {
            num: U256([2, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        assert_eq!(FPDecimal::_sub(five, three), two);
    }

    #[test]
    fn test_sub_neg() {
        let five = FPDecimal {
            num: U256([5, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let neg_three = FPDecimal {
            num: U256([3, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 0,
        };
        let eight = FPDecimal {
            num: U256([8, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        assert_eq!(FPDecimal::_sub(five, neg_three), eight);
    }

    #[test]
    fn test_mul() {
        let five = FPDecimal {
            num: U256([5, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let three = FPDecimal {
            num: U256([3, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let fifteen = FPDecimal {
            num: U256([15, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        assert_eq!(FPDecimal::_mul(five, three), fifteen);
    }

    #[test]
    fn test_mul_precisions() {
        // 8.33157469 * 0.000000000001 = 0.00000000000833157469
        assert_eq!(
            FPDecimal::from_str("8.33157469").unwrap() * FPDecimal::from_str("0.000000000001").unwrap(),
            FPDecimal::from_str("0.000000000008331574").unwrap()
        );

        // 1.5 * 1.5 = 2.25
        assert_eq!(
            FPDecimal::from_str("1.5").unwrap() * FPDecimal::from_str("1.5").unwrap(),
            FPDecimal::from_str("2.25").unwrap()
        );

        // 2.718281828459045235 * 2.718281828459045235 = 7.389056098930650225
        assert_eq!(FPDecimal::E * FPDecimal::E, FPDecimal::from_str("7.389056098930650225").unwrap());

        // 0.5 * 0.5 = 0.25
        assert_eq!(
            FPDecimal::from_str("0.5").unwrap() * FPDecimal::from_str("0.5").unwrap(),
            FPDecimal::from_str("0.25").unwrap()
        );

        // 5 * 0.5 = 2.5
        assert_eq!(FPDecimal::FIVE * FPDecimal::from_str("0.5").unwrap(), FPDecimal::from_str("2.5").unwrap());

        // 0.5 * 5 = 2.5
        assert_eq!(FPDecimal::from_str("0.5").unwrap() * FPDecimal::FIVE, FPDecimal::from_str("2.5").unwrap());

        // 4 * 2.5 = 10
        assert_eq!(FPDecimal::FOUR * FPDecimal::from_str("2.5").unwrap(), FPDecimal::from_str("10").unwrap());

        // 2.5 * 4 = 10
        assert_eq!(FPDecimal::from_str("2.5").unwrap() * FPDecimal::FOUR, FPDecimal::from_str("10").unwrap());

        // 0.000000008 * 0.9 = 0.0000000072
        assert_eq!(
            FPDecimal::from_str("0.000000008").unwrap() * FPDecimal::from_str("0.9").unwrap(),
            FPDecimal::from_str("0.0000000072").unwrap()
        );

        // 0.0000000008 * 0.9 = 0.00000000072
        assert_eq!(
            FPDecimal::from_str("0.0000000008").unwrap() * FPDecimal::from_str("0.9").unwrap(),
            FPDecimal::from_str("0.00000000072").unwrap()
        );

        // -0.5 * 0.5 = -0.25
        assert_eq!(
            FPDecimal::from_str("-0.5").unwrap() * FPDecimal::from_str("0.5").unwrap(),
            FPDecimal::from_str("-0.25").unwrap()
        );

        // -0.5 * -0.5 = 0.25
        assert_eq!(
            FPDecimal::from_str("-0.5").unwrap() * FPDecimal::from_str("-0.5").unwrap(),
            FPDecimal::from_str("0.25").unwrap()
        );

        // -5 * -3 = 15
        assert_eq!(
            FPDecimal::from_str("-5").unwrap() * FPDecimal::from_str("-3").unwrap(),
            FPDecimal::from_str("15").unwrap()
        );
    }

    #[test]
    fn test_mul_pos_neg() {
        let five = FPDecimal {
            num: U256([5, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let neg_three = FPDecimal {
            num: U256([3, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 0,
        };
        let neg_fifteen = FPDecimal {
            num: U256([15, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 0,
        };
        assert_eq!(FPDecimal::_mul(five, neg_three), neg_fifteen);
    }

    #[test]
    fn test_mul_neg_pos() {
        let neg_five = FPDecimal {
            num: U256([5, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 0,
        };
        let three = FPDecimal {
            num: U256([3, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let neg_fifteen = FPDecimal {
            num: U256([15, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 0,
        };
        assert_eq!(FPDecimal::_mul(neg_five, three), neg_fifteen);
    }

    #[test]
    fn test_mul_neg_neg() {
        let neg_five = FPDecimal {
            num: U256([5, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 0,
        };
        let neg_three = FPDecimal {
            num: U256([3, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 0,
        };
        let fifteen = FPDecimal {
            num: U256([15, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        assert_eq!(FPDecimal::_mul(neg_five, neg_three), fifteen);
    }

    #[test]
    fn test_div() {
        let hundred = FPDecimal {
            num: U256([100, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let five = FPDecimal {
            num: U256([5, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let twenty = FPDecimal {
            num: U256([20, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        assert_eq!(FPDecimal::_div(hundred, five), twenty);
    }

    #[test]
    fn test_reciprocal() {
        let five = FPDecimal {
            num: U256([5, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let point_2 = FPDecimal {
            num: FPDecimal::ONE.num / U256([5, 0, 0, 0]),
            sign: 1,
        };
        assert_eq!(FPDecimal::reciprocal(five), point_2);
        assert_eq!(FPDecimal::reciprocal(point_2), five);
        assert_eq!(FPDecimal::reciprocal(FPDecimal::must_from_str("0.5")), FPDecimal::TWO);
    }

    #[test]
    fn test_abs() {
        let neg_five = FPDecimal {
            num: U256([5, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 0,
        };
        let five = FPDecimal {
            num: U256([5, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        assert_eq!(neg_five.abs(), five);
    }

    #[test]
    fn test_div_identity() {
        for i in 1..10000 {
            let a = FPDecimal {
                num: U256([i, 0, 0, 0]) * FPDecimal::ONE.num,
                sign: 1,
            };

            assert_eq!(a / a, FPDecimal::ONE);
        }
    }

    #[test]
    fn test_add_assign() {
        let mut five = FPDecimal {
            num: U256([5, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let four = FPDecimal {
            num: U256([4, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let nine = FPDecimal {
            num: U256([9, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        five += four;
        let nine_2 = five;
        assert_eq!(nine_2, nine);

        let mut nine = nine_2;
        let five = FPDecimal {
            num: U256([5, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let neg_four = FPDecimal {
            num: U256([4, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 0,
        };
        nine += neg_four;
        let five_2 = nine;
        assert_eq!(five, five_2);
    }

    #[test]
    fn test_sub_assign() {
        let mut five = FPDecimal {
            num: U256([5, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let four = FPDecimal {
            num: U256([4, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        five -= four;
        let one = five;
        assert_eq!(one, FPDecimal::one());

        let mut one = one;
        let five = FPDecimal {
            num: U256([5, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let neg_four = FPDecimal {
            num: U256([4, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 0,
        };
        one -= neg_four;
        let five_2 = one;
        assert_eq!(five, five_2);
    }

    #[test]
    fn test_mul_assign() {
        let mut five = FPDecimal {
            num: U256([5, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let two = FPDecimal {
            num: U256([2, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let ten = FPDecimal {
            num: U256([10, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        five *= two;
        let ten_2 = five;
        assert_eq!(ten, ten_2);

        let mut five = FPDecimal {
            num: U256([5, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let two = FPDecimal {
            num: U256([2, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 0,
        };
        let neg_ten = FPDecimal {
            num: U256([10, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 0,
        };
        five *= two;
        let neg_ten_2 = five;
        assert_eq!(neg_ten, neg_ten_2);
    }

    #[test]
    fn test_div_assign() {
        let mut x = FPDecimal::EIGHT;
        x /= FPDecimal::TWO;
        assert_eq!(FPDecimal::FOUR, x);

        let mut y = FPDecimal::FIVE;
        y /= FPDecimal::TWO;
        assert_eq!(FPDecimal::must_from_str("2.5"), y);
    }

    #[test]
    fn test_is_negative() {
        let val = FPDecimal::TWO;
        assert!(!val.is_negative());

        let val = FPDecimal::zero();
        assert!(!val.is_negative());

        // even a manually assigned negative zero value returns positive
        let val = FPDecimal {
            num: U256([0, 0, 0, 0]),
            sign: 1,
        };
        assert!(!val.is_negative());

        let val = FPDecimal::NEGATIVE_ONE;
        assert!(val.is_negative());
    }

    #[test]
    fn test_abs_diff() {
        let lhs = FPDecimal::from(2u128);
        let rhs = FPDecimal::from(3u128);
        let ans = lhs.abs_diff(&rhs);
        assert_eq!(FPDecimal::one(), ans);

        let lhs = FPDecimal::from(3u128);
        let rhs = FPDecimal::one();
        let ans = lhs.abs_diff(&rhs);
        assert_eq!(FPDecimal::from(2u128), ans);

        let lhs = FPDecimal::NEGATIVE_ONE;
        let rhs = FPDecimal::TWO;
        let ans = lhs.abs_diff(&rhs);
        assert_eq!(FPDecimal::from(3u128), ans);
    }

    #[test]
    fn test_remainder() {
        let x = FPDecimal::FIVE;
        let y = x % FPDecimal::TWO;
        assert_eq!(FPDecimal::ONE, y);

        let x = -FPDecimal::SEVEN;
        let y = x % FPDecimal::THREE;
        assert_eq!(FPDecimal::TWO, y);

        let x = -FPDecimal::SEVEN;
        let y = x % FPDecimal::SEVEN;
        assert_eq!(FPDecimal::ZERO, y);

        let x = FPDecimal::must_from_str("3.5");
        let y = x % FPDecimal::must_from_str("0.8");
        assert_eq!(FPDecimal::must_from_str("0.3"), y);

        let x = FPDecimal::must_from_str("-3.5");
        let y = x % FPDecimal::must_from_str("0.8");
        assert_eq!(FPDecimal::must_from_str("0.5"), y);

        let x = FPDecimal::must_from_str("-3.5");
        let y = x % FPDecimal::must_from_str("-0.8");
        assert_eq!(FPDecimal::must_from_str("-0.3"), y);
    }

    #[test]
    fn test_remainder_assign() {
        let mut x = FPDecimal::NINE;
        x %= FPDecimal::FIVE;
        assert_eq!(FPDecimal::FOUR, x);
    }

    #[test]
    fn test_chain_sum() {
        let vector = vec![FPDecimal::ZERO, FPDecimal::ONE, FPDecimal::TWO, FPDecimal::THREE];
        assert_eq!(FPDecimal::SIX, vector.iter().sum());
    }
    #[test]
    fn test_chain_sum_equal_zero() {
        let vector = vec![FPDecimal::ZERO, FPDecimal::ONE, FPDecimal::TWO, -FPDecimal::THREE];
        assert_eq!(FPDecimal::ZERO, vector.iter().sum());
    }
}
