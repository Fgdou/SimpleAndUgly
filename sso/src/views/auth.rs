use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use actix_web::cookie::Cookie;
use actix_web::cookie::time::OffsetDateTime;
use maud::{html, Markup, PreEscaped};
use serde::Deserialize;
use crate::AppState;
use crate::errors::login::LoginError;

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
pub async fn login_post(body: web::Form<LoginRequest>, state: web::Data<AppState>) -> impl Responder {
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
pub async fn logout(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
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