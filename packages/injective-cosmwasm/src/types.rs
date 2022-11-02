use cosmwasm_std::{StdError, StdResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, JsonSchema)]
pub struct MarketId(String);

impl MarketId {
    pub fn new<S>(market_id_s: S) -> StdResult<Self>
    where
        S: Into<String>,
    {
        let market_id = market_id_s.into();

        if !market_id.starts_with("0x") {
            return Err(StdError::generic_err("Invalid prefix: market_id must start with 0x"));
        }

        if market_id.len() != 66 {
            return Err(StdError::generic_err("Invalid length: market_id must be exactly 66 characters"));
        }

        Ok(Self(market_id.to_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for MarketId {
    fn from(market_id: String) -> Self {
        MarketId::new(market_id).unwrap()
    }
}

impl From<&str> for MarketId {
    fn from(market_id: &str) -> Self {
        MarketId::new(market_id).unwrap()
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for MarketId {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, JsonSchema)]
pub struct SubaccountId(String);

impl SubaccountId {
    pub fn new<S>(subaccount_id_s: S) -> StdResult<Self>
    where
        S: Into<String>,
    {
        let subaccount_id = subaccount_id_s.into();

        if !subaccount_id.starts_with("0x") {
            return Err(StdError::generic_err("Invalid prefix: subaccount_id must start with 0x"));
        }

        if subaccount_id.len() != 66 {
            return Err(StdError::generic_err("Invalid length: subaccount_id must be exactly 66 characters"));
        }

        Ok(Self(subaccount_id.to_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for SubaccountId {
    fn from(subaccount_id: String) -> Self {
        SubaccountId::new(subaccount_id).unwrap()
    }
}

impl From<&str> for SubaccountId {
    fn from(subaccount_id: &str) -> Self {
        SubaccountId::new(subaccount_id).unwrap()
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for SubaccountId {
    fn into(self) -> String {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{MarketId, SubaccountId};

    #[test]
    fn subaccount_id_to_lowercase() {
        let subaccount_id: SubaccountId = "0xB5e09b93aCEb70C1711aF078922fA256011D7e56000000000000000000000045".into();
        let subaccount_id_str: String = subaccount_id.into();
        assert_eq!(
            subaccount_id_str,
            "0xB5e09b93aCEb70C1711aF078922fA256011D7e56000000000000000000000045".to_lowercase()
        );
    }

    #[test]
    fn market_id_to_lowercase() {
        let market_id: MarketId = "0x01EDFAB47F124748DC89998EB33144AF734484BA07099014594321729A0CA16B".into();
        let market_id_str: String = market_id.into();
        assert_eq!(
            market_id_str,
            "0x01edfab47f124748dc89998eb33144af734484ba07099014594321729a0ca16b".to_lowercase()
        );
    }
}
