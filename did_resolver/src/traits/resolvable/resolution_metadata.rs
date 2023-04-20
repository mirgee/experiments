use serde::{Deserialize, Serialize};

use super::resolution_error::DIDResolutionErrorType;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct DIDResolutionMetadata {
    content_type: Option<String>,
    error: Option<DIDResolutionErrorType>,
}

impl DIDResolutionMetadata {
    pub fn builder() -> DIDResolutionMetadataBuilder {
        DIDResolutionMetadataBuilder::default()
    }
}

#[derive(Default)]
pub struct DIDResolutionMetadataBuilder {
    content_type: Option<String>,
    error: Option<DIDResolutionErrorType>,
}

impl DIDResolutionMetadataBuilder {
    pub fn content_type(mut self, content_type: String) -> Self {
        self.content_type = Some(content_type);
        self
    }

    pub fn error(mut self, error: DIDResolutionErrorType) -> Self {
        self.error = Some(error);
        self
    }

    pub fn build(self) -> DIDResolutionMetadata {
        DIDResolutionMetadata {
            content_type: self.content_type,
            error: self.error,
        }
    }
}
