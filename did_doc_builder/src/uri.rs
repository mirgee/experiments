use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Uri(#[serde(with = "http_serde::uri")] http::Uri);
