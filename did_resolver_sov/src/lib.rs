pub mod error;

use aries_vcx_core::ledger::base_ledger::BaseLedger;

use did_resolver::{
    did_doc_builder::schema::{
        did_doc::{DIDDocument, DIDDocumentBuilder},
        service::ServiceBuilder,
        types::{did::Did, uri::Uri},
    },
    did_parser::ParsedDID,
    error::GenericError,
    resolvable::DIDResolvable,
};
use std::num::NonZeroUsize;
use std::sync::Arc;

use async_trait::async_trait;
use lru::LruCache;

struct DIDSovResolver {
    ledger: Box<dyn BaseLedger>,
    cache: LruCache<String, Arc<DIDDocument>>,
}

impl DIDSovResolver {
    pub fn new(ledger: Box<dyn BaseLedger>, cache_size: NonZeroUsize) -> Self {
        DIDSovResolver {
            ledger,
            cache: LruCache::new(cache_size),
        }
    }
}

#[async_trait]
impl DIDResolvable for DIDSovResolver {
    async fn resolve(&mut self, did: ParsedDID) -> Result<DIDDocument, GenericError> {
        if let Some(ddo) = self.cache.get(did.did()) {
            return Ok((**ddo).clone());
        }
        let service_endpoint = self.ledger.get_attr(did.did(), "service").await?;
        let service_id = Uri::new(did.did().to_string())?;
        let ddo_did = Did::new(did.did().to_string())?;
        let ddo = Arc::new(
            DIDDocumentBuilder::new(ddo_did)
                .add_service(
                    ServiceBuilder::new(service_id, service_endpoint)
                        .add_type("endpoint".to_string())
                        .build()?,
                )
                .build(),
        );
        self.cache.put(did.did().to_string(), ddo.clone());
        Ok((*ddo).clone())
    }
}

#[cfg(test)]
mod tests {
    use std::future::Future;

    use super::*;
    use aries_vcx_core::{
        global::settings,
        indy::ledger::pool::{
            create_pool_ledger_config, open_pool_ledger,
            test_utils::{create_tmp_genesis_txn_file, delete_test_pool, open_test_pool},
        },
        utils,
    };

    struct SetupProfile {
        pub institution_did: String,
        pub profile: Arc<dyn Profile>,
        pub(self) teardown: Arc<dyn Fn() -> BoxFuture<'static, ()>>,
    }

    fn init_test_logging() {}

    impl SetupProfile {
        pub async fn init() -> SetupProfile {
            init_test_logging();
            set_test_configs();
            SetupProfile::init_indy().await
        }

        async fn init_indy() -> SetupProfile {
            let (institution_did, wallet_handle) = setup_issuer_wallet().await;

            settings::set_config_value(
                settings::CONFIG_GENESIS_PATH,
                utils::get_temp_dir_path(settings::DEFAULT_GENESIS_PATH)
                    .to_str()
                    .unwrap(),
            )
            .unwrap();
            let pool_handle = open_test_pool().await;

            let profile: Arc<dyn Profile> =
                Arc::new(VdrtoolsProfile::new(wallet_handle, pool_handle.clone()));

            async fn indy_teardown(pool_handle: PoolHandle) {
                delete_test_pool(pool_handle.clone()).await;
            }

            SetupProfile {
                institution_did,
                profile,
                teardown: Arc::new(move || Box::pin(indy_teardown(pool_handle))),
            }
        }

        pub async fn run<F>(f: impl FnOnce(Self) -> F)
        where
            F: Future<Output = ()>,
        {
            let init = Self::init().await;

            let teardown = Arc::clone(&init.teardown);

            f(init).await;

            (teardown)().await;

            reset_global_state();
        }
    }

    #[test]
    fn it_works() {
        SetupProfile::run(|init| async move {
            let mut resolver =
                DIDSovResolver::new(Box::new(init.profile), NonZeroUsize::new(10).unwrap());
            let did = ParsedDID::new("did:sov:WRfXPg8dantKVubE3HX8pw").unwrap();
            let ddo = resolver.resolve(did).await.unwrap();
            assert_eq!(ddo.id(), "did:sov:WRfXPg8dantKVubE3HX8pw");
            assert_eq!(ddo.services.len(), 1);
            assert_eq!(ddo.services[0].id, "did:sov:WRfXPg8dantKVubE3HX8pw");
            assert_eq!(ddo.services[0].r#type, "indy");
            assert_eq!(ddo.services[0].service_endpoint, "http://example.com");
        })
    }
}
