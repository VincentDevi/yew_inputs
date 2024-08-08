use yew::prelude::*;

use lib::components::*;
use std::{fmt::Display, num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Percentange(u64);

impl Percentange {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}

impl Display for Percentange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Percentange {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsing = s.parse::<u64>()?;
        Ok(Self::new(parsing))
    }
}

#[derive(Properties, PartialEq)]
pub struct InputPercentageProps {
    pub percentage: InputResult<Percentange>,
    pub on_input: Callback<InputResult<Percentange>>,
    #[prop_or(true)]
    pub is_valid: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub placeholder: AttrValue,
}

#[function_component]
pub fn InputPercentage(props: &InputPercentageProps) -> Html {
    let value = match &props.percentage {
        InputResult::Result(v) => v.to_string(),
        InputResult::Empty => String::default(),
        InputResult::ParsingError(v) => v.old_value.clone(),
    };
    html!(
        <Input<Percentange>
            {value}
            on_input={&props.on_input}
            right_icon={html!({"%"})}
            is_valid={props.is_valid}
            disabled={props.disabled}
            placeholder={&props.placeholder}
            input_type={InptuType::Number}
        />
    )
}
