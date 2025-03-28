use crate::app::app_state::AppState;
use crate::objects::config::Config;
use crate::views::auth::auth_middleware;
use crate::views::nav::get_nav;
use actix_web::body::MessageBody;
use actix_web::middleware::{from_fn, Next};
use actix_web::{get, web, App, Error, HttpServer};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header::{HeaderValue, CACHE_CONTROL};
use maud::{html, Markup};

mod errors;
mod objects;
mod repos;
mod services;
mod views;
mod forms;
mod apis;
mod app;

async fn cache(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // pre-processing
    let mut res = next.call(req).await?;
    // post-processing
    res.headers_mut()
        .insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    Ok(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState::new(&Config::default())))
            .wrap(from_fn(auth_middleware))
            .wrap(from_fn(cache))
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