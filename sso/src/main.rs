mod errors;
mod objects;
mod repositories;
mod services;

use crate::errors::login::{AuthError, LoginError};
use crate::objects::user::User;
use crate::repositories::tokens::TokenRepo;
use crate::repositories::users::UserRepo;
use crate::services::auth::Auth;
use actix_web::body::MessageBody;
use actix_web::cookie::time::OffsetDateTime;
use actix_web::cookie::Cookie;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::middleware::{from_fn, Next};
use actix_web::{get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use maud::{html, Markup, PreEscaped};
use rusqlite::{Connection, OpenFlags};
use serde::Deserialize;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

async fn login_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    if req.path() != "/login" {
        let state: &web::Data<AppState> = req.app_data().unwrap();

        let cookie = match req.cookie("token") {
            None => return Err(Error::from(AuthError::TokenNotExist)),
            Some(cookie) => cookie
        };
        let user = state.auth.authenticate(cookie.value())?;
        *state.user.lock().unwrap() = Some(user);
    }
    let next = Rc::new(next);
    Ok(next.call(req).await?)
}

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

#[get("/login")]
async fn login() -> impl Responder {
    login_content(None)
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}
#[post("/login")]
async fn login_post(body: web::Form<LoginRequest>, state: web::Data<AppState>) -> impl Responder {
    let token = state.auth.login(&body.email, &body.password);

    let mut response = HttpResponse::Ok();

    match token {
        Err(e) => response.content_type("text/html")
            .body(login_content(Some(e))),
        Ok(token) => {

            let content = html! {
                (PreEscaped("<script>setTimeout(() => window.location.replace(\"/\"), 3000)</script>"))
                "You are now connected !"
            };

            response.cookie(
                Cookie::build("token", token.value)
                    .domain("localhost")
                    .path("/")
                    .expires(Some(
                        OffsetDateTime::from_unix_timestamp(token.expiration.unwrap().timestamp())
                            .unwrap()
                    ))
                    .finish()
            )
                .body(content)
        }
    }
}
#[get("/logout")]
async fn logout(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    state.auth.token_repo.invalidate_token(req.cookie("token").unwrap().value()).unwrap();
    HttpResponse::Ok()
        .cookie(
            Cookie::build("token", "")
                .expires(OffsetDateTime::now_utc())
                .finish()
        )
        .body(html! {
            (PreEscaped("<script>setTimeout(() => window.location.replace(\"/\"), 5000)</script>"))
            "You are now logged out"
        })
}

fn login_content(error: Option<LoginError>) -> Markup {
    html!(
        @if let Some(error) = error {
            "Error : " @match error {
                LoginError::InvalidEmail => "Invalid Email",
                LoginError::InvalidPassword => "Invalid Password"
            };
            br;
        }
        form action="/login" method="post" {
            input type="text" name="email" placeholder="email";
            br;
            input type="password" name="password" placeholder="password";
            br;
            button type="submit" {
                "Login"
            };
        }
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let services = AppState::new("/tmp/db.sqlite");
        App::new()
            .service(echo)
            .service(login)
            .service(login_post)
            .service(logout)
            .app_data(web::Data::new(services))
            .wrap(from_fn(login_middleware))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}