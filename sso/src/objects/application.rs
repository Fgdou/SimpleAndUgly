use rusqlite::{Error, Row};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Application {
    pub name: String,
    pub base_url: String,
    pub client_id: String,
    pub client_secret: String,
}

impl Application {
    pub fn from_sql(row: &Row) -> Result<Application, Error> {
        Ok(Application {
            name: row.get(0)?,
            base_url: row.get(1)?,
            client_id: row.get(2)?,
            client_secret: row.get(3)?
        })
    }
}