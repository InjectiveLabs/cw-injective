use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;
use injective_cosmwasm::{MarketId, SubaccountId};

pub const ORDER_CALL_CACHE: Item<Vec<CacheOrderInfo>> = Item::new("order_call_cache");

#[cw_serde]
pub struct CacheOrderInfo {
    pub subaccount: SubaccountId,
    pub market_id: MarketId,
}
