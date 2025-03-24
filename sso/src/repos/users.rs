use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::Utc;
use crate::objects::user::User;

pub trait UserRepo {
    fn get_by_email(&self, email: &str) -> Option<User>;
    fn get_all(&self) -> Vec<User>;
    fn add(&self, user: User);
}

pub struct UserRepoMemory {
    users: Arc<Mutex<HashMap<String, User>>>,
}
impl UserRepoMemory {
    pub fn new() -> Self {
        let obj = Self {
            users: Arc::new(Mutex::new(HashMap::new())),
        };

        obj.add(User{
            email: "admin@example.com".to_string(),
            password: "admin".to_string(),
            name: "Admin".to_string(),
            created: Utc::now(),
            admin: true,
        });

        obj
    }
}

impl UserRepo for UserRepoMemory {
    fn get_by_email(&self, email: &str) -> Option<User> {
        self.users.lock().unwrap().get(email).cloned()
    }

    fn get_all(&self) -> Vec<User> {
        self.users.lock().unwrap().values().cloned().collect()
    }

    fn add(&self, user: User) {
        self.users.lock().unwrap().insert(user.email.clone(), user);
    }
}