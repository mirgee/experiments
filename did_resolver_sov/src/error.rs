use std::error::Error;

use aries_vcx_core::errors::error::AriesVcxCoreError;
use did_resolver::did_doc_builder::error::DIDDocumentBuilderError;

#[derive(Debug)]
pub struct DIDSovError;

impl Error for DIDSovError {}

impl std::fmt::Display for DIDSovError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DIDSovError")
    }
}

impl From<AriesVcxCoreError> for DIDSovError {
    fn from(_err: AriesVcxCoreError) -> Self {
        DIDSovError
    }
}

impl From<serde_json::Error> for DIDSovError {
    fn from(_err: serde_json::Error) -> Self {
        DIDSovError
    }
}

impl From<DIDDocumentBuilderError> for DIDSovError {
    fn from(_err: DIDDocumentBuilderError) -> Self {
        DIDSovError
    }
}
