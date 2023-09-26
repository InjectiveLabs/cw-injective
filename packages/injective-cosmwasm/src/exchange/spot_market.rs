use crate::exchange::types::MarketId;
use cosmwasm_std::StdResult;
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tiny_keccak::Keccak;

use super::market::{GenericMarket, MarketStatus};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SpotMarket {
    pub ticker: String,
    pub base_denom: String,
    pub quote_denom: String,
    pub maker_fee_rate: FPDecimal,
    pub taker_fee_rate: FPDecimal,
    pub relayer_fee_share_rate: FPDecimal,
    pub market_id: MarketId,
    #[serde(default)]
    pub status: MarketStatus,
    pub min_price_tick_size: FPDecimal,
    pub min_quantity_tick_size: FPDecimal,
}

impl GenericMarket for SpotMarket {
    fn get_ticker(&self) -> &str {
        &self.ticker
    }

    fn get_quote_denom(&self) -> &str {
        &self.quote_denom
    }

    fn get_maker_fee_rate(&self) -> FPDecimal {
        self.maker_fee_rate
    }

    fn get_taker_fee_rate(&self) -> FPDecimal {
        self.taker_fee_rate
    }

    fn get_market_id(&self) -> &MarketId {
        &self.market_id
    }

    fn get_status(&self) -> MarketStatus {
        self.status
    }

    fn get_min_price_tick_size(&self) -> FPDecimal {
        self.min_price_tick_size
    }

    fn min_quantity_tick_size(&self) -> FPDecimal {
        self.min_quantity_tick_size
    }
}

pub fn calculate_spot_market_id(base_denom: String, quote_denom: String) -> StdResult<MarketId> {
    let mut hasher = Keccak::new_keccak256();
    hasher.update((base_denom + &quote_denom).as_bytes());
    let mut res = [0u8; 32];
    hasher.finalize(&mut res);

    MarketId::new(format!("0x{}", hex::encode(res)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_spot_market_id() {
        let base = "inj".to_string();
        let quote = "peggy0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string();
        let result = calculate_spot_market_id(base, quote).unwrap();
        let expected_result = "0xa508cb32923323679f29a032c70342c147c17d0145625922b0ef22e955c844c0";

        assert_eq!(
            result.as_str(),
            expected_result,
            "calculate_spot_market_id did not produce the expected hash"
        );
    }
}
