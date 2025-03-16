use actix_web::{get, web, Responder, Scope};
use maud::html;
use crate::AppState;

#[get("/")]
async fn list_users(state: web::Data<AppState>) -> impl Responder {
    let users = state.auth.user_repo.get_users();

    html!(
        table {
            tr {
                th {"Name"}
                th {"Email"}
                th {"Creation"}
            }
            @for user in users {
                tr {
                    td {(user.name)}
                    td {(user.email)}
                    td {(user.creation_date.to_string())}
                }
            }
        }
    )
}

pub fn apply_scope(scope: Scope) -> Scope {
    scope.service(list_users)
}