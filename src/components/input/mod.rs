mod style;

use yew::prelude::*;

use super::super::errors::Error as InputError;
use std::{fmt::Display, str::FromStr};
use style::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq, Clone)]
pub struct InputProps<T>
where
    T: PartialEq,
{
    #[prop_or_default]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub disabled: Disabled,
    #[prop_or_default]
    pub value: AttrValue,
    #[prop_or_default]
    pub is_valid: Validity,
    #[prop_or_default]
    pub left_icon: Html,
    #[prop_or_default]
    pub right_icon: Html,
    #[prop_or_default]
    pub on_input: Callback<Result<T, InputError>>,
    pub input_type: InptuType,
}

#[derive(PartialEq, Clone, Copy, Default, strum_macros::Display)]
pub enum InptuType {
    #[strum(to_string = "text")]
    #[default]
    Text,
    #[strum(to_string = "number")]
    Number,
    #[strum(to_string = "email")]
    Email,
}

#[function_component]
pub fn Input<T>(props: &InputProps<T>) -> Html
where
    T: PartialEq + Clone + Display + FromStr + 'static,
    <T as FromStr>::Err: std::fmt::Debug + Display,
{
    let oninput = {
        let on_input = props.on_input.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            if value.is_empty() {
                on_input.emit(Err(InputError::Empty));
                return;
            }
            let parse_value = value
                .parse::<T>()
                .map_err(|err| InputError::Parsing(err.to_string()));
            on_input.emit(parse_value);
        })
    };

    html!(
        <div class={format!("w-full h-full {} {} ",props.is_valid.border(),props.disabled.background_color())}>
            {props.left_icon.clone()}
            <input
                value={&props.value}
                class={format!("border-none focus:border-transparent focus:outline-none px-2.5 {} ",props.disabled.cursor())}
                {oninput}
                type={props.input_type.to_string()}
                placeholder={&props.placeholder}
            />
            {props.right_icon.clone()}
        </div>
    )
}
