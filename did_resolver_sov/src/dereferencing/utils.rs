use std::io::Cursor;

use did_resolver::{
    did_doc_builder::schema::{
        did_doc::DIDDocument, service::Service, verification_method::VerificationMethod,
    },
    did_parser::ParsedDIDUrl,
    traits::{
        dereferenceable::{
            dereferencing_metadata::DIDDereferencingMetadata,
            dereferencing_output::DIDDereferencingOutput,
        },
        resolvable::resolution_output::DIDResolutionOutput,
    },
};

use crate::error::DIDSovError;

pub fn service_by_id<F>(services: &[Service], predicate: F) -> Option<&Service>
where
    F: Fn(&str) -> bool,
{
    services.iter().find(|svc| predicate(&svc.id()))
}

pub fn verification_by_id<F>(
    authentications: &[VerificationMethod],
    predicate: F,
) -> Option<&VerificationMethod>
where
    F: Fn(&str) -> bool,
{
    authentications
        .iter()
        .find(|auth| predicate(auth.id().did_url()))
}

fn content_stream_from(
    did_document: &DIDDocument,
    did_url: &ParsedDIDUrl,
) -> Result<Cursor<Vec<u8>>, DIDSovError> {
    let fragment = did_url.fragment().ok_or_else(|| {
        DIDSovError::InvalidDID("No fragment provided in the DID URL".to_string())
    })?;

    let did_url_string = did_url.to_string();
    let fragment_string = format!("#{}", fragment);
    let id_matcher = |id: &str| id == did_url_string || id.ends_with(&fragment_string);

    let value = match (
        service_by_id(&did_document.service(), id_matcher),
        verification_by_id(&did_document.verification_method(), id_matcher),
    ) {
        (Some(service), None) => serde_json::to_value(service)?,
        (None, Some(authentication)) => serde_json::to_value(authentication)?,
        (None, None) => {
            return Err(DIDSovError::InvalidDID(format!(
                "Fragment '{}' not found in the DID document",
                fragment
            )));
        }
        (Some(_), Some(_)) => {
            return Err(DIDSovError::InvalidDID(format!(
                "Fragment '{}' is ambiguous",
                fragment
            )));
        }
    };
    Ok(Cursor::new(value.to_string().into_bytes()))
}

pub(crate) fn dereference_did_document(
    resolution_output: DIDResolutionOutput,
    did_url: &ParsedDIDUrl,
) -> Result<DIDDereferencingOutput<Cursor<Vec<u8>>>, DIDSovError> {
    let content_stream = content_stream_from(resolution_output.did_document(), did_url)?;

    let content_metadata = resolution_output.did_document_metadata().clone();

    let dereferencing_metadata = DIDDereferencingMetadata::builder()
        .content_type("application/did+json".to_string())
        .build();

    Ok(DIDDereferencingOutput::builder(content_stream)
        .content_metadata(content_metadata)
        .dereferencing_metadata(dereferencing_metadata)
        .build())
}
