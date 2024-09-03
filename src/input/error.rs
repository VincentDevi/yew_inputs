use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum InputError {
    #[error("cannot_be_empty")]
    Empty,
    #[error("parsing_error `{0}` ")]
    ParsingError(String),
    #[error("`{display}`")]
    ValidationError { value: String, display: String },
}

impl InputError {
    pub fn new(value: impl Into<String>, display: impl Into<String>) -> InputError {
        InputError::ValidationError {
            value: value.into(),
            display: display.into(),
        }
    }

    pub fn get_values(&self) -> String {
        match self {
            InputError::Empty => String::new(),
            InputError::ParsingError(value) => value.clone(),
            InputError::ValidationError { value, .. } => value.clone(),
        }
    }
}
