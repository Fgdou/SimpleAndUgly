mod errors;
mod objects;
mod repositories;
mod services;
mod views;
mod login_middleware;

use crate::objects::user::User;
use crate::repositories::tokens::TokenRepo;
use crate::repositories::users::UserRepo;
use crate::services::auth::Auth;
use actix_web::body::MessageBody;
use actix_web::dev::Service;
use actix_web::middleware::from_fn;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use rusqlite::{Connection, OpenFlags};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct AppState {
    auth: Arc<Auth>,
    user: Arc<Mutex<Option<User>>>,
}
impl AppState {
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

        if let None = auth.user_repo.get_user_by_email("admin@example.com") {
            println!("Info: Creating Admin user");
            auth.user_repo.add_user(
                User::new("admin@example.com".to_string(), "Admin".to_string(), "admin".to_string())
            );
        }

        Self {
            auth,
            user: Arc::new(Mutex::new(None)),
        }

    }
}

#[get("/echo")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body("Hello World !")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let services = AppState::new("/tmp/db.sqlite");
        App::new()
            .service(echo)
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