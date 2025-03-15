#[derive(Debug, PartialEq)]
pub enum LoginError {
    InvalidEmail,
    InvalidPassword,
}
#[derive(Debug, PartialEq)]
pub enum AuthError {
    TokenNotExist,
    TokenExpired,
    UserNotExist,
}
