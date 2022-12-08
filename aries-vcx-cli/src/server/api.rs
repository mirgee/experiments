use std::sync::{RwLock, Arc};

use actix_web::{web, HttpResponse};
use aries_vcx_agent::Agent;

use crate::error::e500;
use super::handlers::handle_message;

pub async fn receive_message(
    req: web::Bytes,
    agent: web::Data<Arc<RwLock<Agent>>>,
) -> Result<HttpResponse, actix_web::Error> {
    match agent.read() {
        Ok(agent_guard) => {
            handle_message(agent_guard, req.to_vec()).await.map_err(e500)?;
            Ok(HttpResponse::Ok().finish())
        }
        Err(err) => {
            error!("Failed to acquire read lock on agent: {:?}", err);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
