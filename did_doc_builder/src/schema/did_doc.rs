use serde::{Deserialize, Serialize};

use super::{
    service::Service,
    types::{did::Did, did_url::DidUrl, uri::Uri},
    utils::OneOrList,
    verification_method::{VerificationMethod, VerificationMethodAlias},
};

type ControllerAlias = OneOrList<Did>;

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
#[allow(dead_code)]
pub struct DIDDocumentBuilder {
    id: Did,
    also_known_as: Vec<Uri>,
    controller: Vec<Did>,
    verification_method: Vec<VerificationMethod>,
    authentication: Vec<VerificationMethodAlias>,
    assertion_method: Vec<VerificationMethodAlias>,
    key_agreement: Vec<VerificationMethodAlias>,
    capability_invocation: Vec<VerificationMethodAlias>,
    capability_delegation: Vec<VerificationMethodAlias>,
    service: Vec<Service>,
}

#[allow(dead_code)]
impl DIDDocumentBuilder {
    pub fn new(id: Did) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn add_also_known_as(&mut self, also_known_as: Uri) -> &mut Self {
        self.also_known_as.push(also_known_as);
        self
    }

    pub fn add_controller(&mut self, controller: Did) -> &mut Self {
        self.controller.push(controller);
        self
    }

    pub fn add_verification_method(
        &mut self,
        verification_method: VerificationMethod,
    ) -> &mut Self {
        self.verification_method.push(verification_method);
        self
    }

    pub fn add_authentication_method(&mut self, method: VerificationMethod) -> &mut Self {
        self.authentication
            .push(VerificationMethodAlias::VerificationMethod(method));
        self
    }

    pub fn add_authentication_reference(&mut self, reference: DidUrl) -> &mut Self {
        self.authentication
            .push(VerificationMethodAlias::VerificationMethodReference(
                reference,
            ));
        self
    }

    pub fn add_assertion_method(&mut self, method: VerificationMethod) -> &mut Self {
        self.assertion_method
            .push(VerificationMethodAlias::VerificationMethod(method));
        self
    }

    pub fn add_assertion_method_reference(&mut self, reference: DidUrl) -> &mut Self {
        self.assertion_method
            .push(VerificationMethodAlias::VerificationMethodReference(
                reference,
            ));
        self
    }

    pub fn add_key_agreement(&mut self, key_agreement: VerificationMethod) -> &mut Self {
        self.key_agreement
            .push(VerificationMethodAlias::VerificationMethod(key_agreement));
        self
    }

    pub fn add_key_agreement_refrence(&mut self, reference: DidUrl) -> &mut Self {
        self.key_agreement
            .push(VerificationMethodAlias::VerificationMethodReference(
                reference,
            ));
        self
    }

    pub fn add_capability_invocation(
        &mut self,
        capability_invocation: VerificationMethod,
    ) -> &mut Self {
        self.capability_invocation
            .push(VerificationMethodAlias::VerificationMethod(
                capability_invocation,
            ));
        self
    }

    pub fn add_capability_invocation_refrence(&mut self, reference: DidUrl) -> &mut Self {
        self.capability_invocation
            .push(VerificationMethodAlias::VerificationMethodReference(
                reference,
            ));
        self
    }

    pub fn add_capability_delegation(
        &mut self,
        capability_delegation: VerificationMethod,
    ) -> &mut Self {
        self.capability_delegation
            .push(VerificationMethodAlias::VerificationMethod(
                capability_delegation,
            ));
        self
    }

    pub fn add_capability_delegation_refrence(&mut self, reference: DidUrl) -> &mut Self {
        self.capability_delegation
            .push(VerificationMethodAlias::VerificationMethodReference(
                reference,
            ));
        self
    }

    pub fn add_service(&mut self, service: Service) -> &mut Self {
        self.service.push(service);
        self
    }

    pub fn build(self) -> DIDDocument {
        DIDDocument {
            id: self.id,
            also_known_as: self.also_known_as,
            controller: if self.controller.is_empty() {
                None
            } else {
                Some(OneOrList::List(self.controller))
            },
            verification_method: self.verification_method,
            authentication: self.authentication,
            assertion_method: self.assertion_method,
            key_agreement: self.key_agreement,
            capability_invocation: self.capability_invocation,
            capability_delegation: self.capability_delegation,
            service: self.service,
        }
    }
}
