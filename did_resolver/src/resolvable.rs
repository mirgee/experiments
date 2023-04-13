use crate::error::GenericError;
use async_trait::async_trait;
use did_doc_builder::DIDDocument;
use did_parser::ParsedDID;

#[async_trait]
pub trait DIDResolvable {
    async fn resolve(&self, did: ParsedDID) -> Result<DIDDocument, GenericError>;
}
