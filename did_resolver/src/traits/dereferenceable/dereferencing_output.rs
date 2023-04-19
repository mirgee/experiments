use crate::traits::did_document_metadata::DIDDocumentMetadata;

use super::dereferencing_metadata::DIDDereferencingMetadata;

pub struct DIDDereferencingOutput {
    dereferencing_metadata: DIDDereferencingMetadata,
    content_stream: Box<dyn std::io::Read + Send + Sync>,
    content_metadata: DIDDocumentMetadata,
}

impl DIDDereferencingOutput {
    pub fn new(content_stream: Box<dyn std::io::Read + Send + Sync>) -> Self {
        DIDDereferencingOutput {
            dereferencing_metadata: DIDDereferencingMetadata::default(),
            content_stream,
            content_metadata: DIDDocumentMetadata::default(),
        }
    }

    pub fn set_content_metadata(mut self, content_metadata: DIDDocumentMetadata) -> Self {
        self.content_metadata = content_metadata;
        self
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
