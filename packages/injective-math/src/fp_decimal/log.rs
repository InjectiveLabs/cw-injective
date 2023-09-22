/// Logarithmic functions for FPDecimal
use crate::fp_decimal::{FPDecimal, U256};

impl FPDecimal {
    pub fn _log_const(self) -> Option<(FPDecimal, u128)> {
        if let Some(value) = self._log2() {
            return Some((value, 2u128));
        }

        if let Some(value) = self._log_e() {
            //NOTE: base e can't be represented by u128, so we  27 in here
            return Some((value, 27u128));
        }
        if let Some(value) = self._log3() {
            return Some((value, 3u128));
        }
        if let Some(value) = self._log5() {
            return Some((value, 5u128));
        }
        if let Some(value) = self._log7() {
            return Some((value, 7u128));
        }
        if let Some(value) = self._log10() {
            return Some((value, 10u128));
        }
        if let Some(value) = self._log11() {
            return Some((value, 11u128));
        }
        None
    }

    pub fn _log_e(self) -> Option<FPDecimal> {
        let e = FPDecimal::E;
        if self == FPDecimal::ONE {
            return Some(FPDecimal::ZERO);
        }
        if self == FPDecimal::E {
            return Some(FPDecimal::ONE);
        }
        if self == e * e {
            return Some(FPDecimal::TWO);
        }
        if self == e * e * e {
            return Some(FPDecimal::THREE);
        }
        if self == e * e * e * e {
            return Some(FPDecimal::FOUR);
        }
        if self == e * e * e * e * e {
            return Some(FPDecimal::FIVE);
        }
        if self == e * e * e * e * e * e {
            return Some(FPDecimal::SIX);
        }
        if self == e * e * e * e * e * e * e {
            return Some(FPDecimal::SEVEN);
        }
        if self == e * e * e * e * e * e * e * e {
            return Some(FPDecimal::EIGHT);
        }
        if self == e * e * e * e * e * e * e * e * e {
            return Some(FPDecimal::NINE);
        }
        if self == e * e * e * e * e * e * e * e * e * e {
            return Some(FPDecimal::TEN);
        }
        if self == FPDecimal::ONE / FPDecimal::E {
            return Some(-FPDecimal::ONE);
        }
        if self == FPDecimal::ONE / (e * e) {
            return Some(-FPDecimal::TWO);
        }
        if self == FPDecimal::ONE / (e * e * e) {
            return Some(-FPDecimal::THREE);
        }
        if self == FPDecimal::ONE / (e * e * e * e) {
            return Some(-FPDecimal::FOUR);
        }
        if self == FPDecimal::ONE / (e * e * e * e * e) {
            return Some(-FPDecimal::FIVE);
        }
        if self == FPDecimal::ONE / (e * e * e * e * e * e) {
            return Some(-FPDecimal::SIX);
        }
        if self == FPDecimal::ONE / (e * e * e * e * e * e * e) {
            return Some(-FPDecimal::SEVEN);
        }
        if self == FPDecimal::ONE / (e * e * e * e * e * e * e * e) {
            return Some(-FPDecimal::EIGHT);
        }
        if self == FPDecimal::ONE / (e * e * e * e * e * e * e * e * e) {
            return Some(-FPDecimal::NINE);
        }
        if self == FPDecimal::ONE / (e * e * e * e * e * e * e * e * e * e) {
            return Some(-FPDecimal::TEN);
        }
        None
    }

    pub fn _log2(self) -> Option<FPDecimal> {
        if self == FPDecimal::ONE {
            return Some(FPDecimal::ZERO);
        }
        if self == FPDecimal::TWO {
            return Some(FPDecimal::ONE);
        }
        if self == FPDecimal::FOUR {
            return Some(FPDecimal::TWO);
        }
        if self == FPDecimal::EIGHT {
            return Some(FPDecimal::THREE);
        }
        if self == FPDecimal::from(16u128) {
            return Some(FPDecimal::FOUR);
        }
        if self == FPDecimal::from(32u128) {
            return Some(FPDecimal::FIVE);
        }
        if self == FPDecimal::from(64u128) {
            return Some(FPDecimal::SIX);
        }
        if self == FPDecimal::from(128u128) {
            return Some(FPDecimal::SEVEN);
        }
        if self == FPDecimal::from(256u128) {
            return Some(FPDecimal::EIGHT);
        }
        if self == FPDecimal::from(512u128) {
            return Some(FPDecimal::NINE);
        }
        if self == FPDecimal::from(1024u128) {
            return Some(FPDecimal::TEN);
        }
        if self == FPDecimal::ONE / FPDecimal::TWO {
            return Some(-FPDecimal::ONE);
        }
        if self == FPDecimal::ONE / FPDecimal::FOUR {
            return Some(-FPDecimal::TWO);
        }
        if self == FPDecimal::ONE / FPDecimal::EIGHT {
            return Some(-FPDecimal::THREE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(16u128) {
            return Some(-FPDecimal::FOUR);
        }
        if self == FPDecimal::ONE / FPDecimal::from(32u128) {
            return Some(-FPDecimal::FIVE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(64u128) {
            return Some(-FPDecimal::SIX);
        }
        if self == FPDecimal::ONE / FPDecimal::from(128u128) {
            return Some(-FPDecimal::SEVEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(256u128) {
            return Some(-FPDecimal::EIGHT);
        }
        if self == FPDecimal::ONE / FPDecimal::from(512u128) {
            return Some(-FPDecimal::NINE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(1024u128) {
            return Some(-FPDecimal::TEN);
        }
        None
    }

    pub fn _log3(self) -> Option<FPDecimal> {
        if self == FPDecimal::ONE {
            return Some(FPDecimal::ZERO);
        }
        if self == FPDecimal::THREE {
            return Some(FPDecimal::ONE);
        }
        if self == FPDecimal::NINE {
            return Some(FPDecimal::TWO);
        }
        if self == FPDecimal::from(27u128) {
            return Some(FPDecimal::THREE);
        }
        if self == FPDecimal::from(81u128) {
            return Some(FPDecimal::FOUR);
        }
        if self == FPDecimal::from(243u128) {
            return Some(FPDecimal::FIVE);
        }
        if self == FPDecimal::from(729u128) {
            return Some(FPDecimal::SIX);
        }
        if self == FPDecimal::from(2187u128) {
            return Some(FPDecimal::SEVEN);
        }
        if self == FPDecimal::from(6561u128) {
            return Some(FPDecimal::EIGHT);
        }
        if self == FPDecimal::from(19683u128) {
            return Some(FPDecimal::NINE);
        }
        if self == FPDecimal::from(59049u128) {
            return Some(FPDecimal::TEN);
        }

        if self == FPDecimal::ONE / FPDecimal::THREE {
            return Some(-FPDecimal::ONE);
        }
        if self == FPDecimal::ONE / FPDecimal::NINE {
            return Some(-FPDecimal::TWO);
        }
        if self == FPDecimal::ONE / FPDecimal::from(27u128) {
            return Some(-FPDecimal::THREE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(81u128) {
            return Some(-FPDecimal::FOUR);
        }
        if self == FPDecimal::ONE / FPDecimal::from(243u128) {
            return Some(-FPDecimal::FIVE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(729u128) {
            return Some(-FPDecimal::SIX);
        }
        if self == FPDecimal::ONE / FPDecimal::from(2187u128) {
            return Some(-FPDecimal::SEVEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(6561u128) {
            return Some(-FPDecimal::EIGHT);
        }
        if self == FPDecimal::ONE / FPDecimal::from(19683u128) {
            return Some(-FPDecimal::NINE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(59049u128) {
            return Some(-FPDecimal::TEN);
        }
        None
    }

    pub fn _log5(self) -> Option<FPDecimal> {
        if self == FPDecimal::ONE {
            return Some(FPDecimal::ZERO);
        }
        if self == FPDecimal::FIVE {
            return Some(FPDecimal::ONE);
        }
        if self == FPDecimal::from(25u128) {
            return Some(FPDecimal::TWO);
        }
        if self == FPDecimal::from(125u128) {
            return Some(FPDecimal::THREE);
        }
        if self == FPDecimal::from(625u128) {
            return Some(FPDecimal::FOUR);
        }
        if self == FPDecimal::from(3125u128) {
            return Some(FPDecimal::FIVE);
        }
        if self == FPDecimal::from(15625u128) {
            return Some(FPDecimal::SIX);
        }
        if self == FPDecimal::from(78125u128) {
            return Some(FPDecimal::SEVEN);
        }
        if self == FPDecimal::from(3906251u128) {
            return Some(FPDecimal::EIGHT);
        }
        if self == FPDecimal::from(1953125u128) {
            return Some(FPDecimal::NINE);
        }
        if self == FPDecimal::from(9765625u128) {
            return Some(FPDecimal::TEN);
        }
        if self == FPDecimal::ONE / FPDecimal::FIVE {
            return Some(-FPDecimal::ONE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(25u128) {
            return Some(-FPDecimal::TWO);
        }
        if self == FPDecimal::ONE / FPDecimal::from(125u128) {
            return Some(-FPDecimal::THREE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(625u128) {
            return Some(-FPDecimal::FOUR);
        }
        if self == FPDecimal::ONE / FPDecimal::from(3125u128) {
            return Some(-FPDecimal::FIVE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(15625u128) {
            return Some(-FPDecimal::SIX);
        }
        if self == FPDecimal::ONE / FPDecimal::from(78125u128) {
            return Some(-FPDecimal::SEVEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(3906251u128) {
            return Some(-FPDecimal::EIGHT);
        }
        if self == FPDecimal::ONE / FPDecimal::from(1953125u128) {
            return Some(-FPDecimal::NINE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(9765625u128) {
            return Some(-FPDecimal::TEN);
        }
        None
    }

    // 7^1..10
    pub fn _log7(self) -> Option<FPDecimal> {
        if self == FPDecimal::ONE {
            return Some(FPDecimal::ZERO);
        }
        if self == FPDecimal::SEVEN {
            return Some(FPDecimal::ONE);
        }
        if self == FPDecimal::from(49u128) {
            return Some(FPDecimal::TWO);
        }
        if self == FPDecimal::from(343u128) {
            return Some(FPDecimal::THREE);
        }
        if self == FPDecimal::from(2401u128) {
            return Some(FPDecimal::FOUR);
        }
        if self == FPDecimal::from(16807u128) {
            return Some(FPDecimal::FIVE);
        }
        if self == FPDecimal::from(117649u128) {
            return Some(FPDecimal::SIX);
        }
        if self == FPDecimal::from(823543u128) {
            return Some(FPDecimal::SEVEN);
        }
        if self == FPDecimal::from(5764801u128) {
            return Some(FPDecimal::EIGHT);
        }
        if self == FPDecimal::from(40353607u128) {
            return Some(FPDecimal::NINE);
        }
        if self == FPDecimal::from(282475249u128) {
            return Some(FPDecimal::TEN);
        }
        if self == FPDecimal::ONE / FPDecimal::SEVEN {
            return Some(-FPDecimal::ONE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(49u128) {
            return Some(-FPDecimal::TWO);
        }
        if self == FPDecimal::ONE / FPDecimal::from(343u128) {
            return Some(-FPDecimal::THREE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(2401u128) {
            return Some(-FPDecimal::FOUR);
        }
        if self == FPDecimal::ONE / FPDecimal::from(16807u128) {
            return Some(-FPDecimal::FIVE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(117649u128) {
            return Some(-FPDecimal::SIX);
        }
        if self == FPDecimal::ONE / FPDecimal::from(823543u128) {
            return Some(-FPDecimal::SEVEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(5764801u128) {
            return Some(-FPDecimal::EIGHT);
        }
        if self == FPDecimal::ONE / FPDecimal::from(40353607u128) {
            return Some(-FPDecimal::NINE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(282475249u128) {
            return Some(-FPDecimal::TEN);
        }
        None
    }

    pub fn _log10(self) -> Option<FPDecimal> {
        if self == FPDecimal::ONE {
            return Some(FPDecimal::ZERO);
        }
        if self == FPDecimal::TEN {
            return Some(FPDecimal::ONE);
        }
        if self == FPDecimal::from(100u128) {
            return Some(FPDecimal::TWO);
        }
        if self == FPDecimal::from(1_000u128) {
            return Some(FPDecimal::THREE);
        }
        if self == FPDecimal::from(10_000u128) {
            return Some(FPDecimal::FOUR);
        }
        if self == FPDecimal::from(100_000u128) {
            return Some(FPDecimal::FIVE);
        }
        if self == FPDecimal::from(1_000_000u128) {
            return Some(FPDecimal::SIX);
        }
        if self == FPDecimal::from(10_000_000u128) {
            return Some(FPDecimal::SEVEN);
        }
        if self == FPDecimal::from(100_000_000u128) {
            return Some(FPDecimal::EIGHT);
        }
        if self == FPDecimal::from(1_000_000_000u128) {
            return Some(FPDecimal::NINE);
        }
        if self == FPDecimal::from(10_000_000_000u128) {
            return Some(FPDecimal::TEN);
        }

        if self == FPDecimal::ONE / FPDecimal::TEN {
            return Some(-FPDecimal::ONE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(100u128) {
            return Some(-FPDecimal::TWO);
        }
        if self == FPDecimal::ONE / FPDecimal::from(1_000u128) {
            return Some(-FPDecimal::THREE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(10_000u128) {
            return Some(-FPDecimal::FOUR);
        }
        if self == FPDecimal::ONE / FPDecimal::from(100_000u128) {
            return Some(-FPDecimal::FIVE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(1_000_000u128) {
            return Some(-FPDecimal::SIX);
        }
        if self == FPDecimal::ONE / FPDecimal::from(10_000_000u128) {
            return Some(-FPDecimal::SEVEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(100_000_000u128) {
            return Some(-FPDecimal::EIGHT);
        }
        if self == FPDecimal::ONE / FPDecimal::from(1_000_000_000u128) {
            return Some(-FPDecimal::NINE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(10_000_000_000u128) {
            return Some(-FPDecimal::TEN);
        }
        None
    }

    // 11^1..10
    pub fn _log11(self) -> Option<FPDecimal> {
        if self == FPDecimal::ONE {
            return Some(FPDecimal::ZERO);
        }
        if self == FPDecimal::from(11u128) {
            return Some(FPDecimal::ONE);
        }
        if self == FPDecimal::from(121u128) {
            return Some(FPDecimal::TWO);
        }
        if self == FPDecimal::from(1331u128) {
            return Some(FPDecimal::THREE);
        }
        if self == FPDecimal::from(14641u128) {
            return Some(FPDecimal::FOUR);
        }
        if self == FPDecimal::from(161051u128) {
            return Some(FPDecimal::FIVE);
        }
        if self == FPDecimal::from(1771561u128) {
            return Some(FPDecimal::SIX);
        }
        if self == FPDecimal::from(19487171u128) {
            return Some(FPDecimal::SEVEN);
        }
        if self == FPDecimal::from(214358881u128) {
            return Some(FPDecimal::EIGHT);
        }
        if self == FPDecimal::from(2357947691u128) {
            return Some(FPDecimal::NINE);
        }
        if self == FPDecimal::from(25937424601u128) {
            return Some(FPDecimal::TEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(11u128) {
            return Some(-FPDecimal::ONE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(121u128) {
            return Some(-FPDecimal::TWO);
        }
        if self == FPDecimal::ONE / FPDecimal::from(1331u128) {
            return Some(-FPDecimal::THREE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(14641u128) {
            return Some(-FPDecimal::FOUR);
        }
        if self == FPDecimal::ONE / FPDecimal::from(161051u128) {
            return Some(-FPDecimal::FIVE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(1771561u128) {
            return Some(-FPDecimal::SIX);
        }
        if self == FPDecimal::ONE / FPDecimal::from(19487171u128) {
            return Some(-FPDecimal::SEVEN);
        }
        if self == FPDecimal::ONE / FPDecimal::from(214358881u128) {
            return Some(-FPDecimal::EIGHT);
        }
        if self == FPDecimal::ONE / FPDecimal::from(2357947691u128) {
            return Some(-FPDecimal::NINE);
        }
        if self == FPDecimal::ONE / FPDecimal::from(25937424601u128) {
            return Some(-FPDecimal::TEN);
        }
        None
    }

    pub fn _log(a: FPDecimal, base: FPDecimal) -> FPDecimal {
        // NOTE: only accurate 1,3,5,7,11, and combinations of these 4 numbers
        //log_base^b = ln(a)/ln(base)
        if a == FPDecimal::ONE {
            return FPDecimal::ZERO;
        }
        if a == FPDecimal::ZERO {
            // FIXME should be an undefined, not sure if it will be better to just add assert!(b>0)
            panic!("Undefined");
        }

        a.ln() / base.ln()
    }

    /// natural logarithm
    #[allow(clippy::many_single_char_names)]
    pub fn _ln(a: FPDecimal) -> FPDecimal {
        assert!(a.sign != 0);
        assert!(a != FPDecimal::zero());
        let mut v = a.num;
        let mut r = FPDecimal::zero();
        while v <= FPDecimal::ONE.num / U256([10, 0, 0, 0]) {
            v = v * U256([10, 0, 0, 0]);
            r -= FPDecimal::LN_10;
        }
        while v >= U256([10, 0, 0, 0]) * FPDecimal::ONE.num {
            v = v / U256([10, 0, 0, 0]);
            r += FPDecimal::LN_10;
        }
        while v < FPDecimal::ONE.num {
            v = FPDecimal::_mul(FPDecimal { num: v, sign: 1 }, FPDecimal::E).num;
            r -= FPDecimal::ONE;
        }
        while v > FPDecimal::E.num {
            v = FPDecimal::_div(FPDecimal { num: v, sign: 1 }, FPDecimal::E).num;
            r += FPDecimal::ONE;
        }
        if v == FPDecimal::ONE.num {
            return r;
        }
        if v == FPDecimal::E.num {
            return r + FPDecimal::ONE;
        }

        let frac_1_5_fpdec = FPDecimal {
            num: U256([3, 0, 0, 0]) * FPDecimal::ONE.num / U256([2, 0, 0, 0]),
            sign: 1,
        };
        let v = FPDecimal { num: v, sign: 1 } - frac_1_5_fpdec;

        r += FPDecimal::LN_1_5;

        let mut m = FPDecimal::ONE * v
            / (v + FPDecimal {
                num: U256([3, 0, 0, 0]) * FPDecimal::ONE.num,
                sign: 1,
            });

        r += FPDecimal {
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
            r += FPDecimal {
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
        if let Some(value) = self._log_e() {
            return value;
        }
        FPDecimal::_ln(*self)
    }

    pub fn log(&self, base: FPDecimal) -> FPDecimal {
        assert!(base > FPDecimal::ZERO);
        if *self == FPDecimal::ONE {
            return FPDecimal::ZERO;
        }
        if *self == FPDecimal::ZERO {
            // FIXME should be an undefined, not sure if it will be better to just add assert!(b>0)
            return FPDecimal::SMALLEST_PRECISION;
        }

        if base == FPDecimal::E {
            return self.ln();
        }
        let numerator = self._log_const();
        let denominator = base._log_const();
        match (numerator, denominator) {
            (Some((n, nbase)), Some((d, dbase))) => {
                if dbase == nbase {
                    return n / d;
                }
                FPDecimal::_log(*self, base)
            }
            (_, _) => FPDecimal::_log(*self, base),
        }
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
        println!("{}", FPDecimal::checked_positive_pow(num, half).unwrap());
    }

    #[test]
    fn test_ln() {
        assert_eq!(FPDecimal::E.ln(), FPDecimal::ONE);
    }

    #[test]
    fn test_ln10() {
        assert_eq!(
            FPDecimal {
                num: U256([10, 0, 0, 0]) * FPDecimal::ONE.num,
                sign: 1
            }
            .ln(),
            FPDecimal::LN_10
        );
    }
    #[test]
    fn test_log_2_8() {
        assert_eq!(FPDecimal::EIGHT.log(FPDecimal::TWO), FPDecimal::THREE);
    }

    #[test]
    fn test_log_11_8() {
        assert_eq!(
            FPDecimal::EIGHT.log(FPDecimal::from(11u128)),
            FPDecimal::must_from_str("0.867194478953663578")
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
        assert_eq!(one_point_five.ln(), FPDecimal::LN_1_5);
    }

    #[test]
    fn test_ln2_3() {
        let three = FPDecimal {
            num: U256([3, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let two = FPDecimal {
            num: U256([2, 0, 0, 0]) * FPDecimal::ONE.num,
            sign: 1,
        };
        let two_point_three = two + three / FPDecimal::from(10u128);
        assert_eq!(two_point_three.ln(), FPDecimal::must_from_str("0.832909122935103999"));
    }

    #[test]
    fn test_ln4_16() {
        let a = FPDecimal::from(16u128);
        let b = FPDecimal::FOUR;
        assert_eq!(a.log(b), FPDecimal::TWO);
    }

    #[test]
    fn test_log_e_16() {
        let a = FPDecimal::from(16u128);
        let b = FPDecimal::FOUR;
        assert_eq!(a.log(b), FPDecimal::TWO);
    }
}
