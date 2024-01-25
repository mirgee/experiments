use std::fmt::Display;

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Clone, Debug, PartialEq)]
pub enum AcceptType {
    DIDCommV1,
    DIDCommV2,
    Other(String),
}

impl<'de> Deserialize<'de> for AcceptType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "didcomm/aip2;env=rfc19" => Ok(AcceptType::DIDCommV1),
            "didcomm/v2" => Ok(AcceptType::DIDCommV2),
            _ => Ok(AcceptType::Other(s)),
        }
    }
}

impl Display for AcceptType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AcceptType::DIDCommV1 => write!(f, "didcomm/aip2;env=rfc19"),
            AcceptType::DIDCommV2 => write!(f, "didcomm/v2"),
            AcceptType::Other(other) => write!(f, "{}", other),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct ExtraFieldsAIP1 {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExtraFieldsDidCommV1 {
    priority: u32,
    recipient_keys: Vec<String>,
    routing_keys: Vec<String>,
    accept: Vec<AcceptType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExtraFieldsDidCommV2 {
    accept: Vec<AcceptType>,
    routing_keys: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ExtraFields {
    AIP1(ExtraFieldsAIP1),
    DIDCommV1(ExtraFieldsDidCommV1),
    DIDCommV2(ExtraFieldsDidCommV2),
}

impl Default for ExtraFields {
    fn default() -> Self {
        ExtraFields::AIP1(ExtraFieldsAIP1::default())
    }
}

#[cfg(test)]
mod tests {
    use did_doc::schema::service::Service;

    use super::*;

    const AIP1_SERVICE: &'static str = r#"
{
    "id": "did:sov:HR6vs6GEZ8rHaVgjg2WodM#endpoint",
    "type": "endpoint",
    "serviceEndpoint": "https://example.com/endpoint"
}
"#;

    const DIDCOMMV1_SERVICE: &'static str = r#"
{
    "id": "did:sov:HR6vs6GEZ8rHaVgjg2WodM#did-communication",
    "type": "did-communication",
    "priority": 0,
    "recipientKeys": [
        "did:sov:HR6vs6GEZ8rHaVgjg2WodM#key-agreement-1"
    ],
    "routingKeys": [],
    "accept": [
        "didcomm/aip2;env=rfc19"
    ],
    "serviceEndpoint": "https://example.com/endpoint"
}
"#;

    const DIDCOMMV2_SERVICE: &'static str = r#"
{
  "id": "did:sov:HR6vs6GEZ8rHaVgjg2WodM#didcomm-1",
  "type": "DIDComm",
  "accept": [
    "didcomm/v2"
  ],
  "routingKeys": [],
  "serviceEndpoint": "https://example.com/endpoint"
}
"#;

    #[test]
    fn it_works() {
        let service: Service<ExtraFields> = serde_json::from_str(AIP1_SERVICE).unwrap();
        println!("{:?}", service);

        let service: Service<ExtraFields> = serde_json::from_str(DIDCOMMV1_SERVICE).unwrap();
        println!("{:?}", service);

        let service: Service<ExtraFields> = serde_json::from_str(DIDCOMMV2_SERVICE).unwrap();
        println!("{:?}", service);
    }
}
