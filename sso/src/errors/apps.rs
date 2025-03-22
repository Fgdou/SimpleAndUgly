use derive_more::Error;
use std::fmt::Display;


#[derive(PartialEq)]
pub enum CreateApplicationError {
    Validation {
        field: String,
        error: String,
    }
}
