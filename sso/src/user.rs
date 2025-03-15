use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    username: String,
    password: String,
    email: String,
    tokens: HashMap<String, DateTime<Utc>>,
    registration_date: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionToken {
    pub value: String,
    pub expiration: DateTime<Utc>
}

impl User {
    pub fn add_token(&mut self, token: SessionToken) {
        self.tokens.insert(token.value, token.expiration);
    }
    pub fn verify_token(&self, token: &str) -> Result<(), TokenError> {
        match self.tokens.get(token) {
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
            registration_date: Utc::now(),
        }
    }
    pub fn get_registration_date(&self) -> &DateTime<Utc> {
        &self.registration_date
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn verify_password(&self, password: &str) -> bool {
        &self.password == password
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenError {
    Expired,
    NotExist,
}

#[cfg(test)]
mod tests {
    use chrono::Days;

    use super::*;

    #[test]
    fn test_verify_token() {
        let mut user = User::new("admin".to_string(), "admin".to_string(), "admin@example.com".to_string());
        user.add_token(SessionToken{
            value: "token1".to_string(),
            expiration: Utc::now() - Days::new(1)
        });
        user.add_token(SessionToken{
            value: "token2".to_string(),
            expiration: Utc::now() + Days::new(1)
        });

        assert_eq!(Ok(()), user.verify_token(&"token2"));
        assert_eq!(Err(TokenError::Expired), user.verify_token(&"token1"));
        assert_eq!(Err(TokenError::NotExist), user.verify_token(&"token3"))
    }

    #[test]
    fn test_verify_password() {
        let user = User::new("admin".to_string(), "admin".to_string(), "admin@example.com".to_string());

        assert_eq!(true, user.verify_password("admin"));
        assert_eq!(false, user.verify_password("admine"));
    }
}
