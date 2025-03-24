use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct LoginToken {
    pub value: String,
    pub user: String,
    pub expiration: DateTime<Utc>,
}