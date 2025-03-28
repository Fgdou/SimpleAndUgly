use actix_web::{get, web, Scope};
use maud::{html, Markup};
use crate::app::app_state::AppState;
use crate::views::nav::get_nav;

#[get("/login")]
async fn login_page(state: web::Data<AppState>) -> Markup {
    html! {
        (get_nav(&state))
        form {
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
}