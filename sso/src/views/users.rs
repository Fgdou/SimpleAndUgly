use actix_web::{get, post, web, Responder, Scope};
use maud::html;
use serde::Deserialize;
use crate::AppState;
use crate::objects::token::{Token, TokenType};

enum ActionInfo {
    None,
    CreateToken(Token),
}
fn list_user_page(state: &web::Data<AppState>, action_info: ActionInfo) -> impl Responder {
    let users = state.auth.user_repo.get_users();
    let tokens = state.auth.token_repo.get_tokens(&TokenType::Registration);

    html!(
        @match action_info {
            ActionInfo::None => {}
            ActionInfo::CreateToken(token) => div {
                @let email = token.user_email;
                @let value = token.value;
                (format!("Created token for {email} : {value}"))
            }
        }

        h3 {"Users"}
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
                    td {(user.creation_date.format("%Y-%m-%d %H:%M:%S"))}
                }
            }
        }

        h3 {"Pending registration"}
        table {
            tr {
                th {"Email"}
                th {"Expiration"}
            }
            @for token in tokens {
                tr {
                    td {(token.user_email)}
                    td {(token.expiration.map_or("None".to_string(), |e| e.format("%Y-%m-%d %H:%M:%S").to_string()))}
                }
            }
        }

        form action="./" method="post" {
            input type="email" name="email" placeholder="email";
            button type="submit" {"Create registration token"}
        }
    )
}
#[get("/")]
async fn list_users(state: web::Data<AppState>) -> impl Responder {
    list_user_page(&state, ActionInfo::None)
}

#[derive(Deserialize)]
struct CreateRegisterToken {
    email: String
}
#[post("/")]
async fn create_register_token(state: web::Data<AppState>, query: web::Form<CreateRegisterToken>) -> impl Responder {
    let email = query.into_inner().email;
    let token = state.auth.create_register_token(email);
    list_user_page(&state, ActionInfo::CreateToken(token))
}

pub fn apply_scope(scope: Scope) -> Scope {
    scope.service(list_users)
        .service(create_register_token)
}