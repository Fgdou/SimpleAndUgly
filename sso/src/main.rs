use login_system::LoginSystem;

mod user;
mod login_system;


fn main() {
    let mut instance = LoginSystem::new("db.json".to_string());

    println!("{:?}", instance.verifyToken(&"I6haNrLAR3yxOnhF24fl1nGwyiyHtSmOxHOMIn9x0VpesfO9Eiwxzf9fJO5TGG0a"));
}
