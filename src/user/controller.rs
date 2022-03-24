use std::ops::Deref;

use crate::user::dto::CreateUserDTO;

use super::service::UserService;

use actix_web::{web, HttpResponse, Responder};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/{user_id}", web::get().to(get_user))
        .route("/", web::get().to(get_users))
        .route("/", web::post().to(create_user));
}

async fn get_user(
    user_service: web::Data<UserService>,
    web::Path(user_id): web::Path<i32>,
) -> impl Responder {
    let user = web::block(move || user_service.get_user(user_id)).await;

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::NotFound().finish(),
    }
}

async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("Get All users!")
}

async fn create_user(
    user_service: web::Data<UserService>,
    create_user_dto: web::Json<CreateUserDTO>,
) -> impl Responder {
    println!("Create User DTO: {:#?}", create_user_dto);
    let user = web::block(move || user_service.create_user(create_user_dto.deref())).await;

    match user {
        Ok(user) => HttpResponse::Ok().body(format!("User {} created", &user.id)),
        _ => HttpResponse::BadRequest().body("Email already in use"),
    }
}
