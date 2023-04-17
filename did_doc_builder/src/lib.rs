extern crate serde;
extern crate serde_json;

pub mod did;
pub mod did_url;
pub mod uri;

use did::Did;
use did_url::DidUrl;
use jsonwebkey::JsonWebKey;
use serde::{Deserialize, Serialize};
use uri::Uri;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
enum VerificationMethodAlias {
    VerificationMethod(VerificationMethod),
    VerificationMethodReference(DidUrl), // TODO: Must be a DID URL
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
enum ControllerAlias {
    SingleController(Did),
    MultipleControllers(Vec<Did>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DIDDocument {
    id: Did,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    also_known_as: Vec<Uri>,
    #[serde(skip_serializing_if = "Option::is_none")]
    controller: Option<ControllerAlias>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    verification_method: Vec<VerificationMethod>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    authentication: Vec<VerificationMethodAlias>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    assertion_method: Vec<VerificationMethodAlias>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    key_agreement: Vec<VerificationMethodAlias>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    capability_invocation: Vec<VerificationMethodAlias>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    capability_delegation: Vec<VerificationMethodAlias>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    service: Vec<Service>,
}

// TODO: It seems that this may contain pretty much anything
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    id: Uri,
    r#type: String,
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

// TODO: Use excl. refs
impl DIDDocument {
    pub fn new(id: &str) -> Result<Self, std::io::Error> {
        Ok(Self {
            id: Did::new(id.to_string())?,
            ..Default::default()
        })
    }

    pub fn add_service(mut self, svc: Service) -> Self {
        self.service.push(svc);
        self
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self)
    }
}
