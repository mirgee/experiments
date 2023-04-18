extern crate serde;
extern crate serde_json;

pub(crate) mod did;
pub(crate) mod did_doc;
pub(crate) mod did_url;
pub mod error;
pub(crate) mod service;
pub(crate) mod uri;
pub(crate) mod utils;
pub(crate) mod verification_method;

// TODO: Adjust visibility appropriately
pub use did_doc::DIDDocument;
// TODO: Consider either removing add_public_key_jwk or exposing a facade over jsonwebkey
pub use jsonwebkey;
pub use uri::Uri;
