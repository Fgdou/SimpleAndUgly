use std::fmt::{Display, Formatter};
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

impl Display for LoginError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            LoginError::EmailNotExist => "Invalid email",
            LoginError::WrongPassword => "Invalid password"
        };
        f.write_str(str)?;
        Ok(())
    }
}
impl Display for AuthenticateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AuthenticateError::TokenNotExist => "Invalid Token",
            AuthenticateError::TokenExpired => "Expired Token",
            AuthenticateError::UserDeleted => "User does not exist",
        };
        f.write_str(str)?;
        Ok(())
    }
}