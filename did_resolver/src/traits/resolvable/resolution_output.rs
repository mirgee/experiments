use did_doc_builder::schema::did_doc::DIDDocument;
use serde::{Deserialize, Serialize};

use crate::traits::did_document_metadata::DIDDocumentMetadata;

use super::resolution_metadata::DIDResolutionMetadata;

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
