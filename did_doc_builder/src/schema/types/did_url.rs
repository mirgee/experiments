use std::str::FromStr;

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
    // TODO: This is just dummy "validation"
    did.starts_with("did:")
        || did.starts_with("/")
        || did.starts_with("#")
        || did.starts_with("?")
        || did.starts_with(";")
}

impl FromStr for DidUrl {
    type Err = DIDDocumentBuilderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

impl ToString for DidUrl {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_did_url_new_valid() {
        let did_url = DidUrl::new("did:example:123456789abcdefghi#key-1".to_string());
        assert!(did_url.is_ok());
    }

    #[test]
    fn test_did_url_new_invalid() {
        let did_url = DidUrl::new("invalid:example:123456789abcdefghi#key-1".to_string());
        assert!(did_url.is_err());
    }

    #[test]
    fn test_did_url_deserialize_valid() {
        let did_url_str = "\"did:example:123456789abcdefghi#key-1\"";
        let did_url: Result<DidUrl, _> = serde_json::from_str(did_url_str);
        assert!(did_url.is_ok());
    }

    #[test]
    fn test_did_url_deserialize_invalid() {
        let did_url_str = "\"invalid:example:123456789abcdefghi#key-1\"";
        let did_url: Result<DidUrl, _> = serde_json::from_str(did_url_str);
        assert!(did_url.is_err());
    }

    #[test]
    fn test_did_url_from_str_valid() {
        let did_url = DidUrl::from_str("did:example:123456789abcdefghi#key-1");
        assert!(did_url.is_ok());
    }

    #[test]
    fn test_did_url_from_str_invalid() {
        let did_url = DidUrl::from_str("invalid:example:123456789abcdefghi#key-1");
        assert!(did_url.is_err());
    }

    #[test]
    fn test_did_url_to_string() {
        let did_url = DidUrl::new("did:example:123456789abcdefghi#key-1".to_string()).unwrap();
        assert_eq!(did_url.to_string(), "did:example:123456789abcdefghi#key-1");
    }

    #[test]
    fn test_is_valid_did_url() {
        assert!(is_valid_did_url("did:example:123456789abcdefghi#key-1"));
        assert!(!is_valid_did_url(
            "invalid:example:123456789abcdefghi#key-1"
        ));
    }
}
