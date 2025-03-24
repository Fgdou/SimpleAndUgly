use crate::errors::validation::ValidationError;

pub enum LoginError {
    EmailNotExist,
    WrongPassword,
}
pub enum RegisterError {
    Validation(ValidationError),
    EmailAlreadyExist,
    TokenRequired,
    TokenNotExist,
    TokenExpired,
}

pub enum AuthenticateError {
    TokenNotExist,
    TokenExpired,
    UserDeleted,
}