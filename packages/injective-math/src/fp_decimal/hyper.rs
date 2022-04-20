/// Hyperbolic Trig functions for FPDecimal
use crate::fp_decimal::{FPDecimal, U256};

impl FPDecimal {
    pub fn _sinh(x: FPDecimal) -> FPDecimal {
        let neg_x: FPDecimal = FPDecimal {
            num: x.num,
            sign: 1 - x.sign,
        };
        let denominator = FPDecimal {
            num: FPDecimal::ONE.num * U256([2, 0, 0, 0]),
            sign: 1,
        };
        let numerator: FPDecimal = FPDecimal::_sub(FPDecimal::_exp(x), FPDecimal::_exp(neg_x));
        FPDecimal::_div(numerator, denominator)
    }

    pub fn sinh(&self) -> FPDecimal {
        FPDecimal::_sinh(*self)
    }

    pub fn _cosh(x: FPDecimal) -> FPDecimal {
        let neg_x: FPDecimal = FPDecimal {
            num: x.num,
            sign: 1 - x.sign,
        };
        let denominator = FPDecimal {
            num: FPDecimal::ONE.num * U256([2, 0, 0, 0]),
            sign: 1,
        };
        let numerator: FPDecimal = FPDecimal::_add(FPDecimal::_exp(x), FPDecimal::_exp(neg_x));
        FPDecimal::_div(numerator, denominator)
    }

    pub fn cosh(&self) -> FPDecimal {
        FPDecimal::_cosh(*self)
    }

    pub fn _tanh(x: FPDecimal) -> FPDecimal {
        FPDecimal::_div(FPDecimal::_sinh(x), FPDecimal::_cosh(x))
    }

    pub fn tanh(&self) -> FPDecimal {
        FPDecimal::_tanh(*self)
    }
}

#[cfg(test)]
mod tests {

    use crate::FPDecimal;
    use std::str::FromStr;

    #[test]
    fn test_sinh() {
        assert_eq!(
            FPDecimal::_sinh(FPDecimal::ONE),
            FPDecimal::from_str("1.175201193643801457").unwrap()
        );
    }

    #[test]
    fn test_cosh() {
        assert_eq!(
            FPDecimal::_cosh(FPDecimal::ONE),
            FPDecimal::from_str("1.543080634815243778").unwrap()
        );
    }

    #[test]
    fn test_tanh() {
        assert_eq!(
            FPDecimal::_tanh(FPDecimal::ONE),
            FPDecimal::from_str("0.761594155955764888").unwrap()
        );
    }
}
