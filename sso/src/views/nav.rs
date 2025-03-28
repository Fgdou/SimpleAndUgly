use maud::{html, Markup};
use crate::app::app_state::AppState;

pub fn get_nav(data: &AppState) -> Markup {
    html! {
        script src="https://unpkg.com/htmx.org@2.0.4" integrity="sha384-HGfztofotfshcF7+8n44JQL2oJmowVChPTg48S+jvZoztPfvwD79OC/LTtG6dMp+" crossorigin="anonymous" {}
        nav {
            a href="/" { "home" }
            @if data.is_connected() {
                a href="/auth/logout" {"logout"}
            } @else {
                a href="/auth/login" {"login"}
                a {"register"}
            }

        }
    }
}