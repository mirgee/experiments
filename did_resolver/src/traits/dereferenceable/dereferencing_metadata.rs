use super::dereferencing_error::DIDDereferencingError;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DIDDereferencingMetadata {
    content_type: Option<String>,
    error: Option<DIDDereferencingError>,
}
