use crate::service::Service;
use crate::uri::Uri;
use crate::utils::OneOrList;
use crate::verification_method::VerificationMethod;
use crate::{did::Did, verification_method::VerificationMethodAlias};
use serde::{Deserialize, Serialize};

pub type ControllerAlias = OneOrList<Did>;

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

#[derive(Debug, Default)]
pub struct DIDDocumentBuilder {
    id: Did,
    also_known_as: Vec<Uri>,
    controller: Option<ControllerAlias>,
    verification_method: Vec<VerificationMethod>,
    authentication: Vec<VerificationMethodAlias>,
    assertion_method: Vec<VerificationMethodAlias>,
    key_agreement: Vec<VerificationMethodAlias>,
    capability_invocation: Vec<VerificationMethodAlias>,
    capability_delegation: Vec<VerificationMethodAlias>,
    service: Vec<Service>,
}

impl DIDDocumentBuilder {
    pub fn new(id: Did) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn set_also_known_as(&mut self, also_known_as: Vec<Uri>) -> &mut Self {
        self.also_known_as = also_known_as;
        self
    }

    pub fn set_controller(&mut self, controller: ControllerAlias) -> &mut Self {
        self.controller = Some(controller);
        self
    }

    pub fn set_verification_method(
        &mut self,
        verification_method: Vec<VerificationMethod>,
    ) -> &mut Self {
        self.verification_method = verification_method;
        self
    }

    pub fn set_authentication(
        &mut self,
        authentication: Vec<VerificationMethodAlias>,
    ) -> &mut Self {
        self.authentication = authentication;
        self
    }

    pub fn set_assertion_method(
        &mut self,
        assertion_method: Vec<VerificationMethodAlias>,
    ) -> &mut Self {
        self.assertion_method = assertion_method;
        self
    }

    pub fn set_key_agreement(&mut self, key_agreement: Vec<VerificationMethodAlias>) -> &mut Self {
        self.key_agreement = key_agreement;
        self
    }

    pub fn set_capability_invocation(
        &mut self,
        capability_invocation: Vec<VerificationMethodAlias>,
    ) -> &mut Self {
        self.capability_invocation = capability_invocation;
        self
    }

    pub fn set_capability_delegation(
        &mut self,
        capability_delegation: Vec<VerificationMethodAlias>,
    ) -> &mut Self {
        self.capability_delegation = capability_delegation;
        self
    }

    pub fn set_service(&mut self, service: Vec<Service>) -> &mut Self {
        self.service = service;
        self
    }

    pub fn id(&self) -> &Did {
        &self.id
    }

    pub fn also_known_as(&self) -> &[Uri] {
        self.also_known_as.as_ref()
    }

    pub fn controller(&self) -> Option<&ControllerAlias> {
        self.controller.as_ref()
    }

    pub fn verification_method(&self) -> &[VerificationMethod] {
        self.verification_method.as_ref()
    }

    pub fn authentication(&self) -> &[VerificationMethodAlias] {
        self.authentication.as_ref()
    }

    pub fn assertion_method(&self) -> &[VerificationMethodAlias] {
        self.assertion_method.as_ref()
    }

    pub fn key_agreement(&self) -> &[VerificationMethodAlias] {
        self.key_agreement.as_ref()
    }

    pub fn capability_invocation(&self) -> &[VerificationMethodAlias] {
        self.capability_invocation.as_ref()
    }

    pub fn capability_delegation(&self) -> &[VerificationMethodAlias] {
        self.capability_delegation.as_ref()
    }

    pub fn service(&self) -> &[Service] {
        self.service.as_ref()
    }

    pub fn build(&self) -> DIDDocument {
        DIDDocument {
            id: self.id.clone(),
            also_known_as: self.also_known_as.clone(),
            controller: self.controller.clone(),
            verification_method: self.verification_method.clone(),
            authentication: self.authentication.clone(),
            assertion_method: self.assertion_method.clone(),
            key_agreement: self.key_agreement.clone(),
            capability_invocation: self.capability_invocation.clone(),
            capability_delegation: self.capability_delegation.clone(),
            service: self.service.clone(),
        }
    }
}
