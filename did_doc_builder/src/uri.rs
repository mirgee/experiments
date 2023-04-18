use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Uri(#[serde(with = "http_serde::uri")] http::Uri);

impl Uri {
    pub fn new(uri: String) -> Result<Self, std::io::Error> {
        Ok(Self(http::Uri::from_maybe_shared(uri).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid URI: {}", e),
            )
        })?))
    }
}
