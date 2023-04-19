use aries_vcx_core::ledger::base_ledger::BaseLedger;

use crate::{
    error::DIDSovError,
    service::{DidSovServiceType, EndpointDidSov},
};
use did_resolver::{
    did_doc_builder::schema::{
        did_doc::{DIDDocument, DIDDocumentBuilder},
        service::ServiceBuilder,
        types::{did::Did, uri::Uri},
    },
    did_parser::ParsedDID,
    error::GenericError,
    traits::resolvable::{
        resolution_options::DIDResolutionOptions, resolution_output::DIDResolutionOutput,
        DIDResolvable,
    },
};
use serde_json::Value;
use std::num::NonZeroUsize;
use std::sync::Arc;

use async_trait::async_trait;
use lru::LruCache;

pub struct DIDSovResolver {
    ledger: Arc<dyn BaseLedger>,
    cache: LruCache<String, Arc<DIDDocument>>,
}

#[allow(dead_code)]
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
        did: ParsedDID,
        _options: DIDResolutionOptions,
    ) -> Result<DIDResolutionOutput, GenericError> {
        if let Some(ddo) = self.cache.get(did.did()) {
            return Ok(DIDResolutionOutput::new((**ddo).clone()));
        }
        let ddo = resolve_ddo(&did, self.ledger.clone()).await?;
        self.cache.put(did.did().to_string(), ddo.clone());
        Ok(DIDResolutionOutput::new((*ddo).clone()))
    }
}

async fn resolve_ddo(
    did: &ParsedDID,
    ledger: Arc<dyn BaseLedger>,
) -> Result<Arc<DIDDocument>, DIDSovError> {
    fn get_data_from_response(resp: &str) -> Result<Value, DIDSovError> {
        let resp: serde_json::Value = serde_json::from_str(resp)?;
        serde_json::from_str(resp["result"]["data"].as_str().unwrap_or("{}"))
            .map_err(|err| err.into())
    }

    let service_id = Uri::new(did.did().to_string())?;
    let ddo_did = Did::new(did.did().to_string())?;

    let service_data = get_data_from_response(&ledger.get_attr(did.did(), "endpoint").await?)?;
    let endpoint: EndpointDidSov = serde_json::from_value(service_data["endpoint"].clone())?;

    let service_builder = {
        let mut service_builder = ServiceBuilder::new(service_id, endpoint.endpoint);
        for t in endpoint.types {
            if t != DidSovServiceType::Unknown {
                service_builder = service_builder.add_type(t.to_string());
            };
        }
        service_builder
    };

    Ok(Arc::new(
        DIDDocumentBuilder::new(ddo_did)
            .add_service(service_builder.build()?)
            .build(),
    ))
}
