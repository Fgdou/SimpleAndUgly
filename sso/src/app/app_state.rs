use crate::objects::config::Config;
use crate::objects::user::User;
use crate::services::factory::Services;
use std::sync::Mutex;

pub struct AppState {
    pub services: Services,
    pub user: Mutex<Option<(String, User)>>
}

impl AppState {
    pub fn new(config: &Config) -> Self {
        Self {
            services: Services::new(config),
            user: Mutex::new(None),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.user.lock().unwrap().is_some()
    }
}