use std::os::linux::raw::stat;
use actix_web::{get, post, web, Error, HttpResponse, Scope};
use actix_web::body::{BoxBody, MessageBody};
use actix_web::cookie::{Cookie, Expiration};
use actix_web::cookie::time::{OffsetDateTime, UtcDateTime};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::middleware::Next;
use maud::{html, Markup};
use crate::app::app_state::AppState;
use crate::forms::auth::LoginForm;
use crate::views::nav::get_nav;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let excluded_paths = [
        "/auth/login",
        "/auth/register",
        "/"
    ];

    let excluded = excluded_paths.contains(&req.path());
    let cookie = req.cookie("token");

    if let Some(cookie) = cookie {
        let value = cookie.value();

        let state = req.app_data::<web::Data<AppState>>().unwrap();
        let user = state.services.auth.authenticate(value);

        match (excluded, user) {
            (_, Ok(user)) => { *state.user.lock().unwrap() = Some((value.to_string(), user)); }
            (false, Err(e)) => {
                let content = html! {
                    script {"window.location.replace('/auth/login')"}
                    div { (e) }
                };

                return Ok(ServiceResponse::new(req.request().clone(), HttpResponse::build(StatusCode::UNAUTHORIZED)
                    .body(content)));
            }
            _ => {},
        }
    }
    // pre-processing
    next.call(req).await
    // post-processing
}

#[get("logout")]
async fn logout(state: web::Data<AppState>) -> HttpResponse {
    if let Some(token) = state.user.lock().unwrap().as_ref() {
        state.services.auth.invalidate_token(&token.0);
    }
    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(html!{
            "You have been disconnected";
            script {"window.location.replace('/')"}
        })
}

#[post("/login")]
async fn login(state: web::Data<AppState>, form: web::Form<LoginForm>) -> HttpResponse {
    let response = state.services.auth.login(&form);

    let (cookie, body) = match response {
        Ok(token) => {
            let content = html! {
                "You are connected, redirecting..."
                script {"window.location.replace('/')"}
            };
            let cookie = Cookie::build("token", token.value)
                .path("/")
                .expires(
                    Expiration::DateTime(
                        OffsetDateTime::from(
                            UtcDateTime::from_unix_timestamp(token.expiration.timestamp()).unwrap()
                        )
                    )
                )
                .finish();
            (Some(cookie), content)
        },
        Err(e) => {
            let content = html! {
                ("Error : ") (e)
            };

            (None, content)
        }
    };

    let mut builder = HttpResponse::build(StatusCode::OK)
        .body(body);
    if let Some(cookie) = cookie {
        let _ = builder.add_cookie(&cookie);
    }
    builder
}

#[get("/login")]
async fn login_page(state: web::Data<AppState>) -> Markup {
    html! {
        (get_nav(&state))
        div {}
        form hx-post="/auth/login" hx-target="previous" {
            input type="email" name="email" placeholder="user@example.com";
            br;
            input type="password" name="password" placeholder="Password";
            br;
            button type="submit" {"Login"}
        }
    }
}

pub fn get_scope() -> Scope {
    web::scope("/auth")
        .service(login_page)
        .service(login)
        .service(logout)
}