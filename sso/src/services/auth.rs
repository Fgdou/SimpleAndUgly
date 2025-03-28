use chrono::{DateTime, Days, Utc};
use rand::distr::Alphanumeric;
use rand::Rng;
use regex::Regex;
use crate::errors::auth::{AuthenticateError, LoginError, RegisterError};
use crate::errors::validation::{ValidationEnumError, ValidationError};
use crate::forms::auth::{LoginForm, RegisterForm};
use crate::objects::config::Config;
use crate::objects::login_token::LoginToken;
use crate::objects::user::User;
use crate::services::factory::Repos;

pub struct AuthService {
    repos: Repos,
    config: Config,
}

type LoginResult = Result<LoginToken, LoginError>;
type RegisterResult = Result<(), RegisterError>;
type AuthenticateResult = Result<User, AuthenticateError>;

impl AuthService {
    pub fn new(config: Config, repos: Repos) -> Self {
        Self {
            repos,
            config
        }
    }
    fn generate_value() -> String {
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect()
    }
    fn generate_token(user: &User) -> LoginToken {
        LoginToken {
            user: user.email.clone(),
            expiration: Utc::now() + Days::new(30),
            value: Self::generate_value(),
        }
    }
    fn date_expired(date: &DateTime<Utc>) -> bool {
        &Utc::now() > date
    }
    fn verify_password(user: &User, password: &str) -> bool {
        user.password == password
    }

    pub fn login(&self, form: &LoginForm) -> LoginResult {
        let user = self.repos.user_repo.get_by_email(&form.email)
            .ok_or(LoginError::EmailNotExist)?;

        if !Self::verify_password(&user, &form.password) {
            return Err(LoginError::WrongPassword);
        }

        let token = Self::generate_token(&user);

        self.repos.login_token_repo.add(token.clone());

        Ok(token)
    }
    fn verify_register_token(&self, token: &str) -> Result<(), RegisterError> {
        match self.repos.register_token_repo.get_by_value(token) {
            None => Err(RegisterError::TokenNotExist),
            Some(token) => {
                match Self::date_expired(&token.expiration) {
                    true => Err(RegisterError::TokenExpired),
                    false => Ok(())
                }
            }
        }
    }
    fn validate_user(form: &RegisterForm) -> Result<(), RegisterError> {
        if form.name.len() < 2 || form.name.len() > 40 {
            return Err(RegisterError::Validation(ValidationError {
                field: "name".to_string(),
                error: ValidationEnumError::Size(2, 40)
            }));
        }
        if form.password.len() < 6 || form.password.len() > 255 {
            return Err(RegisterError::Validation(ValidationError {
                field: "password".to_string(),
                error: ValidationEnumError::Size(6, 255)
            }));
        }
        if !Regex::new(r"^[\w\.-]+@([\w-]+\.)+[\w-]{2,4}$").unwrap().is_match(&form.email) {
            return Err(RegisterError::Validation(ValidationError {
                field: "email".to_string(),
                error: ValidationEnumError::Regex("abc@example.com".to_string()),
            }));
        }
        Ok(())
    }
    pub fn register(&self, form: &RegisterForm) -> RegisterResult {
        match (self.config.restrict_registration, &form.token) {
            (false, _) => Ok(()),
            (true, None) => Err(RegisterError::TokenRequired),
            (true, Some(token)) => self.verify_register_token(token),
        }?;

        Self::validate_user(form)?;

        let user = User {
            email: form.email.clone(),
            password: form.password.clone(),
            name: form.name.clone(),
            admin: false,
            created: Utc::now(),
        };

        self.repos.user_repo.add(user);

        if let Some(token) = &form.token {
            self.repos.register_token_repo.delete(token)
        }

        Ok(())
    }
    pub fn authenticate(&self, token: &str) -> AuthenticateResult {
        let token = match self.repos.login_token_repo.get_by_value(token) {
            None => Err(AuthenticateError::TokenNotExist),
            Some(token) => {
                match Self::date_expired(&token.expiration) {
                    true => Err(AuthenticateError::TokenExpired),
                    false => Ok(token),
                }
            }
        }?;

        match self.repos.user_repo.get_by_email(&token.user) {
            None => Err(AuthenticateError::UserDeleted),
            Some(user) => Ok(user),
        }
    }
    pub fn invalidate_token(&self, token: &str) {
        self.repos.login_token_repo.delete(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_user() {
        assert_eq!(true, AuthService::validate_user(&RegisterForm{
            name: "Test".to_string(),
            password: "testtest".to_string(),
            email: "test@example.com".to_string(),
            token: None,
        }).is_ok());
        assert_eq!(false, AuthService::validate_user(&RegisterForm{
            name: "Test".to_string(),
            password: "testtest".to_string(),
            email: "test".to_string(),
            token: None,
        }).is_ok());
        assert_eq!(false, AuthService::validate_user(&RegisterForm{
            name: "Test".to_string(),
            password: "testtest".to_string(),
            email: "test@@example.com".to_string(),
            token: None,
        }).is_ok());
        assert_eq!(false, AuthService::validate_user(&RegisterForm{
            name: "Test".to_string(),
            password: "test".to_string(),
            email: "test@example.com".to_string(),
            token: None,
        }).is_ok());
    }

    fn get_service() -> AuthService {
        let config = Config::default();
        AuthService{
            repos: Repos::new(&config),
            config,
        }
    }

    #[test]
    fn test_login() {
        let service = get_service();

        assert_eq!(true, service.login(&LoginForm {
            email: "admin@example.com".to_string(),
            password: "admin".to_string()
        }).is_ok());
        assert_eq!(false, service.login(&LoginForm {
            email: "admin@example.com".to_string(),
            password: "admi".to_string()
        }).is_ok());
        assert_eq!(false, service.login(&LoginForm {
            email: "admi@example.com".to_string(),
            password: "admin".to_string()
        }).is_ok())
    }

    #[test]
    fn test_register() {
        assert_eq!(true, get_service().register(&RegisterForm {
            password: "testtest".to_string(),
            email: "admin2@example.com".to_string(),
            token: Some("token".to_string()),
            name: "Admin".to_string()
        }).is_ok())
    }
}