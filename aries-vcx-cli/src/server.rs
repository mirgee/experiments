use actix_web::dev::Server;

use crate::configuration::AppConfig;

fn build_server(_config: &AppConfig) -> anyhow::Result<Server> {
    todo!()
}

pub async fn run_server() -> anyhow::Result<()> {
    todo!()
}
