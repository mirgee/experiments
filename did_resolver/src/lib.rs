extern crate did_doc_builder;
extern crate did_parser;

use std::{collections::HashMap, error::Error, num::NonZeroUsize};

use did_doc_builder::DIDDocument;
use did_parser::ParsedDID;
use lru::LruCache;

#[derive(Debug)]
enum DIDError {
    InvalidDID,
    UnsupportedMethod(String),
}

impl Error for DIDError {}

impl std::fmt::Display for DIDError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DIDError::InvalidDID => write!(f, "Invalid DID"),
            DIDError::UnsupportedMethod(_) => write!(f, "Unsupported DID method"),
        }
    }
}

trait BaseLedger {
    fn get_attr(&self, did: &str) -> Result<String, ()>;
}

struct IndyDID {
    ledger: Box<dyn BaseLedger>,
    cache: LruCache<String, DIDDocument>,
}

impl IndyDID {
    pub fn new(ledger: Box<dyn BaseLedger>, cache_size: NonZeroUsize) -> Self {
        IndyDID {
            ledger,
            cache: LruCache::new(cache_size),
        }
    }
}

struct SovDID {
    ledger: Box<dyn BaseLedger>,
    cache: LruCache<String, DIDDocument>,
}

impl SovDID {
    pub fn new(ledger: Box<dyn BaseLedger>, cache_size: NonZeroUsize) -> Self {
        SovDID {
            ledger,
            cache: LruCache::new(cache_size),
        }
    }
}

trait DIDResolver: Sync + Send {
    fn resolve(&self, did: ParsedDID) -> Result<DIDDocument, DIDError>;
}

pub struct ResolverRegistry {
    resolvers: HashMap<String, Box<dyn DIDResolver>>,
}

impl ResolverRegistry {
    fn new() -> Self {
        ResolverRegistry {
            resolvers: HashMap::new(),
        }
    }

    pub fn register_resolver(&mut self, method: String, resolver: Box<dyn DIDResolver>) {
        self.resolvers.insert(method, resolver);
    }

    pub fn unregister_resolver(&mut self, method: &str) {
        self.resolvers.remove(method);
    }

    pub fn resolve(&self, did: ParsedDID) -> Result<DIDDocument, DIDError> {
        let method = did.method;
        match self.resolvers.get(method) {
            Some(resolver) => resolver.resolve(did),
            None => Err(DIDError::UnsupportedMethod(method.to_string())),
        }
    }
}
