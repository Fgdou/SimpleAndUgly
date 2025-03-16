use std::sync::Arc;
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

pub struct Auth {
    pub user_repo: Arc<UserRepo>,
    pub token_repo: Arc<TokenRepo>,
}

impl Auth {
    pub fn login(&self, email: &str, password: &str) -> Result<Token, LoginError> {
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
    pub fn register(&self, token: &str, user: UserRequest) -> Result<(), RegisterError> {
        if user.email.is_empty() {
            return Err(RegisterError::EmptyEmail)
        }
        if user.name.len() < 3 {
            return Err(RegisterError::NameUnderThreeCharacter)
        }
        if user.password.len() < 5 {
            return Err(RegisterError::PasswordUnderFiveCharacter)
        }

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
    pub fn create_register_token(&self, email: String) -> Token {
        let token = Token::new(email, TokenType::Registration);
        self.token_repo.add_token(token.clone());
        token
    }

    pub fn authenticate(&self, token: &str) -> Result<User, AuthError> {
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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::{Arc, Mutex};
    use rusqlite::Connection;

    use crate::{errors::login::{AuthError, LoginError}, objects::user::User};

    use super::*;

    fn instance_with_admin() -> Auth {
        let path = "/tmp/testdb.sqlite";
        let _ = fs::remove_file(path);

        let conn = Arc::new(Mutex::new(Connection::open(path).unwrap()));

        let tokens = Arc::new(TokenRepo::new(conn.clone()));
        let users = Arc::new(UserRepo::new(conn.clone()));

        let auth = Auth {
            token_repo: tokens,
            user_repo: users,
        };

        let token = auth.create_register_token("admin@example.com".to_string());
        auth.register(&token.value, UserRequest {
            email: "admin@example.com".to_string(),
            name: "Admin".to_string(),
            password: "admin".to_string(),
        }).unwrap();

        auth
    }

    #[test]
    fn test_login() {
        let auth = instance_with_admin();

        assert_eq!(Err(LoginError::InvalidEmail), auth.login("test@test.test", "test"));
        assert_eq!(Err(LoginError::InvalidPassword), auth.login("admin@example.com", "test"));

        let token = auth.login("admin@example.com", "admin");
        assert_eq!(true, token.is_ok());
        let token = token.unwrap();

        assert_eq!(Err(AuthError::TokenNotExist), auth.authenticate("test"));
        assert_eq!(
            Ok(User::new("admin@example.com".to_string(), "admin".to_string(), "admin".to_string())),
            auth.authenticate(&token.value)
        );
    }
}
