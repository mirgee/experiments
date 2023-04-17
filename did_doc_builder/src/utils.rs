use crate::did::Did;
use crate::did_url::DidUrl;
use crate::uri::Uri;
use jsonwebkey::JsonWebKey;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum VerificationMethodAlias {
    VerificationMethod(VerificationMethod),
    VerificationMethodReference(DidUrl),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum OneOrList<T> {
    SingleService(T),
    MultipleServices(Vec<T>),
}

pub type ControllerAlias = OneOrList<Did>;
pub type ServiceTypeAlias = OneOrList<String>;

// TODO: It seems that this may contain pretty much anything?
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    id: Uri,
    r#type: ServiceTypeAlias,
    service_endpoint: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VerificationMethod {
    id: String,
    r#type: String,
    controller: Did,
    public_key_multibase: Option<String>, // TODO: Must be a valid multibase key
    public_key_jwk: Option<JsonWebKey>,
}
