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
    use std::{future::Future, pin::Pin};

    use super::*;
    use aries_vcx_core::{
        indy::{
            ledger::pool::test_utils::{delete_test_pool, open_test_pool},
            wallet::{
                create_wallet_with_master_secret, open_wallet, wallet_configure_issuer,
                WalletConfig,
            },
        },
        ledger::indy_ledger::IndySdkLedger,
        PoolHandle, WalletHandle,
    };

    type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

    struct SetupProfile {
        pub institution_did: String,
        pub ledger: Arc<dyn BaseLedger>,
        pub(self) teardown: Arc<dyn Fn() -> BoxFuture<'static, ()>>,
    }

    fn init_test_logging() {
        env_logger::init();
    }

    pub async fn setup_issuer_wallet() -> (String, WalletHandle) {
        let enterprise_seed = "000000000000000000000000Trustee1";
        let config_wallet = WalletConfig {
            wallet_name: format!("wallet_{}", uuid::Uuid::new_v4().to_string()),
            wallet_key: "8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY".into(),
            wallet_key_derivation: "RAW".into(),
            wallet_type: None,
            storage_config: None,
            storage_credentials: None,
            rekey: None,
            rekey_derivation_method: None,
        };
        create_wallet_with_master_secret(&config_wallet)
            .await
            .unwrap();
        let wallet_handle = open_wallet(&config_wallet).await.unwrap();
        let config_issuer = wallet_configure_issuer(wallet_handle, enterprise_seed)
            .await
            .unwrap();
        // init_issuer_config(&config_issuer.institution_did).unwrap();
        (config_issuer.institution_did, wallet_handle)
    }

    impl SetupProfile {
        pub async fn init() -> SetupProfile {
            init_test_logging();

            let (institution_did, wallet_handle) = setup_issuer_wallet().await;

            // settings::set_config_value(
            //     settings::CONFIG_GENESIS_PATH,
            //     utils::get_temp_dir_path(settings::DEFAULT_GENESIS_PATH)
            //         .to_str()
            //         .unwrap(),
            // )
            // .unwrap();
            let pool_handle = open_test_pool().await;

            let ledger = Arc::new(IndySdkLedger::new(wallet_handle, pool_handle));

            async fn indy_teardown(pool_handle: PoolHandle) {
                delete_test_pool(pool_handle.clone()).await;
            }

            SetupProfile {
                institution_did,
                ledger,
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

            // reset_global_state();
        }
    }

    #[test]
    fn it_works() {
        SetupProfile::run(|init| async move {});
    }
}
