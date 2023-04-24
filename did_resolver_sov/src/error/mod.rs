pub mod parsing;
mod resolution;

use aries_vcx_core::errors::error::AriesVcxCoreError;
use did_resolver::did_doc_builder::error::DIDDocumentBuilderError;
use thiserror::Error;

use self::parsing::ParsingErrorSource;

// TODO: DIDDocumentBuilderError should do key validation and the error
// should me mapped accordingly
// TODO: Perhaps split into input errors and external errors?
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum DIDSovError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("DID method not supported: {0}")]
    MethodNotSupported(String),
    #[error("Internal error")]
    InternalError,
    #[error("Invalid DID: {0}")]
    InvalidDID(String),
    #[error("AriesVCX Core error: {0}")]
    AriesVcxCoreError(#[from] AriesVcxCoreError),
    #[error("DID Document Builder Error: {0}")]
    DIDDocumentBuilderError(#[from] DIDDocumentBuilderError),
    #[error("Parsing error: {0}")]
    ParsingError(#[from] ParsingErrorSource),
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}
