pub mod error;

use std::collections::HashMap;

use did_resolver::{
    did_parser::ParsedDID,
    error::GenericError,
    traits::resolvable::{
        resolution_options::DIDResolutionOptions, resolution_output::DIDResolutionOutput,
        DIDResolvable,
    },
};
use error::DIDResolverRegistryError;

pub struct ResolverRegistry {
    resolvers: HashMap<String, Box<dyn DIDResolvable>>,
}

impl ResolverRegistry {
    pub fn new() -> Self {
        ResolverRegistry {
            resolvers: HashMap::new(),
        }
    }

    pub fn register_resolver(&mut self, method: String, resolver: Box<dyn DIDResolvable>) {
        self.resolvers.insert(method, resolver);
    }

    pub fn unregister_resolver(&mut self, method: &str) {
        self.resolvers.remove(method);
    }

    pub async fn resolve(
        &mut self,
        did: ParsedDID,
        options: DIDResolutionOptions,
    ) -> Result<DIDResolutionOutput, GenericError> {
        let method = did.method();
        match self.resolvers.get_mut(method) {
            Some(resolver) => resolver.resolve(did, options).await,
            None => Err(Box::new(DIDResolverRegistryError::UnsupportedMethod)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use did_resolver::did_doc_builder::schema::did_doc::DIDDocumentBuilder;
    use mockall::{automock, predicate::*};
    use std::{error::Error, pin::Pin};

    struct DummyDIDResolver;

    #[async_trait]
    #[automock]
    impl DIDResolvable for DummyDIDResolver {
        async fn resolve(
            &mut self,
            did: ParsedDID,
            _options: DIDResolutionOptions,
        ) -> Result<DIDResolutionOutput, GenericError> {
            Ok(DIDResolutionOutput::builder(
                DIDDocumentBuilder::new(ParsedDID::parse(did.did().to_string()).unwrap()).build(),
            )
            .build())
        }
    }

    #[derive(Debug)]
    struct DummyResolverError;

    impl std::fmt::Display for DummyResolverError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Dummy resolver error")
        }
    }

    impl Error for DummyResolverError {}

    #[tokio::test]
    async fn test_resolve_error() {
        let did = ParsedDID::parse("did:example:1234".to_string()).unwrap();
        let method = did.method().to_string();

        let mut mock_resolver = MockDummyDIDResolver::new();
        mock_resolver
            .expect_resolve()
            .with(eq(did.clone()), eq(DIDResolutionOptions::default()))
            .times(1)
            .return_once(move |_, _| {
                let future = async move {
                    Err::<DIDResolutionOutput, GenericError>(Box::new(DummyResolverError))
                };
                Pin::from(Box::new(future))
            });

        let mut registry = ResolverRegistry::new();
        registry.register_resolver(method, Box::new(mock_resolver));

        let result = registry.resolve(did, DIDResolutionOptions::default()).await;
        assert!(result.is_err());

        let error = result.unwrap_err();
        if let Some(_) = error.downcast_ref::<DummyResolverError>() {
            assert!(true);
        } else {
            assert!(false, "Error is not of type DummyResolverError");
        }
    }
}
