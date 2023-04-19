use super::dereferencing_error::DIDDereferencingErrorType;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DIDDereferencingMetadata {
    content_type: Option<String>,
    error: Option<DIDDereferencingErrorType>,
}
