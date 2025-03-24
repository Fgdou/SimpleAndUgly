pub enum ValidationEnumError {
    Empty,
    Size(i32, i32),
    Regex(String),
}
pub struct ValidationError {
    pub field: String,
    pub error: ValidationEnumError,
}