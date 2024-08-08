mod input_result;
mod style;

use input_result::ParsingError;
use yew::prelude::*;

pub use input_result::InputResult;
use std::{fmt::Display, str::FromStr};
pub use style::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq, Clone)]
pub struct InputProps<T>
where
    T: PartialEq + Display,
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
    pub on_input: Callback<InputResult<T>>,
    #[prop_or_default]
    pub input_type: InptuType,
}

#[derive(PartialEq, Clone, Copy)]
pub enum InptuType {
    Text,
    Number, // Worst Input, when value is Invalid, always return an empty string...
    Email,
    Date,
}

impl Default for InptuType {
    fn default() -> Self {
        Self::Text
    }
}

impl Display for InptuType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InptuType::Text => write!(f, "text"),
            InptuType::Number => write!(f, "number"),
            InptuType::Email => write!(f, "email"),
            InptuType::Date => write!(f, "date"),
        }
    }
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
                on_input.emit(InputResult::Empty);
                return;
            }

            let parse_value = value.parse::<T>();
            let result = match parse_value {
                Ok(v) => InputResult::Result(v),
                Err(err) => InputResult::ParsingError(ParsingError::new(
                    value.as_str(),
                    err.to_string().as_str(),
                )),
            };
            on_input.emit(result);
        })
    };

    html!(
        <div class={format!("w-full h-full {} {} border ",props.is_valid.border(),props.disabled.background_color())}>
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
