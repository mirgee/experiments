use crate::{
    error::GenericError,
    resolvable::{DIDDocumentMetadata, DIDResolvable},
};
use async_trait::async_trait;
use did_parser::ParsedDID;
use serde::{Deserialize, Serialize};

// This struct may be empty, so every future field should be an option
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DIDDereferencingOptions {
    accept: Option<String>, // NOTE: This is a media type
}

impl DIDDereferencingOptions {
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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DIDDereferencingMetadata {
    content_type: Option<String>,
    error: Option<DIDDereferencingErrorType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum DIDDereferencingErrorType {
    InvalidDid,
    NotFound,
}

#[async_trait]
pub trait DIDDereferenceable: DIDResolvable {
    async fn dereference(
        &mut self,
        did: ParsedDID,
        options: DIDDereferencingOptions,
    ) -> Result<DIDDereferencingOutput, GenericError>;
}
