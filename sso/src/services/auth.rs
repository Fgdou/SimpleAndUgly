use std::{rc::Rc, sync::Arc};

use crate::{errors::{login::{AuthError, LoginError}, register::RegisterError}, objects::{token::{Token, TokenType}, user::User}, repositories::{tokens::TokenRepo, users::UserRepo}};

pub struct UserRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

impl Into<User> for UserRequest {
    fn into(self) -> User {
        User::new(self.email, self.name, self.password)
    }
}

pub trait Auth {
    fn login(&self, email: &str, password: &str) -> Result<Token, LoginError>;
    fn register(&self, token: &str, user: UserRequest) -> Result<(), RegisterError>;
    fn create_register_token(&self, email: String) -> Token;
    fn authenticate(&self, token: &str) -> Result<User, AuthError>;
}

pub struct BasicAuth {
    pub user_repo: Rc<dyn UserRepo>,
    pub token_repo: Rc<dyn TokenRepo>,
}

impl Auth for BasicAuth {
    fn login(&self, email: &str, password: &str) -> Result<Token, LoginError> {
        match self.user_repo.get_user_by_email(email) {
            None => Err(LoginError::InvalidEmail),
            Some(user) => {
                match user.verify_password(password) {
                    false => Err(LoginError::InvalidPassword),
                    true => {
                        let token = Token::new(email.to_string(), TokenType::Session);
                        self.token_repo.add_token(token.clone());
                        Ok(token)
                    }
                }
            }
        }
    }
    fn register(&self, token: &str, user: UserRequest) -> Result<(), RegisterError> {
        let email = match self.token_repo.get_token(token, &TokenType::Registration) {
            None => Err(RegisterError::TokenNotExist),
            Some(token) if !token.is_valid() => Err(RegisterError::TokenExpired),
            Some(email) => Ok(email.user_email),
        }?;

        if email != user.email {
            return Err(RegisterError::InvalidEmail)
        }

        if let Some(_) = self.user_repo.get_user_by_email(&user.email) {
            return Err(RegisterError::EmailAlreadyExist);
        }

        self.user_repo.add_user(user.into());

        if let Err(_) = self.token_repo.invalidate_token(token) {
            return Err(RegisterError::InternalError)
        }

        Ok(())

    }
    fn create_register_token(&self, email: String) -> Token {
        let token = Token::new(email, TokenType::Registration);
        self.token_repo.add_token(token.clone());
        token
    }

    fn authenticate(&self, token: &str) -> Result<User, AuthError> {
        let email = match self.token_repo.get_token(token, &TokenType::Session) {
            None => Err(AuthError::TokenNotExist),
            Some(token) if !token.is_valid() => Err(AuthError::TokenExpired),
            Some(token) => Ok(token.user_email),
        }?;

        match self.user_repo.get_user_by_email(&email) {
            None => Err(AuthError::UserNotExist),
            Some(user) => Ok(user)
        }
    }
}
