use std::sync::{Arc, RwLock};

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

mod agent;
mod commands;
mod configuration;
mod server;
mod error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let config = configuration::configure_app().await?;
    let agent = agent::initialize_agent(&config).await?;
    let agentrc = Arc::new(RwLock::new(agent));
    let server = server::run_server(&config, agentrc.clone())?;
    let user_loop = commands::root_command_loop(agentrc);
    tokio::try_join![server, user_loop].map(|res| res.1).map_err(|err| anyhow!("Error: {}", err))
}
