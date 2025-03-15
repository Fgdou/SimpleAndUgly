use std::collections::HashMap;

use chrono::{DateTime, Days, Utc};
use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    username: String,
    password: String,
    email: String,
    registration_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionTokenList {
    list: HashMap<String, SessionToken>,
}

impl SessionTokenList {
    pub fn new() -> SessionTokenList {
        SessionTokenList {
            list: HashMap::new(),
        }
    }
    pub fn from_list(list: Vec<SessionToken>) -> SessionTokenList {
        SessionTokenList {
            list: list.into_iter().fold(HashMap::new(), |mut map, i| {
                map.insert(i.value.clone(), i);
                map
            })
        }
    }
    pub fn insert_token(&mut self, token: SessionToken) {
        self.list.insert(token.value.clone(), token);
    }
    pub fn verify_token(&self, token: &str) -> Result<&SessionToken, TokenError> {
        match self.list.get(token) {
            None => Err(TokenError::NotExist),
            Some(token) if token.expiration <= Utc::now() => Err(TokenError::Expired),
            Some(token) => Ok(token)
        }
    }

    pub fn remove_token(&mut self, token: &str) {
        let _ = self.list.remove(token);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SessionToken {
    value: String,
    expiration: DateTime<Utc>,
    username: String,
}

impl SessionToken {
    pub fn new(username: String) -> SessionToken {
        SessionToken {
            expiration: Utc::now() + Days::new(10),
            value: Alphanumeric.sample_string(&mut rand::rng(), 64),
            username
        }
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn get_expiration(&self) -> &DateTime<Utc> {
        &self.expiration
    }
    pub fn get_value(&self) -> &str {
        &self.value
    }
}

impl User {
    pub fn new(username: String, password: String, email: String) -> User {
        User {
            username,
            password,
            email,
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
    pub fn get_email(&self) -> &str {
        &self.email
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
        let mut token_list = SessionTokenList::new();

        let token1 = SessionToken{
            value: "token1".to_string(),
            expiration: Utc::now() - Days::new(1),
            username: "admin".to_string(),
        };
        let token2 = SessionToken{
            value: "token2".to_string(),
            expiration: Utc::now() + Days::new(1),
            username: "admin".to_string(),
        };

        token_list.insert_token(token1.clone());
        token_list.insert_token(token2.clone());

        assert_eq!(Ok(&token2), token_list.verify_token(&"token2"));
        assert_eq!(Err(TokenError::Expired), token_list.verify_token(&"token1"));
        assert_eq!(Err(TokenError::NotExist), token_list.verify_token(&"token3"))
    }

    #[test]
    fn test_verify_password() {
        let user = User::new("admin".to_string(), "admin".to_string(), "admin@example.com".to_string());

        assert_eq!(true, user.verify_password("admin"));
        assert_eq!(false, user.verify_password("admine"));
    }
}
