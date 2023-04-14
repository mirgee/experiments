pub mod error;

use aries_vcx_core::ledger::base_ledger::BaseLedger;

use did_resolver::{
    did_doc_builder::{DIDDocument, Service},
    did_parser::ParsedDID,
    error::GenericError,
    resolvable::DIDResolvable,
};
use error::DIDSovError;
use std::sync::Arc;
use std::{error::Error, num::NonZeroUsize};

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
        let ddo = Arc::new(DIDDocument::new(did.did()).add_service(Service {
            id: did.did().to_string(),
            r#type: "indy".to_string(),
            service_endpoint,
        }));
        self.cache.put(did.did().to_string(), ddo.clone());
        Ok((*ddo).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aries_vcx_core::indy::ledger::pool::{
        create_pool_ledger_config, open_pool_ledger,
        test_utils::{create_tmp_genesis_txn_file, delete_test_pool, open_test_pool},
    };

    struct SetupProfile {
        pub institution_did: String,
        pub profile: Arc<dyn Profile>,
        pub(self) teardown: Arc<dyn Fn() -> BoxFuture<'static, ()>>,
    }

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
    fn it_works() {}
}
