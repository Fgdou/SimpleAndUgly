mod errors;
mod objects;
mod repositories;
mod services;

use crate::repositories::tokens::TokenRepo;
use crate::repositories::users::UserRepo;
use crate::services::auth::Auth;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result as AwResult};
use maud::{html, Markup};
use rusqlite::{Connection, OpenFlags};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use crate::errors::login::LoginError;

#[derive(Clone)]
struct Services {
    auth: Arc<Auth>
}
impl Services {
    pub fn new(path: &str) -> Self {
        let connection = Arc::new(Mutex::new(
            Connection::open_with_flags(
                path,
                OpenFlags::default() | OpenFlags::SQLITE_OPEN_FULL_MUTEX
            ).unwrap()
        ));
        let tokens = Arc::new(TokenRepo::new(connection.clone()));
        let users = Arc::new(UserRepo::new(connection.clone()));

        let auth = Arc::new(Auth {
            user_repo: users,
            token_repo: tokens,
        });

        Self {
            auth,
        }

    }
}

#[get("/echo")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body("Hello World !")
}

#[get("/login")]
async fn login() -> AwResult<Markup> {
    Ok(html!(
        form action="/login" method="post" {
            input type="text" name="email" placeholder="email";
            br;
            input type="password" name="password" placeholder="password";
            br;
            button type="submit" {
                "Login"
            };
        }
    ))
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}
#[post("/login")]
async fn login_post(body: web::Form<LoginRequest>, auth: web::Data<Auth>) -> impl Responder {
    Result::<_, LoginError>::Ok(
        web::Json(auth.login(&body.email, &body.password)?)
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let services = Services::new("/tmp/db.sqlite");
        App::new()
            .service(echo)
            .service(login)
            .service(login_post)
            .app_data(web::Data::new(services))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}