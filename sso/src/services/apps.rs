use std::sync::Arc;
use regex::Regex;
use serde::Deserialize;
use crate::errors::apps::CreateApplicationError;
use crate::objects::application::Application;
use crate::objects::user::User;
use crate::repositories::applications::ApplicationRepo;

pub struct ApplicationService {
    pub application: Arc<ApplicationRepo>
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateApp {
    pub name: String,
    pub base_url: String,
}

impl ApplicationService {
    pub fn is_user_authorized(&self, app: &Application, user: &User) -> bool {
        true
    }
    pub fn create_app(&self, app: &CreateApp) -> Result<Application, CreateApplicationError> {
        ApplicationService::validate_name(&app.name)?;
        let app = Application {
            name: app.name.clone(),
            base_url: app.base_url.clone(),
            client_id: String::new(),
            client_secret: String::new(),
        };
        self.application.create(&app);
        Ok(app)
    }
    fn validate_name(name: &str) -> Result<(), CreateApplicationError> {
        if name.len() < 2 {
            return Err(CreateApplicationError::Validation {
                error: "Length must be >= 2".to_string(),
                field: "name".to_string(),
            });
        }
        if !Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").unwrap().is_match(name) {
            return Err(CreateApplicationError::Validation {
                error: "Should be only lower case and dash (-)".to_string(),
                field: "name".to_string(),
            });
        }
        Ok(())
    }

    fn validate_url(url: &str) -> Result<(), CreateApplicationError> {
        match Regex::new(r"^https?://[a-z0-9-]+(\.[a-z0-9-]+)*(:[0-9]*)?/?$").unwrap().is_match(url) {
            false => Err(CreateApplicationError::Validation {
                field: "base_url".to_string(),
                error: "url is not valid".to_string(),
            }),
            true => Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_validation_name() {
        assert_eq!(false, ApplicationService::validate_name("").is_ok());
        assert_eq!(false, ApplicationService::validate_name("a").is_ok());
        assert_eq!(false, ApplicationService::validate_name("1").is_ok());
        assert_eq!(true, ApplicationService::validate_name("ab").is_ok());
        assert_eq!(true, ApplicationService::validate_name("a1").is_ok());
        assert_eq!(true, ApplicationService::validate_name("a-1").is_ok());
        assert_eq!(false, ApplicationService::validate_name("a--1").is_ok());
        assert_eq!(false, ApplicationService::validate_name("a1-").is_ok());
        assert_eq!(true, ApplicationService::validate_name("a1-d").is_ok());
        assert_eq!(true, ApplicationService::validate_name("a1-ddsa-238fhsusd-r3fe").is_ok());
        assert_eq!(false, ApplicationService::validate_name("aZ").is_ok());
        assert_eq!(false, ApplicationService::validate_name("ad%").is_ok());
        assert_eq!(false, ApplicationService::validate_name("ad.").is_ok());
    }

    #[test]
    fn test_validate_url() {
        assert_eq!(false, ApplicationService::validate_url("").is_ok());
        assert_eq!(false, ApplicationService::validate_url("salut").is_ok());
        assert_eq!(true, ApplicationService::validate_url("https://example.com").is_ok());
        assert_eq!(true, ApplicationService::validate_url("https://example.com/").is_ok());
        assert_eq!(true, ApplicationService::validate_url("http://example.com/").is_ok());
        assert_eq!(false, ApplicationService::validate_url("https://example.com/fsl").is_ok());
        assert_eq!(false, ApplicationService::validate_url("httpsd://example.com/fsl").is_ok());
    }
}