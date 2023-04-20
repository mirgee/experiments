use aries_vcx_core::ledger::base_ledger::BaseLedger;
use chrono::{DateTime, NaiveDateTime, Utc};

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
    shared_types::did_document_metadata::DIDDocumentMetadata,
    traits::resolvable::{
        resolution_metadata::DIDResolutionMetadata, resolution_options::DIDResolutionOptions,
        resolution_output::DIDResolutionOutput, DIDResolvable,
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
            return Ok(DIDResolutionOutput::builder((**ddo).clone()).build());
        }
        let ledger_response = self.ledger.get_attr(did.did(), "endpoint").await?;
        let resolution_output = resolve_ddo(&did, &ledger_response).await?;
        self.cache.put(
            did.did().to_string(),
            Arc::new(resolution_output.did_document().clone()),
        );
        Ok(resolution_output)
    }
}

async fn resolve_ddo(did: &ParsedDID, resp: &str) -> Result<DIDResolutionOutput, DIDSovError> {
    fn prepare_ids(did: &ParsedDID) -> Result<(Uri, Did), DIDSovError> {
        let service_id = Uri::new(did.did().to_string())?;
        let ddo_id = Did::new(did.did().to_string())?;
        Ok((service_id, ddo_id))
    }

    fn get_data_from_response(resp: &str) -> Result<Value, DIDSovError> {
        let resp: serde_json::Value = serde_json::from_str(resp)?;
        serde_json::from_str(resp["result"]["data"].as_str().unwrap_or("{}"))
            .map_err(|err| err.into())
    }

    fn get_txn_time_from_response(resp: &str) -> Result<i64, DIDSovError> {
        let resp: serde_json::Value = serde_json::from_str(resp)?;
        let txn_time = resp["result"]["txnTime"].as_i64().ok_or(DIDSovError)?;
        Ok(txn_time)
    }

    fn posix_to_datetime(posix_timestamp: i64) -> Option<DateTime<Utc>> {
        if let Some(date_time) = NaiveDateTime::from_timestamp_opt(posix_timestamp, 0) {
            Some(DateTime::<Utc>::from_utc(date_time, Utc))
        } else {
            None
        }
    }

    let (service_id, ddo_id) = prepare_ids(did)?;

    let service_data = get_data_from_response(&resp)?;
    let endpoint: EndpointDidSov = serde_json::from_value(service_data["endpoint"].clone())?;

    let txn_time = get_txn_time_from_response(&resp)?;
    let datetime = posix_to_datetime(txn_time);

    let service = {
        let mut service_builder = ServiceBuilder::new(service_id, endpoint.endpoint);
        for t in endpoint.types {
            if t != DidSovServiceType::Unknown {
                service_builder = service_builder.add_type(t.to_string());
            };
        }
        service_builder.build()?
    };

    let ddo = DIDDocumentBuilder::new(ddo_id).add_service(service).build();

    let ddo_metadata = {
        let mut metadata_builder = DIDDocumentMetadata::builder().deactivated(false);
        if let Some(datetime) = datetime {
            metadata_builder = metadata_builder.updated(datetime);
        };
        metadata_builder.build()
    };

    let resolution_metadata = DIDResolutionMetadata::builder()
        .content_type("application/did+json".to_string())
        .build();

    Ok(DIDResolutionOutput::builder(ddo)
        .did_document_metadata(ddo_metadata)
        .did_resolution_metadata(resolution_metadata)
        .build())
}
