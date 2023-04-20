use std::{ops::Deref, str::FromStr};

use serde::{de, Deserialize, Deserializer, Serialize};

use crate::error::DIDDocumentBuilderError;

#[derive(Serialize, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Did(String);

impl Did {
    pub fn new(did: String) -> Result<Self, DIDDocumentBuilderError> {
        if is_valid_did(&did) {
            Ok(Self(did))
        } else {
            Err(DIDDocumentBuilderError::InvalidInput(format!(
                "Invalid DID: {}",
                did
            )))
        }
    }
}

impl<'de> Deserialize<'de> for Did {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if is_valid_did(&s) {
            Ok(Self(s))
        } else {
            Err(de::Error::invalid_value(
                de::Unexpected::Str(&s),
                &"a valid DID",
            ))
        }
    }
}

impl FromStr for Did {
    type Err = DIDDocumentBuilderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

impl ToString for Did {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl Deref for Did {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn is_valid_did(did: &str) -> bool {
    // TODO: This is just dummy "validation"
    did.starts_with("did:")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_did_new_valid() {
        let did = Did::new("did:example:123456789abcdefghi".to_string());
        assert!(did.is_ok());
    }

    #[test]
    fn test_did_new_invalid() {
        let did = Did::new("invalid:example:123456789abcdefghi".to_string());
        assert!(did.is_err());
    }

    #[test]
    fn test_did_deserialize_valid() {
        let did_str = "\"did:example:123456789abcdefghi\"";
        let did: Result<Did, _> = serde_json::from_str(did_str);
        assert!(did.is_ok());
    }

    #[test]
    fn test_did_deserialize_invalid() {
        let did_str = "\"invalid:example:123456789abcdefghi\"";
        let did: Result<Did, _> = serde_json::from_str(did_str);
        assert!(did.is_err());
    }

    #[test]
    fn test_did_from_str_valid() {
        let did = Did::from_str("did:example:123456789abcdefghi");
        assert!(did.is_ok());
    }

    #[test]
    fn test_did_from_str_invalid() {
        let did = Did::from_str("invalid:example:123456789abcdefghi");
        assert!(did.is_err());
    }

    #[test]
    fn test_did_to_string() {
        let did = Did::new("did:example:123456789abcdefghi".to_string()).unwrap();
        assert_eq!(did.to_string(), "did:example:123456789abcdefghi");
    }

    #[test]
    fn test_is_valid_did() {
        assert!(is_valid_did("did:example:123456789abcdefghi"));
        assert!(!is_valid_did("invalid:example:123456789abcdefghi"));
    }
}
