use chrono::{DateTime, NaiveDateTime, Utc};
use did_resolver::{
    did_doc_builder::schema::{
        did_doc::DIDDocument,
        service::ServiceBuilder,
        types::{did::Did, uri::Uri},
    },
    did_parser::ParsedDID,
    shared_types::did_document_metadata::DIDDocumentMetadata,
    traits::resolvable::{
        resolution_metadata::DIDResolutionMetadata, resolution_output::DIDResolutionOutput,
    },
};
use serde_json::Value;

use crate::{
    error::DIDSovError,
    service::{DidSovServiceType, EndpointDidSov},
};

fn prepare_ids(did: &ParsedDID) -> Result<(Uri, Did), DIDSovError> {
    let service_id = Uri::new(did.did().to_string())?;
    let ddo_id = Did::new(did.did().to_string())?;
    Ok((service_id, ddo_id))
}

fn get_data_from_response(resp: &str) -> Result<Value, DIDSovError> {
    let resp: serde_json::Value = serde_json::from_str(resp)?;
    serde_json::from_str(resp["result"]["data"].as_str().unwrap_or("{}")).map_err(|err| err.into())
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

pub(super) async fn resolve_ddo(
    did: &ParsedDID,
    resp: &str,
) -> Result<DIDResolutionOutput, DIDSovError> {
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

    let ddo = DIDDocument::builder(ddo_id).add_service(service).build();

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
