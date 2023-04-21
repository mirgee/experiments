pub mod resolution_error;
pub mod resolution_metadata;
pub mod resolution_options;
pub mod resolution_output;

use crate::error::GenericError;
use async_trait::async_trait;
use did_parser::ParsedDIDUrl;

use self::{resolution_options::DIDResolutionOptions, resolution_output::DIDResolutionOutput};

#[async_trait]
pub trait DIDResolvable {
    async fn resolve(
        &mut self,
        did: ParsedDIDUrl, // TODO: should be DID
        options: DIDResolutionOptions,
    ) -> Result<DIDResolutionOutput, GenericError>;
}
