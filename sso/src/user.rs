use std::collections::HashMap;

use chrono::{DateTime, Utc};

pub struct User {
    username: String,
    password: String,
    email: String,
    tokens: HashMap<String, DateTime<Utc>>,
    registrationDate: DateTime<Utc>,
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
            registrationDate: Utc::now(),
        }
    }
    pub fn get_registration_date(&self) -> &DateTime<Utc> {
        &self.registrationDate
    }
    pub fn get_username(&self) -> &String {
        &self.username
    }
    pub fn verify_password(&self, password: &String) -> bool {
        &self.password == password
    }
}

pub enum TokenError {
    Expired,
    NotExist,
}
