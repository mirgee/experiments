use actix_web::{web, App, HttpResponse, Responder};
use serde::Deserialize;

use crate::use_cases::UserUseCase;

#[derive(Deserialize)]
pub struct AddUserRequest {
    name: String,
    email: String,
}

pub async fn add_user(
    user_use_case: web::Data<UserUseCase<'_>>,
    params: web::Json<AddUserRequest>,
) -> impl Responder {
    match user_use_case.add_user(params.name.clone(), params.email.clone()) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

pub async fn get_user(
    user_use_case: web::Data<UserUseCase<'_>>,
    user_id: web::Path<u32>,
) -> impl Responder {
    match user_use_case.get_user(user_id.into_inner()) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
    }
}
