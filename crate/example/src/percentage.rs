use yew::prelude::*;

use lib::*;
use std::{fmt::Display, num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Percentage(u64);

impl Percentage {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}

impl Display for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Percentage {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsing = s.parse::<u64>()?;
        Ok(Self::new(parsing))
    }
}

#[derive(Properties, PartialEq)]
pub struct InputPercentageProps {
    pub percentage: InputResult<Percentage>,
    pub on_input: Callback<InputResult<Percentage>>,
    #[prop_or(true)]
    pub is_valid: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub placeholder: AttrValue,
}

#[function_component]
pub fn InputPercentage(props: &InputPercentageProps) -> Html {
    html!(
        <Input<Percentage>
            value={props.percentage.clone()}
            on_input={&props.on_input}
            right_icon={html!({"%"})}
            is_valid={props.is_valid}
            disabled={props.disabled}
            placeholder={&props.placeholder}
            input_type={InputType::Number}
        />
    )
}
