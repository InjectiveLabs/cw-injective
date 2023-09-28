use crate::fp_decimal::FPDecimal;

pub trait Scaled {
    fn scaled(self, digits: i32) -> Self;
}

impl Scaled for FPDecimal {
    fn scaled(self, digits: i32) -> Self {
        self.to_owned() * FPDecimal::from(10i128).pow(FPDecimal::from(digits as i128))
    }
}

pub fn dec_scale_factor() -> FPDecimal {
    FPDecimal::one().scaled(18)
}

#[cfg(test)]
mod tests {
    use crate::fp_decimal::scale::{dec_scale_factor, Scaled};
    use crate::FPDecimal;

    #[test]
    fn test_scale_descale() {
        let val = FPDecimal::must_from_str("1000000000000000000");
        let descaled = val.scaled(-18);
        assert_eq!(descaled, FPDecimal::must_from_str("1"), "FPDecimal wasn't correctly scaled down");
        let scaled = descaled.scaled(18);
        assert_eq!(scaled, val, "FPDecimal wasn't correctly scaled up");
    }

    #[test]
    fn test_scale_factor() {
        assert_eq!(dec_scale_factor(), FPDecimal::must_from_str("1000000000000000000"));
    }
}
