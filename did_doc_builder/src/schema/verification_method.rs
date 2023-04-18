use std::collections::HashMap;

use jsonwebkey::JsonWebKey;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::types::{did::Did, did_url::DidUrl, multibase::Multibase};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum VerificationMethodAlias {
    VerificationMethod(VerificationMethod),
    VerificationMethodReference(DidUrl),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VerificationMethod {
    id: Did,
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
    pub fn id(&self) -> &Did {
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
#[allow(dead_code)]
pub struct VerificationMethodBuilder {
    id: Did,
    controller: Did,
    r#type: String,
    public_key_multibase: Option<Multibase>,
    public_key_jwk: Option<JsonWebKey>,
    extra: HashMap<String, Value>,
}

#[allow(dead_code)]
impl VerificationMethodBuilder {
    pub fn new(id: Did, controller: Did, r#type: String) -> Self {
        Self {
            id,
            r#type,
            controller,
            ..Default::default()
        }
    }

    // We will rely on users to provide valid multibase keys for now
    pub fn add_public_key_multibase_string(
        &mut self,
        public_key_multibase: Multibase,
    ) -> &mut Self {
        self.public_key_multibase = Some(public_key_multibase);
        self
    }

    pub fn add_public_key_jwk(&mut self, public_key_jwk: JsonWebKey) -> &mut Self {
        self.public_key_jwk = Some(public_key_jwk);
        self
    }

    pub fn add_extra(&mut self, key: String, value: Value) -> &mut Self {
        self.extra.insert(key, value);
        self
    }

    pub fn build(self) -> VerificationMethod {
        VerificationMethod {
            id: self.id,
            r#type: self.r#type,
            controller: self.controller,
            public_key_multibase: self.public_key_multibase,
            public_key_jwk: self.public_key_jwk,
            extra: self.extra,
        }
    }
}
