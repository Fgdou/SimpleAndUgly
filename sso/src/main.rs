mod errors;
mod objects;
mod repositories;
mod services;
mod views;
mod login_middleware;
mod app_state;

use std::ops::Deref;
use actix_web::body::MessageBody;
use actix_web::dev::Service;
use actix_web::middleware::from_fn;
use actix_web::{get, web, App, HttpServer, Responder};
use maud::html;
use serde::Deserialize;
use crate::app_state::AppState;

#[get("/echo")]
async fn echo() -> impl Responder {
    "Hello World !"
}

#[get("/")]
async fn home(state: web::Data<AppState>) -> impl Responder {
    let user = state.user.lock().unwrap();
    let name = match user.deref() {
        None => "Anonymous",
        Some(user) => &user.name
    };

    html! {
        "Hello " (name)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let services = AppState::new("/tmp/db.sqlite");
        App::new()
            .service(echo)
            .service(home)
            .service(views::auth::login)
            .service(views::auth::login_post)
            .service(views::auth::logout)
            .service(views::users::apply_scope(web::scope("/users")))
            .app_data(web::Data::new(services))
            .wrap(from_fn(login_middleware::login_middleware))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}