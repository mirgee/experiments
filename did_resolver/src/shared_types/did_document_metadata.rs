use chrono::{DateTime, Utc};
use did_doc_builder::schema::types::did::Did;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DIDDocumentMetadata {
    created: Option<DateTime<Utc>>,
    updated: Option<DateTime<Utc>>,
    deactivated: Option<bool>,
    next_update: Option<DateTime<Utc>>,
    version_id: String,
    next_version_id: String,
    equivalent_id: Vec<Did>,
    canonical_id: Option<Did>,
}
