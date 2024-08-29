use cosmwasm_std::{coin, Coin};
use injective_math::{scale::Scaled, FPDecimal};

pub fn assert_execute_error(message: &str) -> String {
    format!(
        "execute error: failed to execute message; message index: 0: {}: execute wasm contract failed",
        message
    )
}

pub fn assert_instantiate_error(message: &str) -> String {
    format!(
        "execute error: failed to execute message; message index: 0: {}: instantiate wasm contract failed",
        message
    )
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

pub fn human_to_dec(raw_number: &str, decimals: i32) -> FPDecimal {
    FPDecimal::must_from_str(&raw_number.replace('_', "")).scaled(decimals)
}

pub fn human_to_dec_vector(values: Vec<&str>, decimals: i32) -> Vec<FPDecimal> {
    values.iter().map(|v| human_to_dec(v, decimals)).collect::<Vec<FPDecimal>>()
}

pub fn human_to_i64(raw_number: &str, exponent: i32) -> i64 {
    let scaled_amount = FPDecimal::must_from_str(&raw_number.replace('_', "")).scaled(exponent);
    let as_int: i64 = scaled_amount.to_string().parse().unwrap();
    as_int
}

pub fn human_to_proto(raw_number: &str, decimals: i32) -> String {
    FPDecimal::must_from_str(&raw_number.replace('_', "")).scaled(18 + decimals).to_string()
}

pub fn str_coin(human_amount: &str, denom: &str, decimals: i32) -> Coin {
    let scaled_amount = human_to_dec(human_amount, decimals);
    let as_int: u128 = scaled_amount.into();
    coin(as_int, denom)
}

pub fn scale_price_quantity_spot_market(price: &str, quantity: &str, base_decimals: &i32, quote_decimals: &i32) -> (String, String) {
    let price_dec = FPDecimal::must_from_str(price.replace('_', "").as_str());
    let quantity_dec = FPDecimal::must_from_str(quantity.replace('_', "").as_str());

    let scaled_price = price_dec.scaled(quote_decimals - base_decimals);
    let scaled_quantity = quantity_dec.scaled(*base_decimals);

    (dec_to_proto(scaled_price), dec_to_proto(scaled_quantity))
}

pub fn scale_price_quantity_perp_market(price: &str, quantity: &str, margin_ratio: &str, quote_decimals: &i32) -> (String, String, String) {
    let price_dec = FPDecimal::must_from_str(price.replace('_', "").as_str());
    let quantity_dec = FPDecimal::must_from_str(quantity.replace('_', "").as_str());
    let margin_ratio_dec = FPDecimal::must_from_str(margin_ratio.replace('_', "").as_str());

    let scaled_price = price_dec.scaled(*quote_decimals);
    let scaled_quantity = quantity_dec;

    let scaled_margin = (price_dec * quantity_dec * margin_ratio_dec).scaled(*quote_decimals);

    (dec_to_proto(scaled_price), dec_to_proto(scaled_quantity), dec_to_proto(scaled_margin))
}
