use actix_web::{web, App, HttpServer};
use adapters::{add_user, get_user};
use drivers::InMemoryUserRepository;
use use_cases::UserUseCase;

pub(crate) mod adapters;
pub(crate) mod drivers;
pub(crate) mod entities;
pub(crate) mod use_cases;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let user_repository = InMemoryUserRepository::new();
    let user_use_case = UserUseCase::new(&user_repository);

    HttpServer::new(move || {
        App::new()
            .data(user_use_case.clone())
            .route("/users", web::post().to(add_user))
            .route("/users/{user_id}", web::get().to(get_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
