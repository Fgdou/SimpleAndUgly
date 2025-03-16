use chrono::{DateTime, Days, Utc};
use rand::distr::SampleString;
use rand_distr::Alphanumeric;
use rusqlite::{types::{FromSql, FromSqlError}, ToSql};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum TokenType {
    Registration,
    Session,
}

#[derive(Clone, Debug, Serialize)]
pub struct Token {
    pub value: String,
    pub expiration: Option<DateTime<Utc>>,
    pub user_email: String,
    pub token_type: TokenType,
}

impl FromSql for TokenType {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value.as_str()? {
            "Registration" => Ok(TokenType::Registration),
            "Session" => Ok(TokenType::Session),
            _ => Err(FromSqlError::Other(Box::from("Value does not exist")))
        }
    }
}
impl ToSql for TokenType {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(
            match self {
                TokenType::Registration => "Registration".into(),
                TokenType::Session => "Session".into()
            }
        )
    }
}

impl Token {
    fn generate_value() -> String {
        Alphanumeric.sample_string(&mut rand::rng(), 64)
    }
    fn get_expiration() -> DateTime<Utc> {
        Utc::now() + Days::new(10)
    }
    pub fn new(email: String, token_type: TokenType) -> Token {
        Token {
            value: Token::generate_value(),
            expiration: Some(Token::get_expiration()),
            token_type,
            user_email: email,
        }
    }
    pub fn is_valid(&self) -> bool {
        match self.expiration {
            None => false,
            Some(expiration) => Utc::now() < expiration
        }
    }

    pub fn from_sql(row: &rusqlite::Row<'_>) -> Result<Token, rusqlite::Error> {
        Ok(
            Token {
                value: row.get(0)?,
                user_email: row.get(1)?,
                expiration: row.get(2)?,
                token_type: row.get(3)?,
            }
        )
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
