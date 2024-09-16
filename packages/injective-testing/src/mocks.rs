use crate::utils::human_to_dec;

use injective_math::FPDecimal;
use injective_std::{
    traits::general::MarketId,
    types::injective::exchange::v1beta1::{DerivativeMarket, MarketStatus, QuerySpotMidPriceAndTobResponse, SpotMarket},
};

pub const MOCKED_MARKET_ID: &str = "0x01edfab47f124748dc89998eb33144af734484ba07099014594321729a0ca16b";
pub const MOCKED_SUBACCOUNT_ID: &str = "0x427aee334987c52fa7b567b2662bdbb68614e48c000000000000000000000001";
pub const MOCKED_FEE_RECIPIENT: &str = "0x01edfab47f124748dc89998eb33144af734484ba07099014594321729a0ca16b";

pub const MOCK_EXCHANGE_DECIMALS: i32 = 18i32;
pub const MOCK_BASE_DECIMALS: i32 = 18i32;
pub const MOCK_ATOM_DECIMALS: i32 = 8i32;
pub const MOCK_QUOTE_DECIMALS: i32 = 6i32;

pub const MOCK_ATOM_DENOM: &str = "atom";
pub const MOCK_BASE_DENOM: &str = "inj";
pub const MOCK_QUOTE_DENOM: &str = "usdt";
pub const MOCK_USDC_DENOM: &str = "usdc";

// Mock INJ Market
pub fn mock_spot_market(market_id: &str) -> SpotMarket {
    SpotMarket {
        ticker: String::from("INJ:USDT"),
        base_denom: String::from("inj"),
        quote_denom: String::from("usdt"),
        market_id: MarketId::unchecked(market_id).to_string(),
        maker_fee_rate: FPDecimal::ZERO.to_string(),
        taker_fee_rate: FPDecimal::ZERO.to_string(),
        status: MarketStatus::Active as i32,
        min_notional: FPDecimal::ZERO.to_string(),
        min_price_tick_size: FPDecimal::must_from_str("0.000000000000001000").to_string(),
        min_quantity_tick_size: FPDecimal::must_from_str("10000000000000.0").to_string(), // 0.00001 @.to_string() 18dp
        relayer_fee_share_rate: FPDecimal::must_from_str("0.4").to_string(),
        admin: String::from(""),
        admin_permissions: 0u32,
    }
}

// Mock INJ Market
pub fn mock_derivative_market(market_id: &str) -> DerivativeMarket {
    DerivativeMarket {
        ticker: String::from("INJ:USDT"),
        oracle_base: String::from("inj"),
        oracle_quote: String::from("usdt"),
        oracle_type: 0i32,
        // oracle_type: OracleType::PriceFeed,
        oracle_scale_factor: 0u32,
        quote_denom: String::from("usdt"),
        market_id: MarketId::unchecked(market_id).to_string(),
        initial_margin_ratio: FPDecimal::must_from_str("0.195").to_string(),
        maintenance_margin_ratio: FPDecimal::must_from_str("0.05").to_string(),
        maker_fee_rate: FPDecimal::ZERO.to_string(),
        taker_fee_rate: FPDecimal::ZERO.to_string(),
        isPerpetual: true,
        status: MarketStatus::Active as i32,
        min_price_tick_size: FPDecimal::must_from_str("1000.0").to_string(), // 0.001.to_string()
        min_quantity_tick_size: FPDecimal::must_from_str("0.001").to_string(), // 0.001.to_string()
        min_notional: FPDecimal::ZERO.to_string(),
        relayer_fee_share_rate: FPDecimal::must_from_str("0.4").to_string(),
        admin: String::from(""),
        admin_permissions: 0u32,
    }
}

pub fn mock_mid_price_tob() -> QuerySpotMidPriceAndTobResponse {
    QuerySpotMidPriceAndTobResponse {
        mid_price: human_to_dec("10.0", MOCK_QUOTE_DECIMALS - MOCK_BASE_DECIMALS).to_string(),
        best_buy_price: human_to_dec("9.95", MOCK_QUOTE_DECIMALS - MOCK_BASE_DECIMALS).to_string(),
        best_sell_price: human_to_dec("10.05", MOCK_QUOTE_DECIMALS - MOCK_BASE_DECIMALS).to_string(),
    }
}
