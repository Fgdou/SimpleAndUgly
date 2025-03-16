use std::ops::Deref;
use actix_web::{get, web, Responder};
use maud::{html, Markup};
use crate::app_state::AppState;
use crate::objects::user::User;

pub fn get_navbar(user: Option<&User>) -> Markup {
    html! {
        nav {
            @if let Some(user) = user {
                a href="/users/" {"Users"} "|"
                a href="/logout" {"Logout"}
            } @else {
                a href="/login" {"Login"} "|"
                a href="/register" {"Register"}
            }
        }
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
        (get_navbar(user.as_ref()))

        "Hello " (name)
    }
}