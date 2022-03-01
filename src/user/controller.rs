use super::service::UserService;
use crate::{models::user::CreateUserDTO, user::model::User};
use actix_web::{web, HttpResponse, Responder};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/{user_id}", web::get().to(get_user))
        .route("/", web::get().to(get_users))
        .route("/", web::post().to(create_user));
}

async fn get_user(
    user_service: web::Data<UserService>,
    web::Path(user_id): web::Path<String>,
) -> impl Responder {
    // let user = web::block(move || user_service.get_user(&user_id)).await;

    // match user {
    //     Ok(user) => HttpResponse::Ok().body(format!("Got user with id: {}", user.id)),
    //     _ => HttpResponse::NotFound().finish(),
    // }
    HttpResponse::NotFound().finish()
}

async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("Get All users!")
}

async fn create_user(
    user_service: web::Data<UserService>,
    user: web::Json<CreateUserDTO>,
) -> impl Responder {
    println!("Create User: {:#?}", user);
    let user = web::block(move || {
        let _user = User::new();
        user_service.create_user()
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    });

    match user {
        Ok(user) => HttpResponse::Ok().body(format!("User {} created", &user.id)),
        _ => HttpResponse::InternalServerError().finish(),
    }
}
