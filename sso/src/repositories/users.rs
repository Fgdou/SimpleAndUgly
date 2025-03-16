use std::rc::Rc;

use rusqlite::Connection;

use crate::objects::user::User;


pub struct UserRepo {
    connection: Rc<Connection>,
}

impl UserRepo {
    pub fn new(connection: Rc<Connection>) -> Self {
        connection.execute("
            CREATE TABLE IF NOT EXISTS users (
            email VARCHAR PRIMARY KEY,
            password VARCHAR,
            name VARCHAR,
            creation_date TEXT
            )
            ", ()).unwrap();
        Self {
            connection
        }
    }
    pub fn get_users(&self) -> Vec<User> {
        let mut stmt = self.connection.prepare("SELECT * FROM users").unwrap();
        stmt.query_map([], |row| User::from_sql(row)).unwrap()
            .map(|user| user.unwrap())
            .collect()
    }

    pub fn get_user_by_email(&self, email: &str) -> Option<User> {
        let mut stmt = self.connection.prepare("SELECT * FROM users WHERE email = ?").unwrap();
        let user = stmt.query_map([email], |row| User::from_sql(row)).unwrap()
                .map(|user| user.unwrap())
                .next();
        user
    }

    pub fn add_user(&self, user: User) {
        self.connection.execute("INSERT INTO users (email, password, name, creation_date) VALUES (?, ?, ?, ?)", (
            &user.email,
            user.get_raw_password(),
            &user.name,
            &user.creation_date,
        )).unwrap();
    }
}
