use std::collections::HashMap;

use did_parser::ParsedDIDUrl;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::DIDDocumentBuilderError;

use super::types::{did::Did, jsonwebkey::JsonWebKey, multibase::Multibase};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum VerificationMethodAlias {
    VerificationMethod(VerificationMethod),
    VerificationMethodReference(ParsedDIDUrl),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VerificationMethod {
    id: ParsedDIDUrl,
    controller: Did,
    r#type: String,
    public_key_multibase: Option<Multibase>,
    public_key_jwk: Option<JsonWebKey>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    extra: HashMap<String, Value>,
}

impl VerificationMethod {
    pub fn builder() -> VerificationMethodBuilder {
        VerificationMethodBuilder::default()
    }

    pub fn id(&self) -> &ParsedDIDUrl {
        &self.id
    }

    pub fn controller(&self) -> &Did {
        &self.controller
    }

    pub fn r#type(&self) -> &str {
        self.r#type.as_ref()
    }

    pub fn public_key_multibase(&self) -> Option<&Multibase> {
        self.public_key_multibase.as_ref()
    }

    pub fn public_key_jwk(&self) -> Option<&JsonWebKey> {
        self.public_key_jwk.as_ref()
    }

    pub fn extra(&self, key: &str) -> Option<&Value> {
        self.extra.get(key)
    }
}

#[derive(Debug, Default)]
pub struct VerificationMethodBuilder {
    id: ParsedDIDUrl,
    controller: Did,
    r#type: String,
    public_key_multibase: Option<Multibase>,
    public_key_jwk: Option<JsonWebKey>,
    extra: HashMap<String, Value>,
}

impl VerificationMethodBuilder {
    pub fn new(id: ParsedDIDUrl, controller: Did, r#type: String) -> Self {
        Self {
            id,
            r#type,
            controller,
            ..Default::default()
        }
    }

    // We will rely on users to provide valid multibase keys for now
    pub fn add_public_key_multibase(mut self, public_key_multibase: Multibase) -> Self {
        self.public_key_multibase = Some(public_key_multibase);
        self
    }

    pub fn add_public_key_jwk(mut self, public_key_jwk: JsonWebKey) -> Self {
        self.public_key_jwk = Some(public_key_jwk);
        self
    }

    pub fn add_extra(mut self, key: String, value: Value) -> Self {
        self.extra.insert(key, value);
        self
    }

    pub fn build(self) -> Result<VerificationMethod, DIDDocumentBuilderError> {
        if self.public_key_multibase.is_some() && self.public_key_jwk.is_some() {
            return Err(DIDDocumentBuilderError::InvalidInput(
                "Cannot specify both public_key_multibase and public_key_jwk".to_string(),
            ));
        } else {
            Ok(VerificationMethod {
                id: self.id,
                r#type: self.r#type,
                controller: self.controller,
                public_key_multibase: self.public_key_multibase,
                public_key_jwk: self.public_key_jwk,
                extra: self.extra,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_did() -> Did {
        Did::new("did:example:123456789abcdefghi".to_string()).unwrap()
    }

    fn create_valid_did_url() -> ParsedDIDUrl {
        ParsedDIDUrl::parse("did:example:123456789abcdefghi#fragment".to_string()).unwrap()
    }

    fn create_valid_multibase() -> Multibase {
        Multibase::new("zQmWvQxTqbG2Z9HPJgG57jjwR154cKhbtJenbyYTWkjgF3e".to_string()).unwrap()
    }

    #[test]
    fn test_verification_method_id() {
        let id = create_valid_did_url();
        let controller = create_valid_did();
        let r#type = "Ed25519VerificationKey2018".to_string();
        let vm = VerificationMethodBuilder::new(id.clone(), controller.clone(), r#type.clone())
            .build()
            .unwrap();
        assert_eq!(vm.id(), &id);
    }

    #[test]
    fn test_verification_method_builder() {
        let id = create_valid_did_url();
        let controller = create_valid_did();
        let r#type = "Ed25519VerificationKey2018".to_string();
        let public_key_multibase = create_valid_multibase();

        let vm = VerificationMethodBuilder::new(id.clone(), controller.clone(), r#type.clone())
            .add_public_key_multibase(public_key_multibase.clone())
            .build()
            .unwrap();

        assert_eq!(vm.id(), &id);
        assert_eq!(vm.controller(), &controller);
        assert_eq!(vm.r#type(), &r#type);
        assert_eq!(vm.public_key_multibase().unwrap(), &public_key_multibase);
    }

    #[test]
    fn test_verification_method_extra() {
        let id = create_valid_did_url();
        let controller = create_valid_did();
        let r#type = "Ed25519VerificationKey2018".to_string();
        let extra_key = "foo".to_string();
        let extra_value = Value::String("bar".to_string());

        let vm = VerificationMethodBuilder::new(id.clone(), controller.clone(), r#type.clone())
            .add_extra(extra_key.clone(), extra_value.clone())
            .build()
            .unwrap();
        assert_eq!(vm.extra(&extra_key).unwrap(), &extra_value);
    }

    #[test]
    fn test_verification_method_builder_complete() {
        let id = create_valid_did_url();
        let controller = create_valid_did();
        let r#type = "Ed25519VerificationKey2018".to_string();
        let public_key_multibase = create_valid_multibase();
        let extra_key = "foo".to_string();
        let extra_value = Value::String("bar".to_string());

        let vm = VerificationMethodBuilder::new(id.clone(), controller.clone(), r#type.clone())
            .add_public_key_multibase(public_key_multibase.clone())
            .add_extra(extra_key.clone(), extra_value.clone())
            .build()
            .unwrap();

        assert_eq!(vm.id(), &id);
        assert_eq!(vm.controller(), &controller);
        assert_eq!(vm.r#type(), &r#type);
        assert_eq!(vm.public_key_multibase().unwrap(), &public_key_multibase);
        assert_eq!(vm.extra(&extra_key).unwrap(), &extra_value);
    }
}
