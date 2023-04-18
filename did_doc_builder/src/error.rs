#[derive(Debug)]
pub enum DIDDocumentBuilderError {
    InvalidInput(String),
}

impl std::fmt::Display for DIDDocumentBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DIDDocumentBuilderError::InvalidInput(field) => {
                write!(f, "Invalid input: {}", field)
            }
        }
    }
}

impl std::error::Error for DIDDocumentBuilderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DIDDocumentBuilderError::InvalidInput(_) => None,
        }
    }
}
