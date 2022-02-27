#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::sync::Arc;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use infra::postgres;
use user::controller as UserController;
use user::repository::UserRepository;
use user::service::UserService;

mod infra;
mod models;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = postgres::db::create_pool();

    let user_repo = UserRepository::new(pool.clone());
    let user_service = UserService::new(user_repo);

    HttpServer::new(move || {
        let user_service = web::Data::new(user_service);
        App::new()
            .app_data(user_service.clone())
            .service(web::scope("/user").configure(UserController::configure))
            .service(echo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
