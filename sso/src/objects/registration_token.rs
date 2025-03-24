use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct RegisterToken {
    pub value: String,
    pub expiration: DateTime<Utc>,
}