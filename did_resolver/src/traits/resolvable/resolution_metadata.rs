use serde::{Deserialize, Serialize};

use super::resolution_error::DIDResolutionErrorType;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct DIDResolutionMetadata {
    content_type: Option<String>,
    error: Option<DIDResolutionErrorType>,
}
