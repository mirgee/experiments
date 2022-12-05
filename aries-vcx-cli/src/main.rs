#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

mod agent;
mod configuration;
mod server;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let config = configuration::configure_app().await?;
    let _agent = agent::initialize_agent(&config).await?;
    todo!()
}
