use bigint::U256;
use schemars::JsonSchema;
// pub struct FPDecimal(#[schemars(with = "String")] pub i128);

#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, JsonSchema)]
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

        let abs_x: u128 = x.abs() as u128;
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

// #[cfg(not(target_arch = "wasm32"))]
// impl convert::From<FPDecimal> for f32 {
//     fn from(x: FPDecimal) -> f32 {
//         f32::from_str(&x.to_string()).unwrap()
//     }
// }

impl FPDecimal {
    pub const MAX: FPDecimal = FPDecimal {
        num: U256::MAX,
        sign: 1,
    };
    pub const MIN: FPDecimal = FPDecimal {
        num: U256::MAX,
        sign: 0,
    };
    pub const DIGITS: usize = 18;
    pub const ONE: FPDecimal = FPDecimal {
        num: U256([1_000_000_000_000_000_000, 0, 0, 0]),
        sign: 1,
    };
    pub const MUL_PRECISION: FPDecimal = FPDecimal {
        num: U256([1_000_000_000, 0, 0, 0]),
        sign: 1,
    };
    pub const E_10: FPDecimal = FPDecimal {
        num: U256([1053370797511887454u64, 1194u64, 0, 0]),
        sign: 1,
    }; // e^10
    pub const E: FPDecimal = FPDecimal {
        num: U256([2718281828459045235, 0, 0, 0]),
        sign: 1,
    };
    pub const LN_10: FPDecimal = FPDecimal {
        num: U256([2302585092994045684, 0, 0, 0]),
        sign: 1,
    }; // ln(10)
    pub const LN_1_5: FPDecimal = FPDecimal {
        num: U256([405465108108164382, 0, 0, 0]),
        sign: 1,
    }; // ln(1.5)

    pub const fn one() -> FPDecimal {
        FPDecimal::ONE
    }

    pub const fn zero() -> FPDecimal {
        FPDecimal {
            num: U256([0, 0, 0, 0]),
            sign: 1,
        }
    }

    pub const fn max() -> FPDecimal {
        FPDecimal::MAX
    }

    pub const fn min() -> FPDecimal {
        FPDecimal::MIN
    }

    pub const fn e() -> FPDecimal {
        FPDecimal::E
    }

    pub fn _int(x: FPDecimal) -> FPDecimal {
        let x1 = x.num;
        let x1_1 = x1 / FPDecimal::ONE.num;
        let x_final = x1_1 * FPDecimal::ONE.num;
        FPDecimal {
            num: x_final,
            sign: x.sign,
        }
    }

    pub fn int(&self) -> FPDecimal {
        FPDecimal::_int(*self)
    }

    pub fn _sign(x: FPDecimal) -> i8 {
        x.sign
    }

    pub fn _fraction(x: FPDecimal) -> FPDecimal {
        let x1 = x.num;
        FPDecimal {
            num: x1 - FPDecimal::_int(x).num,
            sign: x.sign,
        }
    }

    pub fn fraction(&self) -> FPDecimal {
        FPDecimal::_fraction(*self)
    }
}

mod arithmetic;
mod display;
mod exp;
mod from_str;
mod hyper;
mod log;
mod serde; // cosmwasm serialization
