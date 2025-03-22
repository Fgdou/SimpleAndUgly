use std::ops::Deref;
use actix_web::{get, web, Responder};
use maud::{html, Markup, PreEscaped};
use crate::app_state::AppState;
use crate::objects::user::User;

pub fn get_navbar(user: Option<&User>, page_name: &str) -> Markup {
    html! {
        head {
            (PreEscaped(r#"<style>
            body {
                display: inline-block;
                margin: 10% auto;
                text-align: left;
            }
            html {
                text-align: center;
            }
            </style>"#));

            title {"SSO" @if let Some(user) = user {
                @let name = &user.name;
                (format!(" - {name}"))
            }}
        }
        nav {
            a href="/" {"Home"} "|"
            @if let Some(user) = user {
                a href="/users/" {"Users"} "|"
                a href="/apps/" {"Applications"} "|"
                a href="/logout" {"Logout"}
            } @else {
                a href="/login" {"Login"} "|"
                a href="/register" {"Register"}
            }
        }

        hr;

        h1 {(page_name)}
    }
}

#[get("/")]
pub async fn home(state: web::Data<AppState>) -> impl Responder {
    let user = state.user.lock().unwrap();
    let name = match user.deref() {
        None => "Anonymous",
        Some(user) => &user.name
    };

    html! {
        (get_navbar(user.as_ref(), "Home"))

        "Hello " (name)
    }
}