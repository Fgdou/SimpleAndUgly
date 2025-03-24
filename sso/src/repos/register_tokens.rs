use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{Days, Utc};
use crate::objects::registration_token::RegisterToken;

pub trait RegisterTokenRepo {
    fn get_by_value(&self, value: &str) -> Option<RegisterToken>;
    fn get_all(&self) -> Vec<RegisterToken>;
    fn add(&self, token: RegisterToken);
    fn delete(&self, token: &str);
}

pub struct RegisterTokenRepoMemory {
    tokens: Arc<Mutex<HashMap<String, RegisterToken>>>
}


impl RegisterTokenRepoMemory {
    pub fn new() -> Self {
        let obj = Self {
            tokens: Arc::new(Mutex::new(HashMap::new())),
        };

        obj.add(RegisterToken{
            value: "token".to_string(),
            expiration: Utc::now() + Days::new(10),
        });

        obj
    }
}
impl RegisterTokenRepo for RegisterTokenRepoMemory {

    fn get_by_value(&self, value: &str) -> Option<RegisterToken> {
        self.tokens.lock().unwrap().get(value).cloned()
    }

    fn get_all(&self) -> Vec<RegisterToken> {
        self.tokens.lock().unwrap().values().cloned().collect()
    }

    fn add(&self, token: RegisterToken) {
        self.tokens.lock().unwrap().insert(token.value.clone(), token);
    }

    fn delete(&self, token: &str) {
        self.tokens.lock().unwrap().remove(token);
    }
}