/// Arithmetic operators for FPDecimal
use crate::fp_decimal::{FPDecimal, U256};
use core::convert::TryFrom;
use primitive_types::U512;
use std::iter;
use std::ops;

impl FPDecimal {
    pub(crate) fn _add(x: FPDecimal, y: FPDecimal) -> FPDecimal {
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
        if y.num == x.num {
            return FPDecimal::ZERO;
        }

        FPDecimal {
            num: y.num - x.num,
            sign: y.sign,
        }
    }

    pub fn add(&self, other: i128) -> FPDecimal {
        FPDecimal::_add(*self, FPDecimal::from(other))
    }

    pub(crate) fn _sub(x: FPDecimal, y: FPDecimal) -> FPDecimal {
        let neg_y = FPDecimal {
            num: y.num,
            sign: 1 - y.sign,
        };
        FPDecimal::_add(x, neg_y)
    }

    pub fn sub(&self, other: i128) -> FPDecimal {
        FPDecimal::_sub(*self, FPDecimal::from(other))
    }

    pub(crate) fn _mul(x: FPDecimal, y: FPDecimal) -> FPDecimal {
        let mut sign = 1;
        if x.sign != y.sign {
            sign = 0;
        }
        let x1 = FPDecimal::_int(x).num / FPDecimal::ONE.num;
        let x2 = FPDecimal::_fraction(x).num;
        let y1 = FPDecimal::_int(y).num / FPDecimal::ONE.num;
        let y2 = FPDecimal::_fraction(y).num;
        let mut x1y1 = x1 * y1;
        let dec_x1y1 = x1y1 * FPDecimal::ONE.num;
        x1y1 = dec_x1y1;
        let x2y1 = x2 * y1;
        let x1y2 = x1 * y2;

        let x2y2 = x2 * y2;
        let mut result = x1y1;
        result += x2y1;
        result += x1y2;
        result += x2y2 / FPDecimal::MUL_PRECISION.num / FPDecimal::MUL_PRECISION.num;

        FPDecimal { num: result, sign }
    }

    pub fn mul(&self, other: i128) -> FPDecimal {
        FPDecimal::_mul(*self, FPDecimal::from(other))
    }

    pub(crate) fn _div(x: FPDecimal, y: FPDecimal) -> FPDecimal {
        if y == FPDecimal::ONE {
            return x;
        }

        assert_ne!(y.num, U256::zero());

        let num = FPDecimal::ONE.num.full_mul(x.num) / U512::from(y.num);
        if num.is_zero() {
            return FPDecimal::ZERO;
        }

        FPDecimal {
            num: U256::try_from(num).unwrap(), // panic only in MIN_FPDecimal/-1
            sign: 1 ^ x.sign ^ y.sign,
        }
    }

    pub fn div(&self, other: i128) -> Self {
        FPDecimal::_div(*self, FPDecimal::from(other))
    }

    pub fn reciprocal(x: FPDecimal) -> Self {
        assert!(x.num != U256::zero());
        FPDecimal {
            num: FPDecimal::ONE.num * FPDecimal::ONE.num / x.num,
            sign: x.sign,
        }
    }

    pub fn abs(&self) -> Self {
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

    use crate::fp_decimal::U256;
    use crate::FPDecimal;

    #[test]
    fn compare() {
        let neg_ten = -FPDecimal::TEN;
        let neg_point_one = -FPDecimal::must_from_str("0.1");
        let neg_point_two = -FPDecimal::must_from_str("0.2");
        let neg_one = -FPDecimal::ONE;
        let point_one = FPDecimal::must_from_str("0.1");
        let one = FPDecimal::ONE;
        let ten = FPDecimal::TEN;
        assert!(neg_ten < neg_one);
        assert!(neg_one < neg_point_two);
        assert!(neg_point_two < neg_point_one);
        assert!(neg_point_one < point_one);
        assert!(point_one < one);
        assert!(one < ten);
    }

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

    #[test]
    #[should_panic(expected = "overflow")]
    fn test_overflow() {
        let num1 = FPDecimal::MAX + FPDecimal::ONE;
        assert_eq!(num1, FPDecimal::ONE);
    }

    #[test]
    fn test_add() {
        let five = FPDecimal::FIVE;
        let three = FPDecimal::THREE;
        let eight = FPDecimal::EIGHT;
        assert_eq!(five + three, eight);
    }

    #[test]
    fn test_add_neg() {
        let neg_five = -FPDecimal::FIVE;
        let neg_three = -FPDecimal::THREE;
        let neg_eight = -FPDecimal::EIGHT;
        assert_eq!(neg_five + neg_three, neg_eight);
    }

    #[test]
    fn test_sub() {
        let five = FPDecimal::FIVE;
        let three = FPDecimal::THREE;
        let two = FPDecimal::TWO;
        assert_eq!(five - three, two);
    }

    #[test]
    fn test_sub_neg() {
        let five = FPDecimal::FIVE;
        let neg_three = -FPDecimal::THREE;
        let eight = FPDecimal::EIGHT;
        assert_eq!(FPDecimal::_sub(five, neg_three), eight);
    }

    #[test]
    fn test_mul() {
        let five = FPDecimal::FIVE;
        let three = FPDecimal::THREE;
        let fifteen = FPDecimal::must_from_str("15");
        assert_eq!(five * three, fifteen);
    }

    #[test]
    fn test_mul_precisions() {
        // 8.33157469 * 0.000000000001 = 0.00000000000833157469
        assert_eq!(
            FPDecimal::must_from_str("8.33157469") * FPDecimal::must_from_str("0.000000000001"),
            FPDecimal::must_from_str("0.000000000008331574")
        );

        // 1.5 * 1.5 = 2.25
        assert_eq!(
            FPDecimal::must_from_str("1.5") * FPDecimal::must_from_str("1.5"),
            FPDecimal::must_from_str("2.25")
        );

        // 2.718281828459045235 * 2.718281828459045235 = 7.389056098930650225
        assert_eq!(FPDecimal::E * FPDecimal::E, FPDecimal::must_from_str("7.389056098930650225"));

        // 0.5 * 0.5 = 0.25
        assert_eq!(
            FPDecimal::must_from_str("0.5") * FPDecimal::must_from_str("0.5"),
            FPDecimal::must_from_str("0.25")
        );

        // 5 * 0.5 = 2.5
        assert_eq!(FPDecimal::FIVE * FPDecimal::must_from_str("0.5"), FPDecimal::must_from_str("2.5"));

        // 0.5 * 5 = 2.5
        assert_eq!(FPDecimal::must_from_str("0.5") * FPDecimal::FIVE, FPDecimal::must_from_str("2.5"));

        // 4 * 2.5 = 10
        assert_eq!(FPDecimal::FOUR * FPDecimal::must_from_str("2.5"), FPDecimal::must_from_str("10"));

        // 2.5 * 4 = 10
        assert_eq!(FPDecimal::must_from_str("2.5") * FPDecimal::FOUR, FPDecimal::must_from_str("10"));

        // 0.000000008 * 0.9 = 0.0000000072
        assert_eq!(
            FPDecimal::must_from_str("0.000000008") * FPDecimal::must_from_str("0.9"),
            FPDecimal::must_from_str("0.0000000072")
        );

        // 0.0000000008 * 0.9 = 0.00000000072
        assert_eq!(
            FPDecimal::must_from_str("0.0000000008") * FPDecimal::must_from_str("0.9"),
            FPDecimal::must_from_str("0.00000000072")
        );

        // -0.5 * 0.5 = -0.25
        assert_eq!(
            FPDecimal::must_from_str("-0.5") * FPDecimal::must_from_str("0.5"),
            FPDecimal::must_from_str("-0.25")
        );

        // -0.5 * -0.5 = 0.25
        assert_eq!(
            FPDecimal::must_from_str("-0.5") * FPDecimal::must_from_str("-0.5"),
            FPDecimal::must_from_str("0.25")
        );

        // -5 * -3 = 15
        assert_eq!(
            FPDecimal::must_from_str("-5") * FPDecimal::must_from_str("-3"),
            FPDecimal::must_from_str("15")
        );
    }

    #[test]
    fn test_mul_pos_neg() {
        let five = FPDecimal::FIVE;
        let neg_three = -FPDecimal::THREE;
        let neg_fifteen = FPDecimal::must_from_str("-15");
        assert_eq!(five * neg_three, neg_fifteen);
    }

    #[test]
    fn test_mul_neg_pos() {
        let neg_five = -FPDecimal::FIVE;
        let three = FPDecimal::THREE;
        let neg_fifteen = FPDecimal::must_from_str("-15");
        assert_eq!(neg_five * three, neg_fifteen);
    }

    #[test]
    fn test_mul_neg_neg() {
        let neg_five = -FPDecimal::FIVE;
        let neg_three = -FPDecimal::THREE;
        let fifteen = FPDecimal::must_from_str("15");
        assert_eq!(neg_five * neg_three, fifteen);
    }

    #[test]
    fn test_div() {
        let hundred = FPDecimal::must_from_str("100");
        let five = FPDecimal::FIVE;
        let twenty = FPDecimal::must_from_str("20");
        assert_eq!(hundred / five, twenty);
    }

    #[test]
    fn test_reciprocal() {
        let five = FPDecimal::FIVE;
        let point_2 = FPDecimal::TWO / FPDecimal::TEN;
        assert_eq!(FPDecimal::reciprocal(five), point_2);
        assert_eq!(FPDecimal::reciprocal(point_2), five);
        assert_eq!(FPDecimal::reciprocal(FPDecimal::must_from_str("0.5")), FPDecimal::TWO);
    }

    #[test]
    fn test_abs() {
        let neg_five = -FPDecimal::FIVE;
        let five = FPDecimal::FIVE;
        assert_eq!(neg_five.abs(), five);
    }

    #[test]
    fn test_div_identity() {
        for i in 1..10000 {
            let a = FPDecimal::must_from_str(&format!("{}", i));
            assert_eq!(a / a, FPDecimal::ONE);
        }
    }

    #[test]
    fn test_add_assign() {
        let mut ans = FPDecimal::FIVE;
        let four = FPDecimal::FOUR;
        let nine = FPDecimal::NINE;
        ans += four;
        assert_eq!(ans, nine);

        let mut ans2 = FPDecimal::NINE;
        let five = FPDecimal::FIVE;
        let neg_four = -FPDecimal::FOUR;
        ans2 += neg_four;
        assert_eq!(five, ans2);
    }

    #[test]
    fn test_sub_assign() {
        let mut ans = FPDecimal::FIVE;
        let four = FPDecimal::FOUR;
        ans -= four;
        assert_eq!(ans, FPDecimal::ONE);

        let mut ans = FPDecimal::ONE;
        let five = FPDecimal::FIVE;
        let neg_four = -FPDecimal::FOUR;
        ans -= neg_four;
        assert_eq!(five, ans);
    }

    #[test]
    fn test_mul_assign() {
        let mut ans = FPDecimal::FIVE;
        let two = FPDecimal::TWO;
        let ten = FPDecimal::TEN;
        ans *= two;
        assert_eq!(ten, ans);

        let mut ans = -FPDecimal::FIVE;
        let two = FPDecimal::TWO;
        let neg_ten = -FPDecimal::TEN;
        ans *= two;
        assert_eq!(neg_ten, ans);
    }

    #[test]
    fn test_div_assign() {
        let mut ans = FPDecimal::EIGHT;
        ans /= FPDecimal::TWO;
        assert_eq!(FPDecimal::FOUR, ans);

        let mut y = FPDecimal::FIVE;
        y /= FPDecimal::TWO;
        assert_eq!(FPDecimal::must_from_str("2.5"), y);

        let mut z = FPDecimal::ONE;
        z /= FPDecimal::THREE;
        assert_eq!(z, FPDecimal::THREE / FPDecimal::NINE);
    }

    #[test]
    fn test_is_negative() {
        let val = FPDecimal::TWO;
        assert!(!val.is_negative());

        let val = FPDecimal::ZERO;
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
        let lhs = FPDecimal::TWO;
        let rhs = FPDecimal::THREE;
        let ans = lhs.abs_diff(&rhs);
        assert_eq!(FPDecimal::ONE, ans);

        let lhs = FPDecimal::THREE;
        let rhs = FPDecimal::ONE;
        let ans = lhs.abs_diff(&rhs);
        assert_eq!(FPDecimal::TWO, ans);

        let lhs = FPDecimal::NEGATIVE_ONE;
        let rhs = FPDecimal::TWO;
        let ans = lhs.abs_diff(&rhs);
        assert_eq!(FPDecimal::THREE, ans);
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
        let vector = [FPDecimal::ZERO, FPDecimal::ONE, FPDecimal::TWO, FPDecimal::THREE];
        assert_eq!(FPDecimal::SIX, vector.iter().sum());
    }
    #[test]
    fn test_chain_sum_equal_zero() {
        let vector = [FPDecimal::ZERO, FPDecimal::ONE, FPDecimal::TWO, -FPDecimal::THREE];
        assert_eq!(FPDecimal::ZERO, vector.iter().sum());
    }
}
