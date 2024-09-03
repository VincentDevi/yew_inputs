use std::ops::Deref;

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct InputValue<T>(Result<Option<T>, InputError>);

impl<T> Default for InputValue<T> {
    fn default() -> Self {
        Self(Ok(None))
    }
}

impl<T> Display for InputValue<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Err(err) => write!(f, "{}", err),
            Ok(value) => match value {
                None => write!(f, ""),
                Some(v) => write!(f, "{}", v),
            },
        }
    }
}

impl<T> Deref for InputValue<T> {
    type Target = Result<Option<T>, InputError>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> InputValue<T> {
    pub fn value(value: T) -> InputValue<T> {
        InputValue(Ok(Some(value)))
    }

    pub fn empty() -> InputValue<T> {
        InputValue(Ok(None))
    }

    pub fn empty_error() -> InputValue<T> {
        InputValue(Err(InputError::Empty))
    }

    pub fn parsing_error(value: String) -> InputValue<T> {
        InputValue(Err(InputError::ParsingError(value)))
    }

    pub fn validation_error(value: String, display: String) -> InputValue<T> {
        InputValue(Err(InputError::new(value, display)))
    }
}

impl<T> From<Option<T>> for InputValue<T> {
    fn from(value: Option<T>) -> Self {
        Self(Ok(value))
    }
}

impl<T> From<T> for InputValue<T> {
    fn from(value: T) -> Self {
        Self(Ok(Some(value)))
    }
}
