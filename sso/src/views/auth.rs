use crate::errors::login::LoginError;
use crate::AppState;
use actix_web::cookie::time::OffsetDateTime;
use actix_web::cookie::Cookie;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, Responder, Scope};
use maud::{html, Markup, PreEscaped};
use serde::Deserialize;
use std::ops::Deref;
use crate::errors::register::RegisterError;
use crate::services::auth::UserRequest;

fn forward(msg: &str, path: &str, timeout: u32) -> Markup {
    html! {
        (PreEscaped(format!("<script>setTimeout(() => window.location.replace(\"{path}\"), {timeout})</script>")))
        (msg) ". Redirecting..."
    }
}

#[get("/login")]
pub async fn login() -> impl Responder {
    login_content(None)
}

#[get("/register")]
pub async fn register() -> impl Responder {
    register_form(None)
}

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    name: String,
    password: String,
    token: String,
}
#[post("/register")]
pub async fn register_post(state: web::Data<AppState>, data: web::Form<RegisterRequest>) -> impl Responder {
    let data = data.into_inner();
    let response = state.auth.register(&data.token, UserRequest {
        email: data.email,
        name: data.name,
        password: data.password,
    });

    match response {
        Err(e) => register_form(Some(e)),
        Ok(()) => {
            forward("User created", "/login", 1000)
        }
    }
}

fn register_form(error: Option<RegisterError>) -> Markup {
    html!(
        h1 {"Register"}

        @if let Some(error) = error {
            "Error : " (error.to_string());
            br;
        }

        form action="/register" method="post" {
            input name="token" placeholder="Register Token" type="text";
            br;
            input name="email" placeholder="Email" type="email";
            br;
            input name="name" placeholder="Name" type="text";
            br;
            input name="password" placeholder="Password" type="password";
            br;
            button type="submit" {"Register"}
        }
    )
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}
#[post("/login")]
pub async fn login_post(body: web::Form<LoginRequest>, state: web::Data<AppState>) -> impl Responder {
    let token = state.auth.login(&body.email, &body.password);

    let mut response = HttpResponse::Ok();

    match token {
        Err(e) => response.content_type("text/html")
            .body(login_content(Some(e))),
        Ok(token) => {

            let content = forward("You are now connected", "/", 1000);

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
pub async fn logout(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    state.auth.token_repo.invalidate_token(req.cookie("token").unwrap().value()).unwrap();
    HttpResponse::Ok()
        .cookie(
            Cookie::build("token", "")
                .expires(OffsetDateTime::now_utc())
                .finish()
        )
        .body(forward("You are not disconnected", "/", 1000))
}

fn login_content(error: Option<LoginError>) -> Markup {
    html!(
        h1 {"Login"}
        @if let Some(error) = error {
            "Error : " (error.to_string());
            br;
        }
        form action="/login" method="post" {
            input type="email" name="email" placeholder="email";
            br;
            input type="password" name="password" placeholder="password";
            br;
            button type="submit" {
                "Login"
            };
        }
    )
}