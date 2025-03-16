use actix_web::error;
use derive_more::derive::{Display, Error};

#[derive(Debug, PartialEq, Display, Error)]
pub enum LoginError {
    InvalidEmail,
    InvalidPassword,
}
#[derive(Debug, PartialEq, Display, Error)]
pub enum AuthError {
    TokenNotExist,
    TokenExpired,
    UserNotExist,
}

impl error::ResponseError for LoginError {}
impl error::ResponseError for AuthError {}
