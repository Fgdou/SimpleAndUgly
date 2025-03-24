use std::collections::HashSet;

pub struct Application {
    pub name: String,
    pub url: String,
    pub client_id: String,
    pub client_secret: String,
    pub users: HashSet<String>,
}