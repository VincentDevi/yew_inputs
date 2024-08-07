use yew::prelude::*;

//use super::super::components::{Input, InputResult};
use std::{fmt::Display, num::ParseIntError, option::Option, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Percentange(u32);

impl Percentange {
    pub fn new(value: u32) -> Self {
        Self(value)
    }
}

impl Display for Percentange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

impl FromStr for Percentange {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsing = s.parse::<u32>()?;
        Ok(Self::new(parsing))
    }
}

#[derive(Properties, PartialEq)]
struct InputPercentageProps {
    pub percentage: Option<Percentange>,
    // pub on_input: Callback<InputResult<Percentange>>,
    #[prop_or(true)]
    pub is_valid: bool,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component]
fn InputPercentage(props: &InputPercentageProps) -> Html {
    html!(
    //        <Input<Percentange>
     //           value={props.percentage.map(|x|x.to_string()).unwrap_or_default()}
      //          on_input={props.on_input.clone()}
       //         right_icon={html!({"%"})}
       //         is_valid={props.is_valid}
       //         disabled={props.disabled}
        //    />
        )
}
