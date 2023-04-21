pub mod dereferencing_error;
pub mod dereferencing_metadata;
pub mod dereferencing_options;
pub mod dereferencing_output;

use crate::{error::GenericError, traits::resolvable::DIDResolvable};
use async_trait::async_trait;
use did_parser::ParsedDIDUrl;

use self::{
    dereferencing_options::DIDDereferencingOptions, dereferencing_output::DIDDereferencingOutput,
};

#[async_trait]
pub trait DIDDereferenceable: DIDResolvable {
    async fn dereference(
        &mut self,
        did: ParsedDIDUrl,
        options: DIDDereferencingOptions,
    ) -> Result<DIDDereferencingOutput, GenericError>;
}
