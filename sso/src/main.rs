use std::{rc::Rc, sync::Arc};

use repositories::{tokens::{TokenRepo, TokenRepoSQLite}, users::UserRepoSQLite};
use rusqlite::Connection;
use services::auth::{Auth, BasicAuth, UserRequest};

mod errors;
mod objects;
mod repositories;
mod services;

pub struct Instance {
    auth: Arc<dyn Auth>
}
impl Instance {
    fn new(path: &str) -> Instance {
        let conn = Rc::new(Connection::open(path).unwrap());

        let tokens = Rc::new(TokenRepoSQLite::new(conn.clone()));
        let users = Rc::new(UserRepoSQLite::new(conn.clone()));

        let auth = BasicAuth {
            token_repo: tokens,
            user_repo: users,
        };

        Instance {
            auth: Arc::new(auth)
        }
    }
}

fn main() {
    println!("Hello, world!");


}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{errors::login::{AuthError, LoginError}, objects::user::User};

    use super::*;

    fn instance_with_admin() -> Instance {
        let path = "/tmp/testdb.sqlite";
        let _ = fs::remove_file(path);
        let instance = Instance::new(path);

        let token = instance.auth.create_register_token("admin@example.com".to_string());
        instance.auth.register(&token.value, UserRequest {
            email: "admin@example.com".to_string(),
            name: "Admin".to_string(),
            password: "admin".to_string(),
        }).unwrap();

        instance
    }

    #[test]
    fn test_login() {
        let instance = instance_with_admin();

        assert_eq!(Err(LoginError::InvalidEmail), instance.auth.login("test@test.test", "test"));
        assert_eq!(Err(LoginError::InvalidPassword), instance.auth.login("admin@example.com", "test"));

        let token = instance.auth.login("admin@example.com", "admin");
        assert_eq!(true, token.is_ok());
        let token = token.unwrap();

        assert_eq!(Err(AuthError::TokenNotExist), instance.auth.authenticate("test"));
        assert_eq!(
            Ok(User::new("admin@example.com".to_string(), "admin".to_string(), "admin".to_string())),
            instance.auth.authenticate(&token.value)
        );
    }
}
