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
}

pub struct LoginSystem {
    data: Data,
    db_path: String,
}

impl LoginSystem {
    pub fn new(db_path: String) -> LoginSystem {
        let mut instance = LoginSystem {
            data: Data {
                users: vec!(),
                login_tokens: SessionTokenList::new(),
            },
            db_path,
        };

        match instance.load() {
            Ok(()) => (),
            Err(ErrorLoading::FailToOpenFile(path)) => {
                println!("Fail to load {}. Create database.", path);
                instance.data.users.push(User::new("admin".to_string(), "admin".to_string(), "admin@example.com".to_string()));
            },
            Err(ErrorLoading::FailToParseJson) => {
                println!("Error : fail to parse JSON from the database");
                exit(1);
            }
        }

        if let None = instance.save() {
            print!("Warn : fail to save database");
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

        if let None = self.save() {
            println!("Warn : fail to save Database");
        }

        return Ok(token);
    }

    pub fn verifyToken(&self, token: &str) -> Result<&User, TokenError> {
        let username = self.data.login_tokens.verify_token(token)?.get_username();

        let user = self.data.users.iter().find(|user| user.get_username() == username);

        match user {
            None => Err(TokenError::NotExist),
            Some(user) => Ok(&user),
        }
    }

    fn save(&self) -> Option<()> {
        let json = serde_json::to_string(&self.data).ok()?;
        fs::write(&self.db_path, json).ok()?;
        Some(())
    }
    fn load(&mut self) -> Result<(), ErrorLoading> {
        match fs::read_to_string(&self.db_path) {
            Err(_) => Err(ErrorLoading::FailToOpenFile(self.db_path.clone())),

            Ok(json) => match serde_json::from_str(&json) {
                Err(_) => Err(ErrorLoading::FailToParseJson),
                Ok(data) => {
                    self.data = data;
                    Ok(())
                }
            }
        }
    }
}

enum ErrorLoading {
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
            },
            db_path: String::new(),
        };
        assert_eq!(Err(TokenError::NotExist), instance.verifyToken(&"token1"));

        let token1 = SessionToken::new("admin".to_string());
        let token2 = SessionToken::new("test".to_string());
        instance.data.login_tokens.insert_token(token1.clone());
        instance.data.login_tokens.insert_token(token2.clone());

        assert_eq!(Ok(&user), instance.verifyToken(&token1.get_value()));
        assert_eq!(Err(TokenError::NotExist), instance.verifyToken(&token2.get_value()));
    }
}
