use std::rc::Rc;

use chrono::{DateTime, Utc};
use rusqlite::Connection;

use crate::objects::token::{Token, TokenType};

pub struct TokenRepo {
    connection: Rc<Connection>,
}

impl TokenRepo {
    pub fn new(connection: Rc<Connection>) -> Self {
        connection.execute("
            CREATE TABLE IF NOT EXISTS tokens (
                value VARCHAR PRIMARY KEY,
                user_email VARCHAR NOT NULL,
                expiration TEXT,
                token_type TEXT
            )
            ", ()).unwrap();
        Self {
            connection
        }
    }
    pub fn get_token(&self, value: &str, token_type: &TokenType) -> Option<Token> {
        let mut stmt = self.connection.prepare("SELECT * FROM tokens WHERE token_type = ? AND value = ?").unwrap();
        let token = stmt.query_map((token_type, value), |row| Token::from_sql(row)).unwrap()
            .map(|token| token.unwrap())
            .next();
        token
    }

    pub fn add_token(&self, token: Token) {
        self.connection.execute(
            "INSERT INTO tokens (value, user_email, expiration, token_type) VALUES (?, ?, ?, ?)",
            (token.value, token.user_email, token.expiration, token.token_type)
        ).unwrap();
    }

    pub fn invalidate_token(&self, value: &str) -> Result<(), ()> {
        let res = self.connection.execute(
            "UPDATE tokens SET expiration = ? WHERE value = ?",
            (Option::<DateTime<Utc>>::None, value)
        ).unwrap();

        if res == 0 {
            Err(())
        } else {
            Ok(())
        }
    }
}
