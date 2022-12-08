mod api;
mod handlers;

use std::sync::{RwLock, Arc};

use actix_web::{dev::Server, middleware, HttpServer, App, web};
use aries_vcx_agent::Agent;

use crate::configuration::AppConfig;

pub fn run_server(config: &AppConfig, agent: Arc<RwLock<Agent>>) -> anyhow::Result<Server> {
    Ok(HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .app_data(web::Data::new(agent.clone()))
            .service(
                web::scope("/didcomm").route("", web::post().to(api::receive_message))
            )
    })
    .workers(1)
    .bind(format!("{}:{}", config.host(), config.port()))?
    .run())
}
