use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EndpointDidSov {
    pub endpoint: String,
    #[serde(default)]
    pub routing_keys: Vec<String>,
    #[serde(default)]
    pub types: Vec<DidSovServiceType>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum DidSovServiceType {
    #[serde(rename = "endpoint")] // AIP 1.0
    Endpoint,
    #[serde(rename = "did-communication")] // AIP 2.0
    DidCommunication,
    #[serde(rename = "DIDComm")] // DIDComm V2
    DIDComm,
    #[serde(other)]
    Unknown,
}

impl Display for DidSovServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DidSovServiceType::Endpoint => write!(f, "endpoint"),
            DidSovServiceType::DidCommunication => write!(f, "did-communication"),
            DidSovServiceType::DIDComm => write!(f, "DIDComm"),
            DidSovServiceType::Unknown => write!(f, "Unknown"),
        }
    }
}
