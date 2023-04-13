pub mod error;

use aries_vcx_core::ledger::base_ledger::BaseLedger;

use did_resolver::{
    did_doc_builder::{DIDDocument, Service},
    did_parser::ParsedDID,
    error::GenericError,
    resolvable::DIDResolvable,
};
use error::DIDSovError;
use std::sync::Arc;
use std::{error::Error, num::NonZeroUsize};

use async_trait::async_trait;
use lru::LruCache;

struct DIDSovResolver {
    ledger: Box<dyn BaseLedger>,
    cache: LruCache<String, Arc<DIDDocument>>,
}

impl DIDSovResolver {
    pub fn new(ledger: Box<dyn BaseLedger>, cache_size: NonZeroUsize) -> Self {
        DIDSovResolver {
            ledger,
            cache: LruCache::new(cache_size),
        }
    }
}

#[async_trait]
impl DIDResolvable for DIDSovResolver {
    async fn resolve(&mut self, did: ParsedDID) -> Result<DIDDocument, GenericError> {
        if let Some(ddo) = self.cache.get(did.did()) {
            return Ok((**ddo).clone());
        }
        let service_endpoint = self.ledger.get_attr(did.did(), "service").await?;
        let ddo = Arc::new(DIDDocument::new(did.did()).add_service(Service {
            id: did.did().to_string(),
            r#type: "indy".to_string(),
            service_endpoint,
        }));
        self.cache.put(did.did().to_string(), ddo.clone());
        Ok((*ddo).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aries_vcx_core::indy::ledger::pool::open_pool_ledger;

    #[test]
    fn it_works() {}
}
