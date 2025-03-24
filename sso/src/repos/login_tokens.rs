use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::objects::login_token::LoginToken;

pub trait LoginTokenRepo {
    fn get_by_value(&self, value: &str) -> Option<LoginToken>;
    fn get_all(&self) -> Vec<LoginToken>;
    fn add(&self, token: LoginToken);
    fn delete(&self, token: &str);
}

pub struct LoginTokenRepoMemory {
    tokens: Arc<Mutex<HashMap<String, LoginToken>>>
}

impl LoginTokenRepoMemory {
    pub fn new() -> Self {
        Self {
            tokens: Arc::new(Mutex::new(HashMap::new()))
        }
    }
}
impl LoginTokenRepo for LoginTokenRepoMemory {

    fn get_by_value(&self, value: &str) -> Option<LoginToken> {
        self.tokens.lock().unwrap().get(value).cloned()
    }

    fn get_all(&self) -> Vec<LoginToken> {
        self.tokens.lock().unwrap().values().cloned().collect()
    }

    fn add(&self, token: LoginToken) {
        self.tokens.lock().unwrap().insert(token.value.clone(), token);
    }

    fn delete(&self, token: &str) {
        self.tokens.lock().unwrap().remove(token);
    }
}