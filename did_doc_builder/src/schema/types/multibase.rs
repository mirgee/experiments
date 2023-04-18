use std::str::FromStr;

use multibase::{decode, Base};
use serde::{Deserialize, Serialize};

use crate::error::DIDDocumentBuilderError;

#[derive(Clone, Debug, PartialEq)]
pub struct Multibase {
    base: Base,
    bytes: Vec<u8>,
}

impl Multibase {
    pub fn new(multibase: String) -> Result<Self, DIDDocumentBuilderError> {
        let (base, bytes) = decode(multibase).map_err(|err| {
            DIDDocumentBuilderError::InvalidInput(format!("Invalid multibase key: {}", err))
        })?;
        Ok(Self { base, bytes })
    }
}

impl Serialize for Multibase {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.base.encode(&self.bytes))
    }
}

impl<'de> Deserialize<'de> for Multibase {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::new(s).map_err(serde::de::Error::custom)
    }
}

impl FromStr for Multibase {
    type Err = DIDDocumentBuilderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}
