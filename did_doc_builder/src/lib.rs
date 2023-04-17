extern crate serde;
extern crate serde_json;

pub(crate) mod did;
pub(crate) mod did_doc;
pub(crate) mod did_doc_builder;
pub(crate) mod did_url;
pub(crate) mod service;
pub(crate) mod uri;
pub(crate) mod utils;
pub(crate) mod verification_method;

pub use did_doc::DIDDocument;
