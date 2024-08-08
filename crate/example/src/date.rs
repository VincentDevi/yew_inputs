use yew::prelude::*;

use lib::components::*;

#[derive(Properties, PartialEq)]
pub struct InputDateProps {
    pub date: InputResult<String>,
    pub on_input: Callback<InputResult<String>>,
    #[prop_or(true)]
    pub is_valid: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub placeholder: AttrValue,
}

#[function_component]
pub fn InputDate(props: &InputDateProps) -> Html {
    let value = match &props.date {
        InputResult::Result(v) => v.clone(),
        InputResult::Empty => String::default(),
        InputResult::ParsingError(v) => v.old_value.to_string(),
    };
    html!(
        <Input<String>
            {value}
            on_input={&props.on_input}
            is_valid={props.is_valid}
            disabled={props.disabled}
            placeholder={&props.placeholder}
            input_type={InptuType::Date}
        />
    )
}
