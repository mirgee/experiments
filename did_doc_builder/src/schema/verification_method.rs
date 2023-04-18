use jsonwebkey::JsonWebKey;
use serde::{Deserialize, Serialize};

use super::types::{did::Did, did_url::DidUrl};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub(crate) enum VerificationMethodAlias {
    VerificationMethod(VerificationMethod),
    VerificationMethodReference(DidUrl),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VerificationMethod {
    id: Did,
    controller: Did,
    r#type: String,
    public_key_multibase: Option<String>,
    public_key_jwk: Option<JsonWebKey>,
}

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct VerificationMethodBuilder {
    id: Did,
    controller: Did,
    r#type: String,
    public_key_multibase: Option<String>, // TODO: Multibase key validation
    public_key_jwk: Option<JsonWebKey>,
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
    pub fn add_public_key_multibase_string(&mut self, public_key_multibase: String) -> &mut Self {
        self.public_key_multibase = Some(public_key_multibase);
        self
    }

    pub fn add_public_key_jwk(&mut self, public_key_jwk: JsonWebKey) -> &mut Self {
        self.public_key_jwk = Some(public_key_jwk);
        self
    }

    pub fn build(self) -> VerificationMethod {
        VerificationMethod {
            id: self.id,
            r#type: self.r#type,
            controller: self.controller,
            public_key_multibase: self.public_key_multibase,
            public_key_jwk: self.public_key_jwk,
        }
    }
}
