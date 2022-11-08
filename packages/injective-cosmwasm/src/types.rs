use cosmwasm_std::{StdError, StdResult};
use cw_storage_plus::{Key, KeyDeserialize, Prefixer, PrimaryKey};
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

    pub fn unchecked<S>(market_id_s: S) -> Self
    where
        S: Into<String>,
    {
        Self(market_id_s.into().to_lowercase())
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

    pub fn unchecked<S>(subaccount_id_s: S) -> Self
    where
        S: Into<String>,
    {
        Self(subaccount_id_s.into().to_lowercase())
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for SubaccountId {
    fn into(self) -> String {
        self.0
    }
}

impl KeyDeserialize for SubaccountId {
    type Output = SubaccountId;

    #[inline(always)]
    fn from_vec(value: Vec<u8>) -> StdResult<Self::Output> {
        Ok(SubaccountId::unchecked(String::from_vec(value)?))
    }
}

impl<'a> PrimaryKey<'a> for SubaccountId {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = Self;
    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key> {
        // this is simple, we don't add more prefixes
        vec![Key::Ref(self.as_bytes())]
    }
}

impl<'a> Prefixer<'a> for SubaccountId {
    fn prefix(&self) -> Vec<Key> {
        vec![Key::Ref(self.as_bytes())]
    }
}

impl KeyDeserialize for &SubaccountId {
    type Output = SubaccountId;

    #[inline(always)]
    fn from_vec(value: Vec<u8>) -> StdResult<Self::Output> {
        Self::Output::from_vec(value)
    }
}

impl AsRef<str> for SubaccountId {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> PrimaryKey<'a> for &'a SubaccountId {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = Self;
    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key> {
        // this is simple, we don't add more prefixes
        vec![Key::Ref(self.as_ref().as_bytes())]
    }
}

impl<'a> Prefixer<'a> for &'a SubaccountId {
    fn prefix(&self) -> Vec<Key> {
        vec![Key::Ref(self.as_bytes())]
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, JsonSchema)]
pub struct TFDenom(String); // format!("factory/{}/lp{}", master_address, vault_address)

impl TFDenom {
    pub fn new<S>(denom: S) -> StdResult<Self>
    where
        S: Into<String>,
    {
        let denom = denom.into();

        if !denom.starts_with("factory/") {
            return Err(StdError::generic_err(
                "Invalid prefix: token factory denoms must start with \"factory/...\"",
            ));
        }

        if denom.len() != "factory/".len() + 33 + "/lp".len() + 33 {
            return Err(StdError::generic_err("Invalid length: token factorydenom must be exactly 77 characters"));
        }

        Ok(Self(denom.to_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn unchecked<S>(denom_s: S) -> Self
    where
        S: Into<String>,
    {
        Self(denom_s.into().to_lowercase())
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for TFDenom {
    fn into(self) -> String {
        self.0
    }
}

impl KeyDeserialize for TFDenom {
    type Output = TFDenom;

    #[inline(always)]
    fn from_vec(value: Vec<u8>) -> StdResult<Self::Output> {
        Ok(TFDenom::unchecked(String::from_vec(value)?))
    }
}

impl<'a> PrimaryKey<'a> for TFDenom {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = Self;
    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key> {
        // this is simple, we don't add more prefixes
        vec![Key::Ref(self.as_bytes())]
    }
}

impl<'a> Prefixer<'a> for TFDenom {
    fn prefix(&self) -> Vec<Key> {
        vec![Key::Ref(self.as_bytes())]
    }
}

impl KeyDeserialize for &TFDenom {
    type Output = TFDenom;

    #[inline(always)]
    fn from_vec(value: Vec<u8>) -> StdResult<Self::Output> {
        Self::Output::from_vec(value)
    }
}

impl AsRef<str> for TFDenom {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> PrimaryKey<'a> for &'a TFDenom {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = Self;
    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key> {
        // this is simple, we don't add more prefixes
        vec![Key::Ref(self.as_ref().as_bytes())]
    }
}

impl<'a> Prefixer<'a> for &'a TFDenom {
    fn prefix(&self) -> Vec<Key> {
        vec![Key::Ref(self.as_bytes())]
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::StdError;

    use crate::{MarketId, SubaccountId};

    #[test]
    fn unchecked_subaccount_id_to_lowercase() {
        let subaccount_id = SubaccountId::unchecked("0xB5e09b93aCEb70C1711aF078922fA256011D7e56000000000000000000000045");
        let subaccount_id_str: String = subaccount_id.into();
        assert_eq!(
            subaccount_id_str,
            "0xB5e09b93aCEb70C1711aF078922fA256011D7e56000000000000000000000045".to_lowercase()
        );
    }

    #[test]
    fn unchecked_market_id_to_lowercase() {
        let market_id = MarketId::unchecked("0x01EDFAB47F124748DC89998EB33144AF734484BA07099014594321729A0CA16B");
        let market_id_str: String = market_id.into();
        assert_eq!(
            market_id_str,
            "0x01edfab47f124748dc89998eb33144af734484ba07099014594321729a0ca16b".to_lowercase()
        );
    }

    #[test]
    fn checked_subaccount_id_to_lowercase() {
        let subaccount_id = SubaccountId::new("0xB5e09b93aCEb70C1711aF078922fA256011D7e56000000000000000000000045").unwrap();
        let subaccount_id_str: String = subaccount_id.into();
        assert_eq!(
            subaccount_id_str,
            "0xB5e09b93aCEb70C1711aF078922fA256011D7e56000000000000000000000045".to_lowercase()
        );
    }

    #[test]
    fn checked_market_id_to_lowercase() {
        let market_id = MarketId::new("0x01EDFAB47F124748DC89998EB33144AF734484BA07099014594321729A0CA16B").unwrap();
        let market_id_str: String = market_id.into();
        assert_eq!(
            market_id_str,
            "0x01edfab47f124748dc89998eb33144af734484ba07099014594321729a0ca16b".to_lowercase()
        );
    }

    #[test]
    fn subaccount_id_checks() {
        let wrong_prefix_err = SubaccountId::new("00B5e09b93aCEb70C1711aF078922fA256011D7e56000000000000000000000045").unwrap_err();
        assert_eq!(
            wrong_prefix_err,
            StdError::generic_err("Invalid prefix: subaccount_id must start with 0x")
        );

        let wrong_length_err = SubaccountId::new("0xB5e09b93aCEb70C1711aF078922fA256011D7e5600000000000000000000004").unwrap_err();
        assert_eq!(
            wrong_length_err,
            StdError::generic_err("Invalid length: subaccount_id must be exactly 66 characters")
        );

        let wrong_length_err = SubaccountId::new("0xB5e09b93aCEb70C1711aF078922fA256011D7e560000000000000000000000451").unwrap_err();
        assert_eq!(
            wrong_length_err,
            StdError::generic_err("Invalid length: subaccount_id must be exactly 66 characters")
        );
    }

    #[test]
    fn market_id_checks() {
        let wrong_prefix_err = MarketId::new("0001EDFAB47F124748DC89998EB33144AF734484BA07099014594321729A0CA16B").unwrap_err();
        assert_eq!(wrong_prefix_err, StdError::generic_err("Invalid prefix: market_id must start with 0x"));

        let wrong_length_err = MarketId::new("0x01EDFAB47F124748DC89998EB33144AF734484BA07099014594321729A0CA16").unwrap_err();
        assert_eq!(
            wrong_length_err,
            StdError::generic_err("Invalid length: market_id must be exactly 66 characters")
        );

        let wrong_length_err = MarketId::new("0x01EDFAB47F124748DC89998EB33144AF734484BA07099014594321729A0CA16B2").unwrap_err();
        assert_eq!(
            wrong_length_err,
            StdError::generic_err("Invalid length: market_id must be exactly 66 characters")
        );
    }

    #[test]
    fn subaccount_id_unchecked_works() {
        let a = SubaccountId::unchecked("123");
        let aa = SubaccountId::unchecked(String::from("123"));
        let b = SubaccountId::unchecked("be");
        assert_eq!(a, aa);
        assert_ne!(a, b);
    }

    #[test]
    fn subaccount_id_as_str_works() {
        let subaccount_id = SubaccountId::unchecked("amazing-id");
        assert_eq!(subaccount_id.as_str(), "amazing-id");
    }

    #[test]
    fn subaccount_id_as_bytes_works() {
        let subaccount_id = SubaccountId::unchecked("literal-string");
        assert_eq!(
            subaccount_id.as_bytes(),
            [108, 105, 116, 101, 114, 97, 108, 45, 115, 116, 114, 105, 110, 103]
        );
    }

    #[test]
    fn subaccount_id_implements_as_ref_for_str() {
        let subaccount_id = SubaccountId::unchecked("literal-string");
        assert_eq!(subaccount_id.as_ref(), "literal-string");
    }
}
