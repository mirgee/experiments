use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::DIDDocumentBuilderError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Uri(uniresid::Uri);

impl Uri {
    pub fn new(uri: String) -> Result<Self, DIDDocumentBuilderError> {
        Ok(Self(uniresid::Uri::try_from(uri).map_err(|e| {
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
