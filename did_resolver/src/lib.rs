pub extern crate did_doc_builder;
pub extern crate did_parser;

pub mod error;

use std::collections::HashMap;

use async_trait::async_trait;
use did_doc_builder::DIDDocument;
use did_parser::ParsedDID;
use error::{DIDResolverError, GenericError};

#[async_trait]
pub trait DIDResolver {
    type DIDResolverError: GenericError;
    async fn resolve(&self, did: ParsedDID) -> Result<DIDDocument, Self::DIDResolverError>;
}

pub struct ResolverRegistry {
    resolvers: HashMap<String, Box<dyn DIDResolver<DIDResolverError = Box<dyn GenericError>>>>,
}

impl ResolverRegistry {
    pub fn new() -> Self {
        ResolverRegistry {
            resolvers: HashMap::new(),
        }
    }

    pub fn register_resolver(
        &mut self,
        method: String,
        resolver: Box<dyn DIDResolver<DIDResolverError = Box<dyn GenericError>>>,
    ) {
        self.resolvers.insert(method, resolver);
    }

    pub fn unregister_resolver(&mut self, method: &str) {
        self.resolvers.remove(method);
    }

    pub async fn resolve(&self, did: ParsedDID) -> Result<DIDDocument, Box<dyn GenericError>> {
        let method = did.method();
        match self.resolvers.get(method) {
            Some(resolver) => resolver.resolve(did).await,
            None => Err(Box::new(DIDResolverError::UnsupportedMethod)),
        }
    }
}
