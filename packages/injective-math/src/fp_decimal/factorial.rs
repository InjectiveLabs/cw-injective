use crate::fp_decimal::FPDecimal;
use std::str::FromStr;
impl FPDecimal {
    fn _factorial(x: FPDecimal) -> FPDecimal {
        if x.is_zero() {
            FPDecimal::ONE
        } else if x < FPDecimal::ZERO {
            x * FPDecimal::_factorial(x + FPDecimal::ONE)
        } else {
            x * FPDecimal::_factorial(x - FPDecimal::ONE)
        }
    }
    pub fn factorial(&self) -> FPDecimal {
        FPDecimal::_factorial(*self)
    }

    pub fn gamma(x: FPDecimal) -> FPDecimal {
        let table: [FPDecimal; 30] = [
            FPDecimal::from_str("1.00000000000000000000").unwrap(),
            FPDecimal::from_str("0.57721566490153286061").unwrap(),
            FPDecimal::from_str("-0.65587807152025388108").unwrap(),
            FPDecimal::from_str("-0.04200263503409523553").unwrap(),
            FPDecimal::from_str("0.16653861138229148950").unwrap(),
            FPDecimal::from_str("-0.04219773455554433675").unwrap(),
            FPDecimal::from_str("-0.00962197152787697356").unwrap(),
            FPDecimal::from_str("0.00721894324666309954").unwrap(),
            FPDecimal::from_str("-0.00116516759185906511").unwrap(),
            FPDecimal::from_str("-0.00021524167411495097").unwrap(),
            FPDecimal::from_str("0.00012805028238811619").unwrap(),
            FPDecimal::from_str("-0.00002013485478078824").unwrap(),
            FPDecimal::from_str("-0.00000125049348214267").unwrap(),
            FPDecimal::from_str("0.00000113302723198170").unwrap(),
            FPDecimal::from_str("-0.00000020563384169776").unwrap(),
            FPDecimal::from_str("0.00000000611609510448").unwrap(),
            FPDecimal::from_str("0.00000000500200764447").unwrap(),
            FPDecimal::from_str("-0.00000000118127457049").unwrap(),
            FPDecimal::from_str("0.00000000010434267117").unwrap(),
            FPDecimal::from_str("0.00000000000778226344").unwrap(),
            FPDecimal::from_str("-0.00000000000369680562").unwrap(),
            FPDecimal::from_str("0.00000000000051003703").unwrap(),
            FPDecimal::from_str("-0.00000000000002058326").unwrap(),
            FPDecimal::from_str("-0.00000000000000534812").unwrap(),
            FPDecimal::from_str("0.00000000000000122678").unwrap(),
            FPDecimal::from_str("-0.00000000000000011813").unwrap(),
            FPDecimal::from_str("0.00000000000000000119").unwrap(),
            FPDecimal::from_str("0.00000000000000000141").unwrap(),
            FPDecimal::from_str("-0.00000000000000000023").unwrap(),
            FPDecimal::from_str("0.00000000000000000002").unwrap(),
        ];
        let y = x - FPDecimal::ONE;

        let mut sm = table[table.len() - 1];
        for i in (0..table.len() - 2).rev() {
            sm = sm * y + table[i];
        }
        FPDecimal::ONE / sm
    }
}
#[cfg(test)]
mod tests {

    use crate::FPDecimal;
    use std::str::FromStr;

    #[test]
    fn test_factorial_nine() {
        let nine = FPDecimal::NINE;
        assert_eq!(nine, FPDecimal::from_str("9").unwrap());
        assert_eq!(FPDecimal::from_str("362880").unwrap(), nine.factorial());
    }

    #[test]
    fn test_factorial_negative_nine() {
        let negative_nine = -FPDecimal::NINE;
        assert_eq!(negative_nine, FPDecimal::from_str("-9").unwrap());
        assert_eq!(FPDecimal::from_str("-362880").unwrap(), negative_nine.factorial());
    }
}
