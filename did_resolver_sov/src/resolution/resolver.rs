use std::{num::NonZeroUsize, sync::Arc};

use aries_vcx_core::ledger::base_ledger::BaseLedger;
use async_trait::async_trait;
use did_resolver::{
    did_doc_builder::schema::did_doc::DIDDocument,
    did_parser::ParsedDID,
    error::GenericError,
    traits::resolvable::{
        resolution_options::DIDResolutionOptions, resolution_output::DIDResolutionOutput,
        DIDResolvable,
    },
};
use lru::LruCache;

use super::utils::resolve_ddo;

pub struct DIDSovResolver {
    ledger: Arc<dyn BaseLedger>,
    cache: LruCache<String, Arc<DIDDocument>>,
}

impl DIDSovResolver {
    pub fn new(ledger: Arc<dyn BaseLedger>, cache_size: NonZeroUsize) -> Self {
        DIDSovResolver {
            ledger,
            cache: LruCache::new(cache_size),
        }
    }
}

#[async_trait]
impl DIDResolvable for DIDSovResolver {
    async fn resolve(
        &mut self,
        parsed_did: ParsedDID,
        _options: DIDResolutionOptions,
    ) -> Result<DIDResolutionOutput, GenericError> {
        let did = parsed_did.did();
        if let Some(ddo) = self.cache.get(did) {
            return Ok(DIDResolutionOutput::builder((**ddo).clone()).build());
        }
        let ledger_response = self.ledger.get_attr(did, "endpoint").await?;
        let resolution_output = resolve_ddo(did, &ledger_response).await?;
        self.cache.put(
            did.to_string(),
            Arc::new(resolution_output.did_document().clone()),
        );
        Ok(resolution_output)
    }
}
