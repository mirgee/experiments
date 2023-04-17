use crate::error::GenericError;
use async_trait::async_trait;
use did_doc_builder::DIDDocument;
use did_parser::ParsedDID;

#[async_trait]
pub trait DIDResolvable {
    // Use interior mutability instead of &mut self?
    async fn resolve(&mut self, did: ParsedDID) -> Result<DIDDocument, GenericError>;
}
