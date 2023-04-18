use std::str::FromStr;

use did_doc_builder::schema::{
    did_doc::DIDDocument,
    types::{did::Did, uri::Uri},
};

use serde_json::json;

// Test data: valid DID document JSON string
const VALID_DID_DOC_JSON: &str = r#"
{
    "id": "did:example:123456789abcdefghi",
    "alsoKnownAs": [
        "https://example.com/user-profile/123"
    ],
    "controller": "did:example:abc123",
    "verificationMethod": [
        {
            "id": "did:example:123456789abcdefghi#keys-1",
            "type": "Ed25519VerificationKey2018",
            "controller": "did:example:123456789abcdefghi",
            "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
        }
    ],
    "authentication": [
        "did:example:123456789abcdefghi#keys-1"
    ],
    "service": [
        {
            "id": "did:example:123456789abcdefghi#vcs",
            "type": "VerifiableCredentialService",
            "serviceEndpoint": "https://example.com/vc/"
        }
    ]
}
"#;

#[test]
fn test_uri() {
    let uri: Uri = "did:example:123456789abcdefghi#vcs".parse().unwrap();
    println!("{:?}", uri);
}

#[test]
fn test_deserialization() {
    let did_doc: DIDDocument = serde_json::from_str(VALID_DID_DOC_JSON).unwrap();

    assert_eq!(
        did_doc.id(),
        &Did::from_str("did:example:123456789abcdefghi").unwrap()
    );
    assert_eq!(
        did_doc.also_known_as(),
        vec![Uri::from_str("https://example.com/user-profile/123").unwrap()]
    );
}

#[test]
fn test_serialization() {
    let did_doc: DIDDocument = serde_json::from_str(VALID_DID_DOC_JSON).unwrap();

    // Serialize the DIDDocument struct
    let serialized_json = serde_json::to_string(&did_doc).unwrap();

    // Validate that the serialized JSON matches the original input JSON
    let original_json_value: DIDDocument = serde_json::from_str(VALID_DID_DOC_JSON).unwrap();
    let serialized_json_value: DIDDocument = serde_json::from_str(&serialized_json).unwrap();
    assert_eq!(serialized_json_value, original_json_value);
}
