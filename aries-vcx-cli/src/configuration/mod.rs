mod app_config;
mod setup;

pub use app_config::AppConfig;

pub async fn configure_app() -> anyhow::Result<app_config::AppConfig> {
    let mut config = app_config::load_config();
    let trustee_seed = setup::get_trustee_seed(&config).await?;
    let genesis_file = setup::download_genesis_file(&config).await?;
    config.set_trustee_seed(trustee_seed);
    config.set_genesis_file(genesis_file);
    config.assure_agent_name();
    Ok(config)
}
