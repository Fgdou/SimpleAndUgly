use std::sync::Arc;
use crate::objects::config::{Config, RepoType};
use crate::repos::applications::{ApplicationRepo, ApplicationRepoMemory};
use crate::repos::login_tokens::{LoginTokenRepo, LoginTokenRepoMemory};
use crate::repos::register_tokens::{RegisterTokenRepo, RegisterTokenRepoMemory};
use crate::repos::users::{UserRepo, UserRepoMemory};
use crate::services::auth::AuthService;

pub struct Repos {
    pub user_repo: Arc<dyn UserRepo>,
    pub login_token_repo: Arc<dyn LoginTokenRepo>,
    pub register_token_repo: Arc<dyn RegisterTokenRepo>,
    pub application_repo: Arc<dyn ApplicationRepo>,
}

pub struct Services {
    pub auth: AuthService
}

impl Services {
    pub fn new (config: &Config) -> Self {
        let repos = Repos::new(config);

        Self {
            auth: AuthService::new(config.clone(), repos)
        }
    }
}

impl Repos {
    pub fn new(config: &Config) -> Self {
        match config.repo_type {
            RepoType::Memory => Self::new_memory(),
        }
    }

    fn new_memory() -> Self {
        Self {
            login_token_repo: Arc::new(LoginTokenRepoMemory::new()),
            register_token_repo: Arc::new(RegisterTokenRepoMemory::new()),
            user_repo: Arc::new(UserRepoMemory::new()),
            application_repo: Arc::new(ApplicationRepoMemory::new()),
        }
    }
}