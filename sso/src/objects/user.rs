use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
    pub admin: bool,
    pub created: DateTime<Utc>
}