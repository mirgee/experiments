use aries_vcx_core::ledger::base_ledger::BaseLedger;
use did_resolver::{
    did_doc_builder::{DIDDocument, Service},
    did_parser::ParsedDID,
    error::DIDError,
    DIDResolver,
};
use std::num::NonZeroUsize;

use async_trait::async_trait;
use lru::LruCache;

struct IndyDID {
    ledger: Box<dyn BaseLedger>,
    cache: LruCache<String, DIDDocument>,
}

impl IndyDID {
    pub fn new(ledger: Box<dyn BaseLedger>, cache_size: NonZeroUsize) -> Self {
        IndyDID {
            ledger,
            cache: LruCache::new(cache_size),
        }
    }
}

#[async_trait]
impl DIDResolver for IndyDID {
    async fn resolve(&self, did: ParsedDID) -> Result<DIDDocument, DIDError> {
        if let Some(ddo) = self.cache.get(did.did()) {
            return Ok(*ddo);
        }
        let service_endpoint = self.ledger.get_attr(did.did(), "service").await?;
        let ddo = DIDDocument::new(did.did()).add_service(Service {
            id: did.did().to_string(),
            r#type: "indy".to_string(),
            service_endpoint,
        });
        self.cache.put(did.did().to_string(), ddo);
        Ok(ddo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
