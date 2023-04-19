use crate::error::GenericError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use did_doc_builder::schema::{did_doc::DIDDocument, types::did::Did};
use did_parser::ParsedDID;
use serde::{Deserialize, Serialize};

// This struct may be empty, so every future field should be an option
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DIDResolutionOptions {
    accept: Option<String>, // NOTE: This is a media type
}

impl DIDResolutionOptions {
    pub fn new() -> Self {
        Self { accept: None }
    }

    pub fn set_accept(mut self, accept: String) -> Self {
        self.accept = Some(accept);
        self
    }

    pub fn accept(&self) -> Option<&String> {
        self.accept.as_ref()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DIDResolutionOutput {
    did_document: DIDDocument,
    did_resolution_metadata: DIDResolutionMetadata,
    did_document_metadata: DIDDocumentMetadata,
}

impl DIDResolutionOutput {
    pub fn new(did_document: DIDDocument) -> Self {
        DIDResolutionOutput {
            did_document,
            did_resolution_metadata: DIDResolutionMetadata::default(),
            did_document_metadata: DIDDocumentMetadata::default(),
        }
    }

    pub fn set_did_resolution_metadata(
        mut self,
        did_resolution_metadata: DIDResolutionMetadata,
    ) -> Self {
        self.did_resolution_metadata = did_resolution_metadata;
        self
    }

    pub fn set_did_document_metadata(mut self, did_document_metadata: DIDDocumentMetadata) -> Self {
        self.did_document_metadata = did_document_metadata;
        self
    }

    pub fn did_document(&self) -> &DIDDocument {
        &self.did_document
    }

    pub fn did_resolution_metadata(&self) -> &DIDResolutionMetadata {
        &self.did_resolution_metadata
    }

    pub fn did_document_metadata(&self) -> &DIDDocumentMetadata {
        &self.did_document_metadata
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum DIDResolutionErrorType {
    InvalidDid,
    NotFound,
    RepresentationNotSupported,
    MethodNotSupported,
    InternalError,
    InvalidPublicKey,
    InvalidPublicKeyLength,
    InvalidPublicKeyType,
    UnsupportedPublicKeyType,
    NotAllowedVerificationMethodType,
    NotAllowedKeyType,
    NotAllowedMethod,
    NotAllowedCertificate,
    NotAllowedLocalDuplicateKey,
    NotAllowedLocalDerivedKey,
    NotAllowedGlobalDuplicateKey,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct DIDResolutionMetadata {
    content_type: Option<String>,
    error: Option<DIDResolutionErrorType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DIDDocumentMetadata {
    created: Option<DateTime<Utc>>,
    updated: Option<DateTime<Utc>>,
    deactivated: Option<bool>,
    next_update: Option<DateTime<Utc>>,
    version_id: String,
    next_version_id: String,
    equivalent_id: Vec<Did>,
    canonical_id: Option<Did>,
}

#[async_trait]
pub trait DIDResolvable {
    // Should take resolution options, return resolution metadata as well
    async fn resolve(
        &mut self,
        did: ParsedDID,
        options: DIDResolutionOptions,
    ) -> Result<DIDResolutionOutput, GenericError>;
}
