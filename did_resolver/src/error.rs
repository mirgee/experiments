use std::error::Error;

pub trait GenericError: Error + Send + Sync + 'static {}

#[derive(Debug)]
pub enum DIDResolverError {
    InvalidDID,
    UnsupportedMethod,
}

impl std::fmt::Display for DIDResolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DIDResolverError::InvalidDID => write!(f, "Invalid DID"),
            DIDResolverError::UnsupportedMethod => write!(f, "Unsupported DID method"),
        }
    }
}

impl GenericError for DIDResolverError {}
impl GenericError for Box<dyn GenericError> {}

impl Error for DIDResolverError {}
impl Error for Box<dyn GenericError> {}
