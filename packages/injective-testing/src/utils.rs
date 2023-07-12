use cosmwasm_std::{coin, Coin};
use injective_math::{scale::Scaled, FPDecimal};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[repr(i32)]
pub enum Decimals {
    Eighteen = 18,
    Six = 6,
}

impl Decimals {
    pub fn get_decimals(&self) -> i32 {
        match self {
            Decimals::Eighteen => 18,
            Decimals::Six => 6,
        }
    }
}

pub fn proto_to_dec(val: &str) -> FPDecimal {
    FPDecimal::must_from_str(val).scaled(-18)
}

pub fn dec_to_human(val: FPDecimal, exponent: i32) -> String {
    val.scaled(-exponent).to_string()
}

pub fn dec_to_proto(val: FPDecimal) -> String {
    val.scaled(18).to_string()
}

pub fn human_to_dec(raw_number: &str, decimals: Decimals) -> FPDecimal {
    FPDecimal::must_from_str(&raw_number.replace('_', "")).scaled(decimals.get_decimals())
}

pub fn human_to_i64(raw_number: &str, exponent: i32) -> i64 {
    let scaled_amount = FPDecimal::must_from_str(&raw_number.replace('_', "")).scaled(exponent);
    let as_int: i64 = scaled_amount.to_string().parse().unwrap();
    as_int
}

pub fn human_to_proto(raw_number: &str, decimals: i32) -> String {
    FPDecimal::must_from_str(&raw_number.replace('_', "")).scaled(18 + decimals).to_string()
}

pub fn str_coin(human_amount: &str, denom: &str, decimals: Decimals) -> Coin {
    let scaled_amount = human_to_dec(human_amount, decimals);
    let as_int: u128 = scaled_amount.into();
    coin(as_int, denom)
}
