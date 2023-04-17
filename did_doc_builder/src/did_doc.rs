use crate::uri::Uri;
use crate::utils::{Service, VerificationMethod, VerificationMethodAlias};
use crate::{did::Did, utils::ControllerAlias};
use serde::{Deserialize, Serialize};

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
