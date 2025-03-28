use crate::app::app_state::AppState;
use crate::objects::config::Config;
use crate::views::auth::auth_middleware;
use crate::views::nav::get_nav;
use actix_web::body::MessageBody;
use actix_web::middleware::from_fn;
use actix_web::{get, web, App, HttpServer};
use maud::{html, Markup};

mod errors;
mod objects;
mod repos;
mod services;
mod views;
mod forms;
mod apis;
mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState::new(&Config::default())))
            .wrap(from_fn(auth_middleware))
            .service(hello_world)
            .service(home)
            .service(views::auth::get_scope())
    }).bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[get("/hello")]
async fn hello_world() -> &'static str {
    "Hello world"
}

#[get("/")]
async fn home(state: web::Data<AppState>) -> Markup {
    html! {
        (get_nav(&state))
        "Hello world"
    }
}