use crate::types::injective::exchange::v1beta1::ExchangeQuerier;

use cosmwasm_std::{Addr, Coin, CustomQuery, Deps, Empty, StdError, StdResult};
use cw_storage_plus::{Key, KeyDeserialize, Prefixer, PrimaryKey};
use ethereum_types::H160;
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{de::Error, ser::Error as SerError, Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{fmt, str::FromStr};
use subtle_encoding::bech32;

pub enum MarketType {
    Spot,
    Derivative,
}

#[derive(Serialize_repr, Deserialize_repr, Default, Clone, Debug, PartialEq, Eq, JsonSchema, Copy)]
#[repr(i32)]
pub enum AtomicMarketOrderAccessLevel {
    #[default]
    Nobody = 0,
    BeginBlockerSmartContractsOnly = 1,
    SmartContractsOnly = 2,
    Everyone = 3,
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, JsonSchema)]
pub struct MarketId(String);

impl fmt::Display for MarketId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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

    pub fn validate<Q: CustomQuery>(self, querier: &ExchangeQuerier<Q>, market_type: MarketType) -> StdResult<Self> {
        match market_type {
            MarketType::Spot => {
                let res = querier.spot_market(self.to_string())?;

                if res.market.is_none() {
                    return Err(StdError::generic_err("Market not found"));
                }
            }
            MarketType::Derivative => {
                let res = querier.derivative_market(self.to_string())?;

                if res.market.is_none() {
                    return Err(StdError::generic_err("Market not found"));
                }
            }
        };

        Ok(self)
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

impl<'a> From<&'a str> for MarketId {
    fn from(s: &'a str) -> Self {
        MarketId::new(s).unwrap()
    }
}

impl<'de> Deserialize<'de> for MarketId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let market_id = String::deserialize(deserializer)?;

        if !market_id.starts_with("0x") {
            let error_message = format!("Invalid prefix in deserialization: market_id must start with 0x, received {}", market_id);
            return Err(D::Error::custom(error_message));
        }

        if market_id.len() != 66 {
            let error_message = format!(
                "Invalid length in deserialization: market_id must be exactly 66 characters, received {}",
                market_id
            );
            return Err(D::Error::custom(error_message));
        }

        Ok(MarketId::unchecked(market_id))
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, JsonSchema)]
pub struct SubaccountId(String);

impl SubaccountId {
    pub fn new<S>(subaccount_id_s: S) -> Result<SubaccountId, StdError>
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

impl<'de> Deserialize<'de> for SubaccountId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let subaccount_id = String::deserialize(deserializer)?;

        if !subaccount_id.starts_with("0x") {
            let error_message = format!(
                "Invalid prefix in deserialization: subaccount_id must start with 0x, received {}",
                subaccount_id
            );
            return Err(D::Error::custom(error_message));
        }

        if subaccount_id.len() != 66 {
            let error_message = format!(
                "Invalid length in deserialization: subaccount_id must be exactly 66 characters, received {}",
                subaccount_id
            );
            return Err(D::Error::custom(error_message));
        }

        Ok(SubaccountId::unchecked(subaccount_id))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, JsonSchema)]
pub struct ShortSubaccountId(String);

const MAX_SHORT_SUBACCOUNT_NONCE: u16 = 999;

impl ShortSubaccountId {
    pub fn new<S>(id_s: S) -> Result<ShortSubaccountId, StdError>
    where
        S: Into<String>,
    {
        let id = id_s.into();
        let as_short = Self(id);
        as_short.validate()
    }

    pub fn must_new<S>(id_s: S) -> ShortSubaccountId
    where
        S: Into<String>,
    {
        let id = id_s.into();
        let as_short = Self(id);
        as_short.validate().unwrap()
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn unchecked<S>(id_s: S) -> Self
    where
        S: Into<String>,
    {
        Self(id_s.into())
    }

    pub fn validate(&self) -> StdResult<Self> {
        let as_decimal = match u32::from_str_radix(self.as_str(), 16) {
            Ok(dec) => Ok(dec),
            Err(_) => Err(StdError::generic_err(format!(
                "Invalid value: ShortSubaccountId was not a hexadecimal number: {}",
                &self.0
            ))),
        };

        match as_decimal?.to_string().parse::<u16>() {
            Ok(value) if value <= MAX_SHORT_SUBACCOUNT_NONCE => Ok(self.clone()),
            _ => Err(StdError::generic_err(format!(
                "Invalid value: ShortSubaccountId must be a number between 0-999, but {} was received",
                &self.0
            ))),
        }
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl<'de> Deserialize<'de> for ShortSubaccountId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = String::deserialize(deserializer)?;

        match id.parse::<u16>() {
            Ok(value) if value <= MAX_SHORT_SUBACCOUNT_NONCE => Ok(ShortSubaccountId::unchecked(format!("{:03x}", value))),
            _ => {
                let maybe_long = SubaccountId::unchecked(id);
                let maybe_short: ShortSubaccountId = ShortSubaccountId::from(maybe_long);
                maybe_short.validate().map_err(|e| D::Error::custom(e.to_string()))
            }
        }
    }
}

impl Serialize for ShortSubaccountId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.validate().map_err(|e| S::Error::custom(e.to_string()))?;

        let as_decimal = match u32::from_str_radix(self.0.as_str(), 16) {
            Ok(dec) => Ok(dec),
            Err(_) => Err(S::Error::custom(format!(
                "Invalid value: ShortSubaccountId was not a hexadecimal number: {}",
                &self.0
            ))),
        };

        serializer.serialize_str(&format!("{:0>3}", as_decimal?.to_string()))
    }
}

impl From<SubaccountId> for ShortSubaccountId {
    fn from(subaccount_id: SubaccountId) -> Self {
        let last_three_chars = &subaccount_id.as_str()[subaccount_id.as_str().len() - 3..];
        ShortSubaccountId::unchecked(last_three_chars.to_string())
    }
}

impl fmt::Display for SubaccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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

    const KEY_ELEMS: u16 = 42;

    #[inline(always)]
    fn from_vec(value: Vec<u8>) -> std::result::Result<SubaccountId, StdError> {
        Ok(SubaccountId::unchecked(String::from_vec(value)?))
    }

    fn from_slice(value: &[u8]) -> StdResult<Self::Output> {
        Self::from_vec(value.to_vec())
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

    const KEY_ELEMS: u16 = 42;

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, JsonSchema)]
pub struct Hash([u8; 32]);

impl Hash {
    pub fn new(bytes: [u8; 32]) -> Hash {
        Hash(bytes)
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    pub fn from_hex<T: AsRef<[u8]>>(s: T) -> StdResult<Hash> {
        let mut bytes = [0u8; 32];
        hex::decode_to_slice(s, &mut bytes).map_err(|e| StdError::generic_err(e.to_string()))?;
        Ok(Hash::new(bytes))
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", self.to_hex())
    }
}

pub fn get_default_subaccount_id_for_checked_address(addr: &Addr) -> SubaccountId {
    checked_address_to_subaccount_id(addr, 0)
}

pub fn checked_address_to_subaccount_id(addr: &Addr, nonce: u32) -> SubaccountId {
    let address_str = bech32_to_hex(addr);
    let hex_nonce = format!("{nonce:08x}");
    let nonce_str = left_pad_with_zeroes(hex_nonce, 24);

    SubaccountId::new(format!("{address_str}{nonce_str}")).expect("failed to create subaccount_id")
}

pub fn is_default_subaccount(subaccount_id: &SubaccountId) -> bool {
    let subaccount_id_str = subaccount_id.as_str();
    let hex_nonce = &subaccount_id_str[subaccount_id_str.len() - 24..subaccount_id_str.len()];

    hex_nonce == "000000000000000000000000"
}

fn left_pad_with_zeroes(mut input: String, length: usize) -> String {
    while input.len() < length {
        input = "0".to_string() + &input;
    }
    input
}

pub fn bech32_to_hex(addr: &Addr) -> String {
    let decoded_bytes = bech32::decode(addr.as_str()).unwrap().1;
    let decoded_h160 = H160::from_slice(&decoded_bytes);
    let decoded_string = format!("{decoded_h160:?}");
    decoded_string
}

pub fn addr_to_bech32(addr: String) -> String {
    let encoded_bytes = H160::from_str(&addr[2..addr.len()]).unwrap();
    bech32::encode("inj", encoded_bytes)
}

pub fn subaccount_id_to_ethereum_address(subaccount_id: &SubaccountId) -> String {
    let subaccount_id_str = subaccount_id.as_str();
    subaccount_id_str[0..subaccount_id_str.len() - 24].to_string()
}

pub fn subaccount_id_to_injective_address(subaccount_id: &SubaccountId, deps: &Deps) -> StdResult<Addr> {
    let ethereum_address = subaccount_id_to_ethereum_address(subaccount_id);

    deps.api.addr_validate(addr_to_bech32(ethereum_address).as_str())
}

pub fn subaccount_id_to_unchecked_injective_address(subaccount_id: &SubaccountId) -> String {
    let ethereum_address = subaccount_id_to_ethereum_address(subaccount_id);
    addr_to_bech32(ethereum_address)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_dependencies, Addr, StdError};
    use serde_test::{assert_de_tokens, assert_ser_tokens, Token};
    use std::panic::catch_unwind;

    use crate::traits::general::{
        bech32_to_hex, checked_address_to_subaccount_id, get_default_subaccount_id_for_checked_address, subaccount_id_to_injective_address, MarketId,
        ShortSubaccountId, SubaccountId,
    };

    #[test]
    fn bech32_to_hex_test() {
        let decoded_string = bech32_to_hex(&Addr::unchecked("inj1khsfhyavadcvzug67pufytaz2cq36ljkrsr0nv"));
        assert_eq!(decoded_string, "0xB5e09b93aCEb70C1711aF078922fA256011D7e56".to_lowercase());
    }

    #[test]
    fn checked_address_to_subaccount_id_test() {
        let subaccount_id = checked_address_to_subaccount_id(&Addr::unchecked("inj1khsfhyavadcvzug67pufytaz2cq36ljkrsr0nv"), 69);
        assert_eq!(
            subaccount_id.as_str(),
            "0xb5e09b93aceb70c1711af078922fa256011d7e56000000000000000000000045"
        );

        assert_eq!(
            get_default_subaccount_id_for_checked_address(&Addr::unchecked("inj1khsfhyavadcvzug67pufytaz2cq36ljkrsr0nv")).as_str(),
            "0xb5e09b93aceb70c1711af078922fa256011d7e56000000000000000000000000"
        );
    }

    #[test]
    fn subaccount_id_to_address_test() {
        let subaccount_id = "0xb5e09b93aceb70c1711af078922fa256011d7e56000000000000000000000000";
        let address = subaccount_id_to_injective_address(
            &SubaccountId::new(subaccount_id.to_string()).expect("Wong bech32 prefix"),
            &mock_dependencies().as_ref(),
        )
        .unwrap();

        assert_eq!(address.to_string(), "inj1khsfhyavadcvzug67pufytaz2cq36ljkrsr0nv".to_string());
    }

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

    #[test]
    fn subaccount_id_implements_display() {
        let subaccount_id = SubaccountId::unchecked("literal-string");
        assert_eq!(format!("{}", subaccount_id), "literal-string");
    }

    #[test]
    fn smallest_hex_short_subaccount_id_works() {
        let as_short = ShortSubaccountId::new("001");
        assert!(as_short.is_ok(), "001 should be a valid short subaccount id");
        assert_eq!(as_short.unwrap().as_str(), "001", "short subaccount id should be 001");
    }

    #[test]
    fn hex_short_subaccount_id_works() {
        let as_short = ShortSubaccountId::new("00a");
        assert!(as_short.is_ok(), "00a should be a valid short subaccount id");
        assert_eq!(as_short.unwrap().as_str(), "00a", "short subaccount id should be 00a");
    }

    #[test]
    fn must_new_hex_short_subaccount_id_works() {
        let as_short = ShortSubaccountId::must_new("00a");
        assert_eq!(as_short.as_str(), "00a", "short subaccount id should be 00a");
    }

    #[test]
    fn hex_short_subaccount_id_works_2() {
        let as_short = ShortSubaccountId::new("a");
        assert!(as_short.is_ok(), "a should be a valid short subaccount id");
        assert_eq!(as_short.unwrap().as_str(), "a", "short subaccount id should be a");
    }

    #[test]
    fn hex_short_subaccount_id_works_3() {
        let as_short = ShortSubaccountId::new("010");
        assert!(as_short.is_ok(), "010 should be a valid short subaccount id");
        assert_eq!(as_short.unwrap().as_str(), "010", "short subaccount id should be 010");
    }

    #[test]
    fn biggest_hex_short_subaccount_id_works() {
        let as_short = ShortSubaccountId::new("3E7");
        assert!(as_short.is_ok(), "3E7 should be a valid short subaccount id");
        assert_eq!(as_short.unwrap().as_str(), "3E7", "short subaccount id should be 3E7");
    }

    #[test]
    fn too_big_hex_short_subaccount_id_returns_err() {
        let as_short = ShortSubaccountId::new("3E8");
        assert!(as_short.is_err(), "3E8 should not be a valid short subaccount id");
    }

    #[test]
    #[should_panic]
    fn must_new_too_big_hex_short_subaccount_id_panics() {
        ShortSubaccountId::must_new("3E8");
    }

    #[test]
    fn random_string_short_subaccount_id_returns_err() {
        let as_short = ShortSubaccountId::new("1ag");
        assert!(as_short.is_err(), "1ag should not be a valid short subaccount id");
    }

    #[test]
    fn smallest_hex_short_subaccount_id_works_unchecked() {
        let as_short = ShortSubaccountId::unchecked("001");
        assert_eq!(as_short.as_str(), "001", "unchecked short subaccount id should be 001");
    }

    #[test]
    fn hex_short_subaccount_id_works_unchecked() {
        let as_short = ShortSubaccountId::unchecked("00a");
        assert_eq!(as_short.as_str(), "00a", "unchecked short subaccount id should be 00a");
    }

    #[test]
    fn hex_short_subaccount_id_works_2_unchecked() {
        let as_short = ShortSubaccountId::unchecked("a");
        assert_eq!(as_short.as_str(), "a", "unchecked short subaccount id should be a");
    }

    #[test]
    fn hex_short_subaccount_id_works_3_unchecked() {
        let as_short = ShortSubaccountId::unchecked("010");
        assert_eq!(as_short.as_str(), "010", "unchecked short subaccount id should be 010");
    }

    #[test]
    fn biggest_hex_short_subaccount_id_works_unchecked() {
        let as_short = ShortSubaccountId::unchecked("3E7");
        assert_eq!(as_short.as_str(), "3E7", "unchecked short subaccount id should be 3E7");
    }

    #[test]
    fn too_big_hex_short_subaccount_id_returns_unchecked_input() {
        let as_short = ShortSubaccountId::unchecked("3E8");
        assert_eq!(as_short.as_str(), "3E8", "unchecked short subaccount id should be 3E8");
    }

    #[test]
    fn random_string_short_subaccount_id_returns_unchecked_input() {
        let as_short = ShortSubaccountId::unchecked("1ag");
        assert_eq!(as_short.as_str(), "1ag", "unchecked short subaccount id should be 1ag");
    }

    #[test]
    fn smallest_hex_short_subaccount_is_correctly_serialized() {
        let short_subaccount = ShortSubaccountId::unchecked("001");
        assert_ser_tokens(&short_subaccount, &[Token::Str("001")]);
    }

    #[test]
    fn hex_short_subaccount_is_correctly_serialized_as_decimal() {
        let short_subaccount = ShortSubaccountId::unchecked("00a");
        assert_ser_tokens(&short_subaccount, &[Token::Str("010")]);
    }

    #[test]
    fn hex_short_subaccount_is_correctly_serialized_as_decimal_2() {
        let short_subaccount = ShortSubaccountId::unchecked("a");
        assert_ser_tokens(&short_subaccount, &[Token::Str("010")]);
    }

    #[test]
    fn hex_short_subaccount_is_correctly_serialized_as_decimal_3() {
        let short_subaccount = ShortSubaccountId::unchecked("010");
        assert_ser_tokens(&short_subaccount, &[Token::Str("016")]);
    }

    #[test]
    fn biggest_hex_short_subaccount_is_correctly_serialized_as_decimal() {
        let short_subaccount = ShortSubaccountId::unchecked("3E7");
        assert_ser_tokens(&short_subaccount, &[Token::Str("999")]);
    }

    #[test]
    #[should_panic]
    fn too_big_hex_short_subaccount_returns_error_when_serialized() {
        let short_subaccount = ShortSubaccountId::unchecked("3E8");
        assert_ser_tokens(&short_subaccount, &[Token::Str("1000")]);
    }

    #[test]
    fn random_string_short_subaccount_returns_error_when_serialized() {
        let short_subaccount = ShortSubaccountId::unchecked("ah7");
        let result = catch_unwind(|| assert_ser_tokens(&short_subaccount, &[Token::Str("1000")]));
        assert!(result.is_err(), "serializing invalid short subaccount id should fail");
    }

    #[test]
    fn short_subaccount_id_can_be_deserialized_from_valid_short_decimal() {
        let short_subaccount = ShortSubaccountId::unchecked("001");
        assert_de_tokens(&short_subaccount, &[Token::Str("1")]);
    }

    #[test]
    fn short_subaccount_id_can_be_deserialized_from_valid_long_decimal() {
        let short_subaccount = ShortSubaccountId::unchecked("00a");
        assert_de_tokens(&short_subaccount, &[Token::Str("010")]);
    }

    #[test]
    fn short_subaccount_id_can_be_deserialized_from_valid_long_subaccount_id() {
        let short_subaccount = ShortSubaccountId::unchecked("001");
        assert_de_tokens(
            &short_subaccount,
            &[Token::Str("0x17dcdb32a51ee1c43c3377349dba7f56bdf48e35000000000000000000000001")],
        );
    }

    #[test]
    fn short_subaccount_id_can_be_deserialized_from_valid_hex_long_subaccount_id() {
        let short_subaccount = ShortSubaccountId::unchecked("00a");
        assert_de_tokens(
            &short_subaccount,
            &[Token::Str("0x17dcdb32a51ee1c43c3377349dba7f56bdf48e3500000000000000000000000a")],
        );
    }

    #[test]
    fn short_subaccount_id_can_be_deserialized_from_valid_hex_highest_long_subaccount_id() {
        let short_subaccount = ShortSubaccountId::unchecked("3e7");
        assert_de_tokens(
            &short_subaccount,
            &[Token::Str("0x17dcdb32a51ee1c43c3377349dba7f56bdf48e350000000000000000000003E7")],
        );
    }

    #[test]
    fn short_subaccount_id_can_be_deserialized_from_valid_highest_hex_short_subaccount_id() {
        let short_subaccount = ShortSubaccountId::unchecked("3e7");
        assert_de_tokens(&short_subaccount, &[Token::Str("3e7")]);
    }

    #[test]
    fn short_subaccount_id_can_be_deserialized_from_valid_highest_decimal_short_subaccount_id() {
        let short_subaccount = ShortSubaccountId::unchecked("3e7");
        assert_de_tokens(&short_subaccount, &[Token::Str("999")]);
    }

    #[test]
    #[should_panic]
    fn short_subaccount_id_cannot_be_deserialized_from_too_high_hex_long_subaccount_id() {
        let short_subaccount = ShortSubaccountId::unchecked("1000");
        assert_de_tokens(
            &short_subaccount,
            &[Token::Str("0x17dcdb32a51ee1c43c3377349dba7f56bdf48e3500000000000000000000003E8")],
        );
    }

    #[test]
    #[should_panic]
    fn short_subaccount_id_cannot_be_deserialized_from_too_high_hex_short_subaccount_id() {
        let short_subaccount = ShortSubaccountId::unchecked("3E8");
        assert_de_tokens(&short_subaccount, &[Token::Str("3E8")]);
    }
}
