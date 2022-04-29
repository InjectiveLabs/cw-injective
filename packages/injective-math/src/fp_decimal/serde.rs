use crate::fp_decimal::FPDecimal;
use serde::{de, ser, Deserialize, Deserializer, Serialize};
use std::fmt;
use std::str::FromStr;

#[allow(clippy::upper_case_acronyms)]
struct FPDecimalVisitor;

impl<'de> de::Visitor<'de> for FPDecimalVisitor {
    type Value = FPDecimal;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("FPDecimal (string-encoded)")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match FPDecimal::from_str(v) {
            Ok(d) => Ok(d),
            Err(e) => Err(E::custom(format!("Error parsing FPDecimal '{}': {}", v, e))),
        }
    }
}

/// Serializes as a decimal string
impl Serialize for FPDecimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&format!("{}", &self))
    }
}

/// Deserializes as a base64 string
impl<'de> Deserialize<'de> for FPDecimal {
    fn deserialize<D>(deserializer: D) -> Result<FPDecimal, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FPDecimalVisitor)
    }
}
