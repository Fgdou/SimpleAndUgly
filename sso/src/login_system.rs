use chrono::{Days, Utc};
use rand::distr::{Alphanumeric, SampleString};

use crate::user::{SessionToken, TokenError, User};


enum LoginError {
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
    users: Vec<User>
}

impl LoginSystem {
    pub fn new() -> LoginSystem {
        LoginSystem {
            users: vec!(
                User::new("admin".to_string(), "admin".to_string(), "admin@example.com".to_string())
            )
        }
    }

    pub fn login(&mut self, username: String, password: String) -> Result<SessionToken, LoginError> {
        let user = self.users.iter_mut().find(|user| user.username == username);

        let user = match user {
            None => Err(LoginError::InvalidUser),
            Some(user) if user.password != password => Err(LoginError::InvalidPassword),
            Some(user) => Ok(user),
        }?;

        let token = generate_token();
        user.add_token(token.clone());
        return Ok(token);
    }

    pub fn verifyToken(&self, token: &SessionToken) -> Result<&User, TokenError> {
        for user in &self.users {
            match user.verify_token(token) {
                Ok(()) => return Ok(user),
                Err(TokenError::Expired) => return Err(TokenError::Expired),
                Err(TokenError::NotExist) => ()
            }
        }

        Err(TokenError::NotExist)
    }
}
