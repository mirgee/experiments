use crate::did::Did;
use crate::did_url::DidUrl;
use jsonwebkey::JsonWebKey;
use serde::{Deserialize, Serialize};

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
    r#type: String,
    controller: Did,
    public_key_multibase: Option<String>, // TODO: Must be a valid multibase key
    public_key_jwk: Option<JsonWebKey>,
}
