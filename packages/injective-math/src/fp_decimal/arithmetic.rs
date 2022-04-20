/// Arithmetic operators for FPDecimal
use crate::fp_decimal::{FPDecimal, U256};
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

        FPDecimal {
            num: y.num - x.num,
            sign,
        }
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
        let mut x2: U256 = FPDecimal::_fraction(x).num;
        let y1: U256 = FPDecimal::_int(y).num / FPDecimal::ONE.num;
        let mut y2: U256 = FPDecimal::_fraction(y).num;
        let mut x1y1 = x1 * y1;
        let dec_x1y1 = x1y1 * FPDecimal::ONE.num;
        x1y1 = dec_x1y1;
        let x2y1 = x2 * y1;
        let x1y2 = x1 * y2;
        x2 = x2 / FPDecimal::MUL_PRECISION.num;
        y2 = y2 / FPDecimal::MUL_PRECISION.num;
        let x2y2 = x2 * y2;
        let mut result = x1y1;
        result = result + x2y1;
        result = result + x1y2;
        result = result + x2y2;
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
        FPDecimal {
            num: FPDecimal::ONE.num * x.num / y.num,
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
        FPDecimal {
            num: self.num,
            sign: 1i8,
        }
    }
}

impl ops::Add for FPDecimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        FPDecimal::_add(self, rhs)
    }
}

impl ops::Sub for FPDecimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        FPDecimal::_sub(self, rhs)
    }
}

impl ops::Mul for FPDecimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        FPDecimal::_mul(self, rhs)
    }
}

impl ops::Div for FPDecimal {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        FPDecimal::_div(self, rhs)
    }
}

#[cfg(test)]
mod tests {

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
}
