use chrono::{DateTime, Utc};
use rusqlite::Error;

#[derive(Debug)]
pub struct User {
    pub email: String,
    password: String,
    pub name: String,
    pub creation_date: DateTime<Utc>
}

impl User {
    pub fn new(email: String, name: String, password: String) -> User {
        User {
            email,
            name,
            password,
            creation_date: Utc::now(),
        }
    }
    pub fn verify_password(&self, password: &str) -> bool {
        self.password == password
    }
    pub fn set_password(&mut self, password: String) {
        self.password = password
    }

    pub fn from_sql(row: &rusqlite::Row<'_>) -> Result<User, Error> {
        Ok(User {
            email: row.get(0)?,
            password: row.get(1)?,
            name: row.get(2)?,
            creation_date: row.get(3)?,
        })
    }

    pub fn get_raw_password(&self) -> &str {
        &self.password
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.email == other.email
    }
}
