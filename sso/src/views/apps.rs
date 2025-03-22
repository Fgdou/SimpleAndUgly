use actix_web::{get, post, web, Responder, Scope};
use maud::html;
use crate::app_state::AppState;
use crate::errors::apps::CreateApplicationError;
use crate::services::apps::CreateApp;
use crate::views::home::get_navbar;

fn list_apps(state: &web::Data<AppState>, form: Option<&CreateApp>, err: Option<&CreateApplicationError>) -> impl Responder {
    let apps = state.apps.application.get_applications();

    html! {
        (get_navbar(state.user.lock().unwrap().as_ref(), "Applications"))

        table {
            tr {
                th { "Name" }
                th { "Url" }
                th { "Client Id" }
                th { "Client Secret" }
            }

            @for app in apps {
                tr {
                    td { (app.name) }
                    td { (app.base_url) }
                    td { (app.client_id) }
                    td { (app.client_secret) }
                }
            }
        }

        @if let Some(err) = err {
            div {
                @match err {
                    CreateApplicationError::Validation{field, error} => (format!("{} : {}", field, error))
                }
            }
        }

        form action="/apps/" method="post" {
            input type="text" name="name" placeholder="Name" value=(form.map_or("", |f| &f.name));
            br;
            input type="text" name="base_url" placeholder="https://service.example.com/" value=(form.map_or("", |f| &f.base_url));
            br;
            button type="submit" { "Create" }
        }
    }
}

#[post("/")]
async fn create_app(state: web::Data<AppState>, form: web::Form<CreateApp>) -> impl Responder {
    let form = form.into_inner();
    let created = state.apps.create_app(&form);

    list_apps(&state, Some(&form), created.as_ref().err())
}

#[get("/")]
async fn get_apps(state: web::Data<AppState>) -> impl Responder {
    list_apps(&state, None, None)
}

pub fn apply_scope(scope: Scope) -> Scope {
    scope.service(get_apps)
}