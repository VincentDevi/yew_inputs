use std::{fmt::Display, str::FromStr};

use yew::{html::IntoPropValue, AttrValue};

#[derive(Debug, PartialEq, Clone)]
pub enum InputResult<T>
where
    T: Display,
{
    Result(T),
    Empty,
    ParsingError(ParsingError),
}

impl<T> Display for InputResult<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputResult::Result(value) => writeln!(f, "{}", value),
            InputResult::Empty => writeln!(f),
            InputResult::ParsingError(value) => writeln!(f, "{}", value.old_value),
        }
    }
}

impl<T> IntoPropValue<AttrValue> for InputResult<T>
where
    T: Display + FromStr + 'static,
    <T as FromStr>::Err: Display,
{
    fn into_prop_value(self) -> AttrValue {
        match self {
            InputResult::Empty => "".to_string(),
            InputResult::ParsingError(v) => v.old_value,
            InputResult::Result(v) => v.to_string(),
        }
        .into()
    }
}

// Exist only because of the input number
// Since it always returns an empty string
// when using oninput, it will clear our input, that's why I keep track of the old value
// otherwise, our input will be confusing for our users

#[derive(Debug, PartialEq, Clone)]
pub struct ParsingError {
    pub old_value: String,
    pub error: String,
}

impl ParsingError {
    pub fn new(old_value: &str, error: &str) -> Self {
        Self {
            old_value: old_value.to_string(),
            error: error.to_string(),
        }
    }
}
