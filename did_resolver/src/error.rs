use std::error::Error;

pub type GenericError = Box<dyn Error + Send + Sync + 'static>;

#[derive(Debug)]
pub enum DIDResolverError {
    UnsupportedMethod,
}

impl std::fmt::Display for DIDResolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DIDResolverError::UnsupportedMethod => write!(f, "Unsupported DID method"),
        }
    }
}

impl Error for DIDResolverError {}
