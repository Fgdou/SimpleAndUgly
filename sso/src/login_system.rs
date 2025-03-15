use chrono::{Days, Utc};
use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::prelude::*;
use std::process::exit;

use crate::user::{SessionToken, SessionTokenList, TokenError, User};


#[derive(Debug, Clone)]
pub enum LoginError {
    InvalidUser,
    InvalidPassword,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    users: Vec<User>,
    login_tokens: SessionTokenList,
    registration_tokens: SessionTokenList,
}

pub struct LoginSystem {
    data: Data,
    db_path: String,
    config: Config,
}

pub struct Config {
    registration_token_enabled: bool,
}

#[derive(PartialEq, Debug)]
pub enum RegisterError {
    TokenError(TokenError),
    UsernameAlreadyExist,
    EmailAlreadyExist,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            registration_token_enabled: true,
        }
    }
}

impl LoginSystem {
    pub fn new(db_path: String, config: Config) -> LoginSystem {
        let mut instance = LoginSystem {
            data: Data {
                users: vec!(),
                login_tokens: SessionTokenList::new(),
                registration_tokens: SessionTokenList::new(),
            },
            db_path,
            config
        };

        match instance.load() {
            Ok(()) => (),
            Err(LoadError::FailToOpenFile(path)) => {
                println!("Fail to load {}. Create database.", path);
                instance.data.users.push(User::new("admin".to_string(), "admin".to_string(), "admin@example.com".to_string()));
            },
            Err(LoadError::FailToParseJson) => {
                println!("Warn : fail to parse JSON from the database");
            }
        }

        instance
    }

    pub fn login(&mut self, username: &str, password: &str) -> Result<SessionToken, LoginError> {
        let user = self.data.users.iter_mut().find(|user| user.get_username() == username);

        match user {
            None => Err(LoginError::InvalidUser),
            Some(user) if !user.verify_password(password) => Err(LoginError::InvalidPassword),
            Some(user) => Ok(user),
        }?;

        let token = SessionToken::new(username.to_string());
        self.data.login_tokens.insert_token(token.clone());

        return Ok(token);
    }

    pub fn verify_login_token(&self, token: &str) -> Result<&User, TokenError> {
        let username = self.data.login_tokens.verify_token(token)?.get_username();

        let user = self.data.users.iter().find(|user| user.get_username() == username);

        match user {
            None => Err(TokenError::NotExist),
            Some(user) => Ok(&user),
        }
    }

    pub fn register(&mut self, token: &str, user: User) -> Result<(), RegisterError> {
        if self.config.registration_token_enabled {
            match self.data.registration_tokens.verify_token(token) {
                Err(e) => return Err(RegisterError::TokenError(e)),
                Ok(_) => (),
            }
        }

        if self.data.users.iter().any(|u| user.get_username() == u.get_username()) {
            return Err(RegisterError::UsernameAlreadyExist)
        }
        if self.data.users.iter().any(|u| user.get_email() == u.get_email()) {
            return Err(RegisterError::EmailAlreadyExist)
        }

        self.data.users.push(user);

        self.data.registration_tokens.remove_token(token);

        Ok(())
    }

    pub fn save(&self) -> Option<()> {
        let json = serde_json::to_string(&self.data).ok()?;
        fs::write(&self.db_path, json).ok()?;
        Some(())
    }
    pub fn load(&mut self) -> Result<(), LoadError> {
        match fs::read_to_string(&self.db_path) {
            Err(_) => Err(LoadError::FailToOpenFile(self.db_path.clone())),

            Ok(json) => match serde_json::from_str(&json) {
                Err(_) => Err(LoadError::FailToParseJson),
                Ok(data) => {
                    self.data = data;
                    Ok(())
                }
            }
        }
    }
}

#[derive(PartialEq, Debug)]
enum LoadError {
    FailToOpenFile(String),
    FailToParseJson
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_token() {
        let user = User::new("admin".to_string(), "admin".to_string(), "admin@example.com".to_string());
        let mut instance = LoginSystem {
            data: Data {
                login_tokens: SessionTokenList::new(),
                users: vec!(
                    user.clone(),
                ),
                registration_tokens: SessionTokenList::new(),
            },
            db_path: String::new(),
            config: Config::default(),
        };
        assert_eq!(Err(TokenError::NotExist), instance.verify_login_token(&"token1"));

        let token1 = SessionToken::new("admin".to_string());
        let token2 = SessionToken::new("test".to_string());
        instance.data.login_tokens.insert_token(token1.clone());
        instance.data.login_tokens.insert_token(token2.clone());

        assert_eq!(Ok(&user), instance.verify_login_token(&token1.get_value()));
        assert_eq!(Err(TokenError::NotExist), instance.verify_login_token(&token2.get_value()));
    }

    #[test]
    fn test_registration_token_disabled() {
        let user1 = User::new("admin2".to_string(), "admin2".to_string(), "admin2@example.com".to_string());
        let mut instance = LoginSystem::new("".to_string(), Config { registration_token_enabled: false});
        assert_eq!(Ok(()), instance.register("", user1.clone()));
    }

    #[test]
    fn test_registration_token() {
        let token = SessionToken::new("test".to_string());
        let user1 = User::new("admin".to_string(), "admin".to_string(), "admin@example.com".to_string());
        let user11 = User::new("admin".to_string(), "admin".to_string(), "admin11@example.com".to_string());
        let user12 = User::new("admin11".to_string(), "admin".to_string(), "admin@example.com".to_string());
        let user2 = User::new("admin2".to_string(), "admin2".to_string(), "admin2@example.com".to_string());

        let mut instance = LoginSystem::new("".to_string(), Config { registration_token_enabled: true});
        instance.data.registration_tokens.insert_token(token.clone());
        instance.data.users.push(user1.clone());

        let number_of_users = instance.data.users.len();

        assert_eq!(Err(RegisterError::TokenError(TokenError::NotExist)), instance.register(&"abc", user1.clone()));
        assert_eq!(Err(RegisterError::UsernameAlreadyExist), instance.register(&token.get_value(), user11.clone()));
        assert_eq!(Err(RegisterError::EmailAlreadyExist), instance.register(&token.get_value(), user12.clone()));

        assert_eq!(number_of_users, instance.data.users.len());

        assert_eq!(Ok(()), instance.register(&token.get_value(), user2.clone()));

        assert_eq!(number_of_users + 1, instance.data.users.len());

        assert_eq!(Err(RegisterError::TokenError(TokenError::NotExist)), instance.register(&token.get_value(), user1.clone()));
    }

    #[test]
    fn test_file_not_exist() {
        let mut instance = LoginSystem::new("".to_string(), Config::default());
        assert_eq!(None, instance.save());
        assert_eq!(Err(LoadError::FailToOpenFile("".to_string())), instance.load());
    }

    #[test]
    fn test_file_empty() {
        let file = "/tmp/db_empty.json".to_string();

        fs::write(&file, "").expect("Failed to write file");

        let mut instance = LoginSystem::new(file.clone(), Config::default());
        assert_eq!(Err(LoadError::FailToParseJson), instance.load());
        assert_eq!(Some(()), instance.save());
    }

    #[test]
    fn test_file_normal() {
        let file = "/tmp/db.json".to_string();
        let mut instance = LoginSystem::new(file, Config::default());
        assert_eq!(Some(()), instance.save());
        assert_eq!(Ok(()), instance.load());
    }
}
