use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum InputResult<T>
where
    T: Display,
{
    Result(T),
    Empty,
    ParsingError(String),
}

impl<T> Display for InputResult<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputResult::Result(value) => writeln!(f, "{}", value),
            InputResult::Empty => writeln!(f),
            InputResult::ParsingError(value) => writeln!(f, "{}", value),
        }
    }
}
