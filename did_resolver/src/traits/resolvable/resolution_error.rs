use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum DIDResolutionErrorType {
    InvalidDid,
    NotFound,
    RepresentationNotSupported,
    MethodNotSupported,
    InternalError,
    InvalidPublicKey,
    InvalidPublicKeyLength,
    InvalidPublicKeyType,
    UnsupportedPublicKeyType,
    NotAllowedVerificationMethodType,
    NotAllowedKeyType,
    NotAllowedMethod,
    NotAllowedCertificate,
    NotAllowedLocalDuplicateKey,
    NotAllowedLocalDerivedKey,
    NotAllowedGlobalDuplicateKey,
}
