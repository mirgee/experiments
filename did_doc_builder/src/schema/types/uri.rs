use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::DIDDocumentBuilderError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Uri(#[serde(with = "http_serde::uri")] http::Uri);

impl Uri {
    pub fn new(uri: String) -> Result<Self, DIDDocumentBuilderError> {
        Ok(Self(http::Uri::from_maybe_shared(uri).map_err(|e| {
            DIDDocumentBuilderError::InvalidInput(format!("Invalid URI: {}", e))
        })?))
    }
}

impl FromStr for Uri {
    type Err = DIDDocumentBuilderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}
