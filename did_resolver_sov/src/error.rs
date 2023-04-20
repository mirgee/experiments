use aries_vcx_core::errors::error::AriesVcxCoreError;
use did_resolver::{
    did_doc_builder::error::DIDDocumentBuilderError,
    traits::resolvable::{
        resolution_error::DIDResolutionError, resolution_metadata::DIDResolutionMetadata,
    },
};
use thiserror::Error;

// TODO: DIDDocumentBuilderError should do key validation and the error
// should me mapped accordingly
#[derive(Debug, Error)]
pub enum DIDSovError {
    #[error("DID document not found")]
    NotFound,
    #[error("DID method not supported: {0}")]
    MethodNotSupported(String),
    #[error("Internal error")]
    InternalError,
    #[error("AriesVCX Core error: {0}")]
    AriesVcxCoreError(#[from] AriesVcxCoreError),
    #[error("DID Document Builder Error: {0}")]
    DIDDocumentBuilderError(#[from] DIDDocumentBuilderError),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

impl From<DIDSovError> for DIDResolutionError {
    fn from(err: DIDSovError) -> Self {
        match err {
            DIDSovError::NotFound => DIDResolutionError::NotFound,
            DIDSovError::MethodNotSupported(_) => DIDResolutionError::MethodNotSupported,
            _ => DIDResolutionError::InternalError,
        }
    }
}

impl From<DIDSovError> for DIDResolutionMetadata {
    fn from(err: DIDSovError) -> Self {
        DIDResolutionMetadata::builder().error(err.into()).build()
    }
}
