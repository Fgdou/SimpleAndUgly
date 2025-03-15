use chrono::{Days, Utc};
use rand::distr::{Alphanumeric, SampleString};
use std::fs::{self, File};
use std::io::prelude::*;
use std::process::exit;

use crate::user::{SessionToken, TokenError, User};


#[derive(Debug, Clone)]
pub enum LoginError {
    InvalidUser,
    InvalidPassword,
}

fn generate_token() -> SessionToken {
    SessionToken {
        expiration: Utc::now() + Days::new(10),
        value: Alphanumeric.sample_string(&mut rand::rng(), 64),
    }
}

pub struct LoginSystem {
    users: Vec<User>,
    db_path: String,
}

impl LoginSystem {
    pub fn new(db_path: String) -> LoginSystem {
        let mut instance = LoginSystem {
            users: vec!(),
            db_path,
        };

        match instance.load() {
            Ok(()) => (),
            Err(ErrorLoading::FailToOpenFile(path)) => {
                println!("Fail to load {}. Create database.", path);
                instance.users.push(User::new("admin".to_string(), "admin".to_string(), "admin@example.com".to_string()));
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
        let user = self.users.iter_mut().find(|user| user.get_username() == username);

        let user = match user {
            None => Err(LoginError::InvalidUser),
            Some(user) if !user.verify_password(password) => Err(LoginError::InvalidPassword),
            Some(user) => Ok(user),
        }?;

        let token = generate_token();
        user.add_token(token.clone());

        if let None = self.save() {
            println!("Warn : fail to save Database");
        }

        return Ok(token);
    }

    pub fn verifyToken(&self, token: &str) -> Result<&User, TokenError> {
        for user in &self.users {
            match user.verify_token(token) {
                Ok(()) => return Ok(user),
                Err(TokenError::Expired) => return Err(TokenError::Expired),
                Err(TokenError::NotExist) => ()
            }
        }

        Err(TokenError::NotExist)
    }

    fn save(&self) -> Option<()> {
        let json = serde_json::to_string(&self.users).ok()?;
        fs::write(&self.db_path, json).ok()?;
        Some(())
    }
    fn load(&mut self) -> Result<(), ErrorLoading> {
        match fs::read_to_string(&self.db_path) {
            Err(_) => Err(ErrorLoading::FailToOpenFile(self.db_path.clone())),

            Ok(json) => match serde_json::from_str(&json) {
                Err(_) => Err(ErrorLoading::FailToParseJson),
                Ok(data) => {
                    self.users = data;
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
