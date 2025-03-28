#[derive(Clone)]
pub enum RepoType {
    Memory,
}

#[derive(Clone)]
pub struct Config {
    pub repo_type: RepoType,
    pub restrict_registration: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            repo_type: RepoType::Memory,
            restrict_registration: true,
        }
    }
}