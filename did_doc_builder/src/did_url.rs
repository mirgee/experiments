use serde::{de, Deserialize, Deserializer, Serialize};

use crate::error::DIDDocumentBuilderError;

#[derive(Serialize, Clone, Debug, PartialEq, Default)]
pub struct DidUrl(String);

impl DidUrl {
    pub fn new(did_url: String) -> Result<Self, DIDDocumentBuilderError> {
        if is_valid_did_url(&did_url) {
            Ok(Self(did_url))
        } else {
            Err(DIDDocumentBuilderError::InvalidInput(format!(
                "Invalid DID URL: {}",
                did_url
            )))
        }
    }
}

impl<'de> Deserialize<'de> for DidUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if is_valid_did_url(&s) {
            Ok(Self(s))
        } else {
            Err(de::Error::invalid_value(
                de::Unexpected::Str(&s),
                &"a valid DID URL",
            ))
        }
    }
}

fn is_valid_did_url(did: &str) -> bool {
    did.starts_with("did:")
}
