use aries_vcx_agent::{Agent as AriesAgent, InitConfig, PoolInitConfig, WalletInitConfig};

pub async fn initialize(port: u32) -> AriesAgent {
    let enterprise_seed = get_trustee_seed().await;
    let genesis_path = download_genesis_file()
        .await
        .expect("Failed to download the genesis file");
    let service_endpoint = format!("http://{}:{}/didcomm", host, port);
    let init_config = InitConfig {
        enterprise_seed,
        pool_config: PoolInitConfig {
            genesis_path,
            pool_name: "pool_name".to_string(),
        },
        wallet_config: WalletInitConfig {
            wallet_name: format!("rust_agent_{}", uuid::Uuid::new_v4()),
            wallet_key: "8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY".to_string(),
            wallet_kdf: "RAW".to_string(),
        },
        agency_config: None,
        service_endpoint
    };
    AriesAgent::initialize(init_config).await.unwrap()
}
