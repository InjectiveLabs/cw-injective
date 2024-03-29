use crate::fp_decimal::{FPDecimal, U256};
use cosmwasm_std::StdError;
use std::str::FromStr;

impl FromStr for FPDecimal {
    type Err = StdError;

    /// Converts the decimal string to a FPDecimal
    /// Possible inputs: "1.23", "1", "000012", "1.123000000"
    /// Disallowed: "", ".23"
    ///
    /// This never performs any kind of rounding.
    /// More than 18 fractional digits, even zeros, result in an error.
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut sign = if input.starts_with('-') { 0 } else { 1 };
        let parts = input.trim_start_matches('-').split('.').collect::<Vec<&str>>();
        match parts.len() {
            1 => {
                let integer = U256::from_dec_str(parts[0]).map_err(|_| StdError::generic_err("Error parsing integer"))?;
                if integer == U256([0, 0, 0, 0]) {
                    sign = 1;
                }
                Ok(FPDecimal {
                    num: integer * FPDecimal::ONE.num,
                    sign,
                })
            }
            2 => {
                let integer = U256::from_dec_str(parts[0]).map_err(|_| StdError::generic_err("Error parsing integer"))?;
                let fraction = U256::from_dec_str(parts[1]).map_err(|_| StdError::generic_err("Error parsing fraction"))?;
                let exp = FPDecimal::DIGITS
                    .checked_sub(parts[1].len())
                    .ok_or_else(|| StdError::generic_err(format!("Cannot parse more than {} fractional digits", FPDecimal::DIGITS)))?;

                Ok(FPDecimal {
                    num: integer * FPDecimal::ONE.num + fraction * U256::exp10(exp),
                    sign,
                })
            }
            _ => Err(StdError::generic_err("Unexpected number of dots")),
        }
    }
}

impl FPDecimal {
    pub fn must_from_str(input: &str) -> Self {
        Self::from_str(input).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::FPDecimal;
    use primitive_types::U256;
    use std::str::FromStr;

    #[test]
    fn test_from_str_neg() {
        //-0.69314718055994530943
        assert_eq!((FPDecimal::ONE / FPDecimal::TWO).ln(), FPDecimal::must_from_str("-0.693147180435828445"));
        assert_eq!(
            (FPDecimal::ONE / FPDecimal::must_from_str("1.9")).ln(),
            FPDecimal::must_from_str("-0.641853885737462182")
        );
        assert_eq!(
            (FPDecimal::ONE / FPDecimal::THREE).ln(),
            FPDecimal::must_from_str("-1.098612288365102671")
        );
    }

    #[test]
    fn test_from_str_zero() {
        let zero = FPDecimal::from_str("0").unwrap();
        let neg_zero = FPDecimal::from_str("-0").unwrap();
        let zero_zero = FPDecimal::from_str("00").unwrap();
        let neg_zero_zero = FPDecimal::from_str("-00").unwrap();
        assert_eq!(zero, FPDecimal::ZERO);
        assert_eq!(zero_zero, FPDecimal::ZERO);
        assert_eq!(neg_zero, FPDecimal::ZERO);
        assert_eq!(neg_zero_zero, FPDecimal::ZERO);
    }

    #[test]
    fn test_from_str() {
        let val = FPDecimal::from_str("-1.23");
        assert_eq!(
            val.unwrap(),
            FPDecimal {
                num: U256([123, 0, 0, 0]) * FPDecimal::ONE.num / U256::from(100),
                sign: 0
            }
        );
    }

    #[test]
    fn test_from_str_one() {
        let val = FPDecimal::from_str("1");
        assert_eq!(val.unwrap(), FPDecimal::ONE);
    }
}
