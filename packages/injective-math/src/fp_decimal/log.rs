/// Logarithmic functions for FPDecimal
use crate::fp_decimal::{FPDecimal, U256};

impl FPDecimal {
    /// natural logarithm
    #[allow(clippy::many_single_char_names)]
    pub fn _ln(a: FPDecimal) -> FPDecimal {
        assert!(a.sign != 0);
        assert!(a != FPDecimal::zero());
        let mut v = a.num;
        let mut r = FPDecimal::zero();
        while v <= FPDecimal::ONE.num / U256([10, 0, 0, 0]) {
            v = v * U256([10, 0, 0, 0]);
            r = r - FPDecimal::LN_10;
        }
        while v >= U256([10, 0, 0, 0]) * FPDecimal::ONE.num {
            v = v / U256([10, 0, 0, 0]);
            r = r + FPDecimal::LN_10;
        }
        while v < FPDecimal::ONE.num {
            v = FPDecimal::_mul(FPDecimal { num: v, sign: 1 }, FPDecimal::E).num;
            r = r - FPDecimal::ONE;
        }
        while v > FPDecimal::E.num {
            v = FPDecimal::_div(FPDecimal { num: v, sign: 1 }, FPDecimal::E).num;
            r = r + FPDecimal::ONE;
        }
        if v == FPDecimal::ONE.num {
            return r;
            // return FPDecimal { num: r, sign: 1 };
        }
        if v == FPDecimal::E.num {
            return r + FPDecimal::ONE;
            // return FPDecimal {
            //     num: FPDecimal::ONE.num + r,
            //     sign: 1,
            // };
        }

        let frac_1_5_fpdec = FPDecimal {
            num: U256([3, 0, 0, 0]) * FPDecimal::ONE.num / U256([2, 0, 0, 0]),
            sign: 1,
        };
        let v = FPDecimal { num: v, sign: 1 } - frac_1_5_fpdec;

        r = r + FPDecimal::LN_1_5;

        let mut m = FPDecimal::ONE * v
            / (v + FPDecimal {
                num: U256([3, 0, 0, 0]) * FPDecimal::ONE.num,
                sign: 1,
            });

        r = r + FPDecimal {
            num: U256([2, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        } * m;
        let m2 = m * m / FPDecimal::ONE;
        let mut i: u64 = 3;

        loop {
            m = m * m2 / FPDecimal::ONE;

            let fpdec_i = FPDecimal {
                num: U256([i, 0, 0, 0]) * FPDecimal::ONE.num,
                sign: 1,
            };
            r = r + FPDecimal {
                num: U256([2, 0, 0, 0]) * FPDecimal::ONE.num,
                sign: 1,
            } * m
                / fpdec_i;
            i += 2;
            if i >= 3 + 2 * FPDecimal::DIGITS as u64 {
                break;
            }
        }
        r
    }

    pub fn ln(&self) -> FPDecimal {
        FPDecimal::_ln(*self)
    }
}

#[cfg(test)]
mod tests {

    use crate::FPDecimal;
    use bigint::U256;

    #[test]
    fn test_ln_sanity() {
        let half = FPDecimal::one().div(2i128);
        println!("{}", FPDecimal::_ln(half)); // works if you comment this out
        let num = FPDecimal::one().mul(5).div(4);
        println!("{}", FPDecimal::_pow(num, half));
    }

    #[test]
    fn test_ln() {
        assert_eq!(FPDecimal::_ln(FPDecimal::E), FPDecimal::ONE);
    }

    #[test]
    fn test_ln10() {
        assert_eq!(
            FPDecimal::_ln(FPDecimal {
                num: U256([10, 0, 0, 0]) * FPDecimal::ONE.num,
                sign: 1
            }),
            FPDecimal::LN_10
        );
    }

    #[test]
    fn test_ln1_5() {
        let three = FPDecimal {
            num: U256([3, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let two = FPDecimal {
            num: U256([2, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let one_point_five = FPDecimal::_div(three, two);
        assert_eq!(FPDecimal::_ln(one_point_five), FPDecimal::LN_1_5);
    }
}
