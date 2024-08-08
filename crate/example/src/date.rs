use yew::prelude::*;

use lib::*;

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
    html!(
        <Input<String>
            value={props.date.clone()}
            on_input={&props.on_input}
            is_valid={props.is_valid}
            disabled={props.disabled}
            placeholder={&props.placeholder}
            input_type={InputType::Date}
        />
    )
}
