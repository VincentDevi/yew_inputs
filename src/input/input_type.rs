use std::fmt::Display;

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