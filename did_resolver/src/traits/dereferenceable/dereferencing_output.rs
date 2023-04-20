use crate::shared_types::did_document_metadata::DIDDocumentMetadata;
use std::io::Read;

use super::dereferencing_metadata::DIDDereferencingMetadata;

pub struct DIDDereferencingOutput {
    dereferencing_metadata: DIDDereferencingMetadata,
    content_stream: Box<dyn Read + Send + Sync>,
    content_metadata: DIDDocumentMetadata,
}

impl DIDDereferencingOutput {
    pub fn builder(content_stream: Box<dyn Read + Send + Sync>) -> DIDDereferencingOutputBuilder {
        DIDDereferencingOutputBuilder {
            dereferencing_metadata: None,
            content_stream,
            content_metadata: None,
        }
    }

    pub fn dereferencing_metadata(&self) -> &DIDDereferencingMetadata {
        &self.dereferencing_metadata
    }

    pub fn content_stream(&self) -> &Box<dyn std::io::Read + Send + Sync> {
        &self.content_stream
    }

    pub fn content_metadata(&self) -> &DIDDocumentMetadata {
        &self.content_metadata
    }
}

pub struct DIDDereferencingOutputBuilder {
    dereferencing_metadata: Option<DIDDereferencingMetadata>,
    content_stream: Box<dyn Read + Send + Sync>,
    content_metadata: Option<DIDDocumentMetadata>,
}

impl DIDDereferencingOutputBuilder {
    pub fn dereferencing_metadata(
        mut self,
        dereferencing_metadata: DIDDereferencingMetadata,
    ) -> Self {
        self.dereferencing_metadata = Some(dereferencing_metadata);
        self
    }

    pub fn content_metadata(mut self, content_metadata: DIDDocumentMetadata) -> Self {
        self.content_metadata = Some(content_metadata);
        self
    }

    pub fn build(self) -> DIDDereferencingOutput {
        DIDDereferencingOutput {
            dereferencing_metadata: self.dereferencing_metadata.unwrap_or_default(),
            content_stream: self.content_stream,
            content_metadata: self.content_metadata.unwrap_or_default(),
        }
    }
}
