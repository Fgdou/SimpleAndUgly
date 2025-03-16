use derive_more::Display;

#[derive(Debug, PartialEq, Display)]
pub enum RegisterError {
    EmailAlreadyExist,
    TokenNotExist,
    TokenExpired,
    InternalError,
    InvalidEmail
}
