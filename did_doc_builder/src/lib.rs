extern crate serde;
extern crate serde_json;

pub(crate) mod did;
pub(crate) mod did_doc;
pub(crate) mod did_url;
pub(crate) mod uri;
pub(crate) mod utils;

pub use did_doc::DIDDocument;
