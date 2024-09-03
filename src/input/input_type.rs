use std::fmt::Display;

use yew::{html::IntoPropValue, AttrValue};

#[derive(PartialEq, Clone, Copy)]
pub enum InputType {
    Text,
    Number,
    Email,
    Date,
    Phone,
    Month,
    Search,
}

impl Default for InputType {
    fn default() -> Self {
        Self::Text
    }
}

impl Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputType::Text => write!(f, "text"),
            InputType::Number => write!(f, "number"),
            InputType::Email => write!(f, "email"),
            InputType::Date => write!(f, "date"),
            InputType::Phone => write!(f, "tel"),
            InputType::Month => write!(f, "month"),
            InputType::Search => write!(f, "search"),
        }
    }
}

impl IntoPropValue<InputType> for AttrValue {
    fn into_prop_value(self) -> InputType {
        match self {
            AttrValue::Static("number") => InputType::Number,
            AttrValue::Static("email") => InputType::Email,
            AttrValue::Static("date") => InputType::Date,
            AttrValue::Static("phone") => InputType::Phone,
            AttrValue::Static("month") => InputType::Month,
            AttrValue::Static("search") => InputType::Search,
            _ => InputType::Text,
        }
    }
}
