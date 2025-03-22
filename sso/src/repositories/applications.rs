use std::sync::{Arc, Mutex};
use rusqlite::Connection;
use rusqlite::fallible_iterator::FallibleIterator;
use crate::objects::application::Application;
use crate::services::apps::CreateApp;

pub struct ApplicationRepo {
    conn: Arc<Mutex<Connection>>
}

impl ApplicationRepo {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        {
            let conn = conn.lock().unwrap();
            conn.execute("
            CREATE TABLE IF NOT EXISTS applications (
                name VARCHAR PRIMARY KEY,
                base_url TEXT,
                client_id TEXT,
                client_secret TEXT
            )
            ", ()).unwrap();
        }
        Self {
            conn
        }
    }

    pub fn get_applications(&self) -> Vec<Application> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM applications").unwrap();

        let res = stmt.query_map((), Application::from_sql)
            .unwrap()
            .map(|app| app.unwrap())
            .collect();
        res
    }

    pub fn get_by_name(&self, name: &str) -> Option<Application> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM applications WHERE name = ?").unwrap();

        let res = stmt.query_map((name,), Application::from_sql)
            .unwrap()
            .map(|app| app.unwrap())
            .next();
        res
    }

    pub fn create(&self, app: &Application) {
        let conn = self.conn.lock().unwrap();
        conn.execute("
        INSERT INTO applications (name, base_url, client_id, client_secret)
        VALUES (?, ?, ?, ?)
        ", (&app.name, &app.base_url, &app.client_id, &app.client_secret)).unwrap();
    }
}