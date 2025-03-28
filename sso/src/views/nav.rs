use maud::{html, Markup};
use crate::app::app_state::AppState;

pub fn get_nav(data: &AppState) -> Markup {
    html! {
        nav {
            a href="/" { "home" }
            @if data.is_connected() {
                a {"logout"}
            } @else {
                a href="/auth/login" {"login"}
                a {"register"}
            }

        }
    }
}