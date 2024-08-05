use yew::prelude::*;

use super::super::components::Input;

use derive_more::Constructor;
use std::{fmt::Display, num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy, Constructor)]
struct Percetange(u32);

impl Display for Percetange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

impl FromStr for Percetange {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsing = s.parse::<u32>()?;
        Ok(Self::new(parsing))
    }
}

#[derive(Properties, PartialEq)]
struct InputPercentageProps {
    percentage: Option<Percetange>,
}

#[function_component]
fn InputPercentage(props: &InputPercentageProps) -> Html {
    html!(
        <Input<Percetange>
            value={props.percentage.map(|x|x.to_string()).unwrap_or_default()}
        />
    )
}
