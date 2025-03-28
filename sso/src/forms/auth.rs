use serde::Deserialize;
use crate::objects::registration_token::RegisterToken;

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}
pub struct RegisterForm {
    pub email: String,
    pub password: String,
    pub name: String,
    pub token: Option<String>
}