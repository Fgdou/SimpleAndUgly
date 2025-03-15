use std::collections::HashMap;

use chrono::{DateTime, Utc};

pub struct User {
    pub username: String,
    pub password: String,
    pub email: String,
    tokens: HashMap<String, DateTime<Utc>>,
}

#[derive(Clone, Debug)]
pub struct SessionToken {
    pub value: String,
    pub expiration: DateTime<Utc>
}

impl User {
    pub fn add_token(&mut self, token: SessionToken) {
        self.tokens.insert(token.value, token.expiration);
    }
    pub fn verify_token(&self, token: &SessionToken) -> Result<(), TokenError> {
        match self.tokens.get(&token.value) {
            None => Err(TokenError::NotExist),
            Some(date) if date <= &Utc::now() => Err(TokenError::Expired),
            _ => Ok(())
        }
    }
    pub fn new(username: String, password: String, email: String) -> User {
        User {
            username,
            password,
            email,
            tokens: HashMap::new(),
        }
    }
}

pub enum TokenError {
    Expired,
    NotExist,
}
