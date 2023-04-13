extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DIDDocument {
    pub id: String,
    pub public_key: Vec<PublicKey>,
    pub authentication: Vec<Authentication>,
    pub service: Vec<Service>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    pub id: String,
    pub r#type: String,
    pub controller: String,
    pub public_key_base58: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Authentication {
    pub id: String,
    pub r#type: String,
    pub public_key: String,
}

// TODO: It seems that this may contain pretty much anything
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub id: String,
    pub r#type: String,
    pub service_endpoint: String,
}

impl DIDDocument {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            public_key: Vec::new(),
            authentication: Vec::new(),
            service: Vec::new(),
        }
    }

    pub fn add_public_key(mut self, key: PublicKey) -> Self {
        self.public_key.push(key);
        self
    }

    pub fn add_authentication(mut self, auth: Authentication) -> Self {
        self.authentication.push(auth);
        self
    }

    pub fn add_service(mut self, svc: Service) -> Self {
        self.service.push(svc);
        self
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_did_document_builder() {
        let did_doc = DIDDocument::new("did:example:123456789abcdefghi")
            .add_public_key(PublicKey {
                id: "did:example:123456789abcdefghi#keys-1".to_string(),
                r#type: "Ed25519VerificationKey2018".to_string(),
                controller: "did:example:123456789abcdefghi".to_string(),
                public_key_base58: "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".to_string(),
            })
            .add_authentication(Authentication {
                id: "did:example:123456789abcdefghi#keys-1".to_string(),
                r#type: "Ed25519VerificationKey2018".to_string(),
                public_key: "did:example:123456789abcdefghi#keys-1".to_string(),
            })
            .add_service(Service {
                id: "did:example:123456789abcdefghi#agent".to_string(),
                r#type: "DIDCommMessaging".to_string(),
                service_endpoint: "https://example.com/endpoint".to_string(),
            });

        let expected_json = r#"{
  "id": "did:example:123456789abcdefghi",
  "publicKey": [
    {
      "id": "did:example:123456789abcdefghi#keys-1",
      "type": "Ed25519VerificationKey2018",
      "controller": "did:example:123456789abcdefghi",
      "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
    }
  ],
  "authentication": [
    {
      "id": "did:example:123456789abcdefghi#keys-1",
      "type": "Ed25519VerificationKey2018",
      "publicKey": "did:example:123456789abcdefghi#keys-1"
    }
  ],
  "service": [
    {
      "id": "did:example:123456789abcdefghi#agent",
      "type": "DIDCommMessaging",
      "serviceEndpoint": "https://example.com/endpoint"
    }
  ]
}"#;

        assert_eq!(did_doc.to_json().unwrap(), expected_json);
    }
}
