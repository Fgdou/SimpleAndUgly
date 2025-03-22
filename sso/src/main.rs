mod errors;
mod objects;
mod repositories;
mod services;
mod views;
mod login_middleware;
mod app_state;

use std::ops::Deref;
use actix_web::middleware::from_fn;
use actix_web::{get, web, App, HttpServer, Responder};
use crate::app_state::AppState;

#[get("/echo")]
async fn echo() -> impl Responder {
    "Hello World !"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let services = AppState::new("/tmp/db.sqlite");
        App::new()
            .service(echo)
            .service(views::home::home)
            .service(views::users::apply_scope(web::scope("/users")))
            .service(views::apps::apply_scope(web::scope("/apps")))
            .service(views::auth::register_post)
            .service(views::auth::register)
            .service(views::auth::login)
            .service(views::auth::login_post)
            .service(views::auth::logout)
            .app_data(web::Data::new(services))
            .wrap(from_fn(login_middleware::login_middleware))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}