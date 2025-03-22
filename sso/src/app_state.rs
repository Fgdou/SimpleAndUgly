use std::sync::{Arc, Mutex};
use rusqlite::{Connection, OpenFlags};
use crate::objects::user::User;
use crate::repositories::applications::ApplicationRepo;
use crate::repositories::tokens::TokenRepo;
use crate::repositories::users::UserRepo;
use crate::services::apps::ApplicationService;
use crate::services::auth::Auth;

#[derive(Clone)]
pub struct AppState {
    pub auth: Arc<Auth>,
    pub user: Arc<Mutex<Option<User>>>,
    pub apps: Arc<ApplicationService>,
}
impl AppState {
    pub fn new(path: &str) -> Self {
        let connection = Arc::new(Mutex::new(
            Connection::open_with_flags(
                path,
                OpenFlags::default() | OpenFlags::SQLITE_OPEN_FULL_MUTEX
            ).unwrap()
        ));
        let tokens = Arc::new(TokenRepo::new(connection.clone()));
        let users = Arc::new(UserRepo::new(connection.clone()));

        let auth = Arc::new(Auth {
            user_repo: users,
            token_repo: tokens,
        });

        if let None = auth.user_repo.get_user_by_email("admin@example.com") {
            println!("Info: Creating Admin user");
            auth.user_repo.add_user(
                User::new("admin@example.com".to_string(), "Admin".to_string(), "admin".to_string())
            );
        }

        let apps = ApplicationService {
            application: Arc::new(ApplicationRepo::new(connection)),
        };

        Self {
            auth,
            user: Arc::new(Mutex::new(None)),
            apps: Arc::new(apps),
        }

    }
}