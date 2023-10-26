use std::str::FromStr;
use std::{convert::TryFrom, ops::Neg};

use cosmwasm_std::{Decimal256, StdError, Uint128, Uint256};
use primitive_types::U256;
use schemars::JsonSchema;

#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Default, Debug, Eq, JsonSchema)]
pub struct FPDecimal {
    #[schemars(with = "String")]
    pub num: U256,
    pub sign: i8,
}

impl From<u128> for FPDecimal {
    fn from(x: u128) -> FPDecimal {
        FPDecimal {
            num: U256::from_little_endian(&x.to_le_bytes()) * FPDecimal::ONE.num,
            sign: 1,
        }
    }
}

impl From<i128> for FPDecimal {
    fn from(x: i128) -> FPDecimal {
        let mut sign = 1;
        if x < 0 {
            sign = 0;
        }

        let abs_x: u128 = x.unsigned_abs();
        FPDecimal {
            num: U256::from_little_endian(&abs_x.to_le_bytes()) * FPDecimal::ONE.num,
            sign,
        }
    }
}

impl From<FPDecimal> for u128 {
    fn from(x: FPDecimal) -> u128 {
        let num: U256 = x.int().num / FPDecimal::ONE.num;
        if num.bits() > 128 {
            panic!("overflow");
        }

        let mut array: [u8; 32] = [0; 32];
        num.to_little_endian(&mut array);

        let mut arr2: [u8; 16] = Default::default();
        arr2.copy_from_slice(&array[0..16]);
        u128::from_le_bytes(arr2)
    }
}

impl From<U256> for FPDecimal {
    fn from(x: U256) -> FPDecimal {
        FPDecimal { num: x, sign: 1 }
    }
}

impl From<FPDecimal> for Uint128 {
    fn from(x: FPDecimal) -> Uint128 {
        let number: u128 = x.into();
        number.into()
    }
}

impl From<Uint128> for FPDecimal {
    fn from(x: Uint128) -> FPDecimal {
        FPDecimal::from_str(&x.to_string()).unwrap()
    }
}

impl From<Uint256> for FPDecimal {
    fn from(x: Uint256) -> FPDecimal {
        FPDecimal::from_str(&x.to_string()).unwrap()
    }
}

// impl that converts cosmwasm Decimal256 to FPDecimal.
impl From<Decimal256> for FPDecimal {
    fn from(value: Decimal256) -> FPDecimal {
        let atomics = value.atomics().to_be_bytes();
        FPDecimal {
            num: atomics.into(),
            sign: 1,
        }
    }
}

// impl that tries to convert FPDecimal into Decimal256.
impl TryFrom<FPDecimal> for Decimal256 {
    type Error = StdError;

    fn try_from(fp_decimal: FPDecimal) -> Result<Self, Self::Error> {
        if fp_decimal.is_negative() {
            return Err(StdError::generic_err(format!("Value {} must be >= {}", fp_decimal.num, 0)));
        }

        let fp_decimal_num_uint256 = fp_decimal.to_u256();

        Ok(Decimal256::new(fp_decimal_num_uint256))
    }
}

impl Neg for FPDecimal {
    type Output = FPDecimal;
    fn neg(mut self) -> Self::Output {
        if self.is_zero() {
            return self;
        }
        match self.sign {
            0 => {
                self.sign = 1;
            }
            _ => {
                self.sign = 0;
            }
        }
        self
    }
}

// constants
impl FPDecimal {
    pub const MAX: FPDecimal = FPDecimal { num: U256::MAX, sign: 1 };
    pub const MIN: FPDecimal = FPDecimal { num: U256::MAX, sign: 0 };
    pub const DIGITS: usize = 18;
    pub const NEGATIVE_ONE: FPDecimal = FPDecimal {
        num: U256([1_000_000_000_000_000_000, 0, 0, 0]),
        sign: 0,
    };
    pub const ZERO: FPDecimal = FPDecimal {
        num: U256([0, 0, 0, 0]),
        sign: 1,
    };

    pub const LN2: FPDecimal = FPDecimal {
        num: U256([693_147_180_559_945_309, 0, 0, 0]),
        sign: 1,
    };

    pub const ONE: FPDecimal = FPDecimal {
        num: U256([1_000_000_000_000_000_000, 0, 0, 0]),
        sign: 1,
    };

    pub const TWO: FPDecimal = FPDecimal {
        num: U256([2_000_000_000_000_000_000, 0, 0, 0]),
        sign: 1,
    };

    pub const THREE: FPDecimal = FPDecimal {
        num: U256([3_000_000_000_000_000_000, 0, 0, 0]),
        sign: 1,
    };

    pub const FOUR: FPDecimal = FPDecimal {
        num: U256([4_000_000_000_000_000_000, 0, 0, 0]),
        sign: 1,
    };

    pub const FIVE: FPDecimal = FPDecimal {
        num: U256([5_000_000_000_000_000_000, 0, 0, 0]),
        sign: 1,
    };
    pub const SIX: FPDecimal = FPDecimal {
        num: U256([6_000_000_000_000_000_000, 0, 0, 0]),
        sign: 1,
    };
    pub const SEVEN: FPDecimal = FPDecimal {
        num: U256([7_000_000_000_000_000_000, 0, 0, 0]),
        sign: 1,
    };
    pub const EIGHT: FPDecimal = FPDecimal {
        num: U256([8_000_000_000_000_000_000, 0, 0, 0]),
        sign: 1,
    };
    pub const NINE: FPDecimal = FPDecimal {
        num: U256([9_000_000_000_000_000_000, 0, 0, 0]),
        sign: 1,
    };
    pub const TEN: FPDecimal = FPDecimal {
        num: U256([10_000_000_000_000_000_000, 0, 0, 0]),
        sign: 1,
    };

    pub const ELEVEN: FPDecimal = FPDecimal {
        num: U256([11_000_000_000_000_000_000, 0, 0, 0]),
        sign: 1,
    };

    pub const SMALLEST_PRECISION: FPDecimal = FPDecimal {
        num: U256([1, 0, 0, 0]),
        sign: 1,
    };

    pub const MUL_PRECISION: FPDecimal = FPDecimal {
        num: U256([1_000_000_000, 0, 0, 0]),
        sign: 1,
    };

    pub const E: FPDecimal = FPDecimal {
        // 1_000_000_000_000_000_000
        num: U256([2_718_281_828_459_045_235, 0, 0, 0]),
        sign: 1,
    };

    pub const E_10: FPDecimal = FPDecimal {
        num: U256([1_053_370_797_511_854_089u64, 1194u64, 0, 0]),
        sign: 1,
    }; // e^10

    pub const LN_1_5: FPDecimal = FPDecimal {
        // 405_465_108_704_634_977
        // 0.40546510810816438199
        num: U256([405_465_108_108_164_382, 0, 0, 0]),
        sign: 1,
    }; // ln(1.5)

    pub const LN_10: FPDecimal = FPDecimal {
        num: U256([2_302_585_092_994_045_684, 0, 0, 0]),
        sign: 1,
    }; // ln(10)

    pub const PI: FPDecimal = FPDecimal {
        num: U256([3_141_592_653_589_793_115, 0, 0, 0]),
        sign: 1,
    };

    pub fn is_zero(&self) -> bool {
        self.num.is_zero()
    }

    pub fn is_negative(&self) -> bool {
        // Zero is positive regardless of sign
        if self.num == FPDecimal::ZERO.num {
            return false;
        }

        self.sign == 0
    }

    pub fn _int(x: FPDecimal) -> Self {
        let x1 = x.num;
        let x1_1 = x1 / FPDecimal::ONE.num;
        let x_final = x1_1 * FPDecimal::ONE.num;
        FPDecimal { num: x_final, sign: x.sign }
    }

    pub fn int(&self) -> Self {
        FPDecimal::_int(*self)
    }
    pub fn is_int(&self) -> bool {
        *self == self.int()
    }

    pub fn _fraction(x: FPDecimal) -> Self {
        let x1 = x.num;
        FPDecimal {
            num: x1 - FPDecimal::_int(x).num,
            sign: x.sign,
        }
    }

    pub fn fraction(&self) -> Self {
        FPDecimal::_fraction(*self)
    }

    pub fn into_uint256_ceil(self) -> Uint256 {
        let uint256 = self.to_u256();
        uint256.checked_add(1u64.into()).unwrap() // Add 1 to round up
    }

    pub fn into_uint256_floor(self) -> Uint256 {
        self.to_u256()
    }

    pub fn to_u256(&self) -> Uint256 {
        let mut bytes = [0u8; 32];
        self.num.to_big_endian(&mut bytes);
        Uint256::from_be_bytes(bytes)
    }
}

mod arithmetic;
mod comparison;
mod display;
mod exp;
mod factorial;
mod from_str;
mod hyper;
mod log;
pub mod scale;
mod serde;
mod trigonometry;

#[cfg(test)]
mod tests {
    use std::{convert::TryFrom, str::FromStr};

    use crate::fp_decimal::U256;
    use crate::FPDecimal;
    use cosmwasm_std::{Decimal256, Uint256};

    #[test]
    fn test_const_pi() {
        let pi = FPDecimal::PI;
        let three_point_two = FPDecimal::must_from_str("3.2");
        let three_point_one = FPDecimal::must_from_str("3.1");
        let pi_precise = FPDecimal::must_from_str("3.141592653589793115");
        assert!(three_point_one < pi);
        assert!(pi < three_point_two);
        assert_eq!(pi, pi_precise);
    }

    #[test]
    fn test_const_ln2() {
        let ln2 = FPDecimal::LN2;
        let ln2_precise = FPDecimal::must_from_str("0.693147180559945309");
        assert_eq!(ln2, ln2_precise);
    }

    #[test]
    fn test_neg_sign() {
        let lhs = FPDecimal::ZERO - FPDecimal::ONE;
        let rhs = -FPDecimal::ONE;
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_neg_zero() {
        let lhs = FPDecimal::ZERO;
        let rhs = -FPDecimal::ZERO;
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_is_int() {
        assert!(FPDecimal::TWO.is_int());
    }

    #[test]
    fn test_is_not_int() {
        assert!(!FPDecimal::must_from_str("2.1").is_int());
        assert_eq!(FPDecimal::must_from_str("2.1") % FPDecimal::ONE, FPDecimal::must_from_str("0.1"));
    }

    #[test]
    fn test_to_u256() {
        let fp_decimal = FPDecimal {
            num: U256::from(12345u64),
            sign: 1, // Assuming it's always positive
        };

        let uint256 = fp_decimal.to_u256();
        assert_eq!(uint256, Uint256::from(12345u64));
    }

    #[test]
    fn into_uint256_floor() {
        let fp_decimal = FPDecimal {
            num: U256::from_dec_str("12345").unwrap(),
            sign: 1,
        };

        let uint256 = fp_decimal.into_uint256_floor();
        assert_eq!(uint256, Uint256::from(12345u64));
    }

    #[test]
    fn into_uint256_ceil() {
        let fp_decimal = FPDecimal {
            num: U256::from_dec_str("12345").unwrap(),
            sign: 1,
        };

        let uint256 = fp_decimal.into_uint256_ceil();
        assert_eq!(uint256, Uint256::from(12346u64));
    }

    #[test]
    fn dec256_to_fpdecimal_conversion() {
        // Decimal value for testing
        let decimal_value = Decimal256::from_str("1.234").unwrap(); // returns 1.234

        //  Perform the conversion
        let fp_decimal: FPDecimal = decimal_value.into();

        // Check if the conversion produced the expected FPDecimal

        let expected_fp_decimal = FPDecimal::must_from_str("1.234");

        // Use assertions to check if the actual value matches the expected value
        assert_eq!(fp_decimal.num, expected_fp_decimal.num);
        assert_eq!(fp_decimal.sign, expected_fp_decimal.sign);
    }

    #[test]
    fn fpdecimal_to_dec256() {
        // Test a valid positive value
        let fp_decimal_valid = FPDecimal::must_from_str("1.2345");
        let decimal_valid = Decimal256::try_from(fp_decimal_valid).unwrap();
        assert_eq!(decimal_valid.to_string(), fp_decimal_valid.to_string());

        // Test a valid zero value
        let fp_decimal_zero = FPDecimal::ZERO;
        let decimal_zero = Decimal256::try_from(fp_decimal_zero).unwrap();
        assert_eq!(decimal_zero.to_string(), "0");

        // Test a negative value (should fail)
        let fp_decimal_negative = FPDecimal::must_from_str("-1.2345");
        assert!(Decimal256::try_from(fp_decimal_negative).is_err());
    }
}
