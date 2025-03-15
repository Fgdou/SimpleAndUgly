#[derive(Debug, PartialEq)]
pub enum RegisterError {
    EmailAlreadyExist,
    TokenNotExist,
    TokenExpired,
    InternalError,
    InvalidEmail
}
