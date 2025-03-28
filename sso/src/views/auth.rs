use actix_web::{get, post, web, HttpResponse, Scope};
use actix_web::body::MessageBody;
use actix_web::cookie::{Cookie, Expiration};
use actix_web::cookie::time::{OffsetDateTime, UtcDateTime};
use actix_web::http::StatusCode;
use maud::{html, Markup};
use crate::app::app_state::AppState;
use crate::forms::auth::LoginForm;
use crate::views::nav::get_nav;

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
}