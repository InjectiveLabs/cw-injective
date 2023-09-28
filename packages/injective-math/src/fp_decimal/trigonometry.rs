use crate::fp_decimal::FPDecimal;

impl FPDecimal {
    pub fn _cos(mut x: FPDecimal) -> FPDecimal {
        x = FPDecimal::_change_range(x);
        FPDecimal::_sin(FPDecimal::PI / FPDecimal::TWO - x)
    }

    fn _sine_taylor_expansion(x: FPDecimal) -> FPDecimal {
        x - (x.pow(FPDecimal::THREE) / FPDecimal::THREE.factorial()) + (x.pow(FPDecimal::FIVE) / FPDecimal::FIVE.factorial())
            - (x.pow(FPDecimal::SEVEN) / FPDecimal::SEVEN.factorial())
            + (x.pow(FPDecimal::NINE) / FPDecimal::NINE.factorial())
            - (x.pow(FPDecimal::TWO + FPDecimal::NINE) / (FPDecimal::TWO + FPDecimal::NINE).factorial())
            + (x.pow(FPDecimal::FOUR + FPDecimal::NINE) / (FPDecimal::FOUR + FPDecimal::NINE).factorial())
    }

    pub fn _sin(mut x: FPDecimal) -> FPDecimal {
        x = FPDecimal::_change_range(x);
        let pi_by_2 = FPDecimal::PI / FPDecimal::TWO;
        let pi_plus_pi_by_2 = FPDecimal::PI + FPDecimal::PI / FPDecimal::TWO;

        if (FPDecimal::ZERO == x) || (FPDecimal::PI == x) {
            return FPDecimal::ZERO;
        }

        if pi_by_2 == x {
            return FPDecimal::ONE;
        }

        if pi_plus_pi_by_2 == x {
            return FPDecimal::ZERO - FPDecimal::ONE;
        }

        if FPDecimal::ZERO < x && x < pi_by_2 {
            return FPDecimal::_sine_taylor_expansion(x);
        }

        if pi_by_2 < x && x < FPDecimal::PI {
            return FPDecimal::_sine_taylor_expansion(FPDecimal::PI - x);
        }

        if FPDecimal::PI < x && x < pi_plus_pi_by_2 {
            let mut output = FPDecimal::_sine_taylor_expansion(x - FPDecimal::PI);
            output.sign = 0;
            return output;
        }

        let mut output = FPDecimal::_sine_taylor_expansion(FPDecimal::PI * FPDecimal::TWO - x);
        output.sign = 0;
        output
    }

    fn _change_range(x: FPDecimal) -> FPDecimal {
        if x.is_zero() {
            return x;
        }
        let mut output = x;
        let two_pi = FPDecimal::PI * FPDecimal::TWO;
        match x < FPDecimal::ZERO {
            true => {
                while output < FPDecimal::ZERO {
                    output += two_pi;
                }
            }
            false => {
                while output > two_pi {
                    output -= two_pi;
                }
            }
        }
        output
    }

    pub fn imprecise_cos(&self) -> FPDecimal {
        FPDecimal::_cos(*self)
    }

    pub fn imprecise_sin(&self) -> FPDecimal {
        FPDecimal::_sin(*self)
    }
}

#[cfg(test)]
mod tests {
    use crate::FPDecimal;
    use std::str::FromStr;
    fn almost_eq(x: FPDecimal, target: FPDecimal) {
        assert!(((x - target) / x).abs() <= FPDecimal::from_str("0.01").unwrap());
    }

    #[test]
    fn test_cosine_zero() {
        assert_eq!(FPDecimal::ZERO.imprecise_cos(), FPDecimal::ONE);
    }

    #[test]
    fn test_cosine_one() {
        almost_eq(FPDecimal::ONE.imprecise_cos(), FPDecimal::from_str("0.54030230586").unwrap());
    }

    #[test]
    fn test_cosine_negative_one() {
        almost_eq(
            (FPDecimal::ZERO - FPDecimal::ONE).imprecise_cos(),
            FPDecimal::from_str("0.54030230586").unwrap(),
        );
    }

    #[test]
    fn test_sine_zero() {
        assert_eq!(FPDecimal::ZERO.imprecise_sin(), FPDecimal::ZERO);
    }

    #[test]
    fn test_sine_one() {
        almost_eq(FPDecimal::ONE.imprecise_sin(), FPDecimal::from_str("0.8414709848").unwrap());
    }

    #[test]
    fn test_sine_negative_one() {
        almost_eq(
            (FPDecimal::ZERO - FPDecimal::ONE).imprecise_sin(),
            FPDecimal::from_str("-0.8414709848").unwrap(),
        );
    }
}
