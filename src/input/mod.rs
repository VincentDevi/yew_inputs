use std::{fmt::Display, str::FromStr};
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod context;
mod error;
mod input_type;
mod value;

pub use context::*;
pub use error::*;
use input_type::*;
pub use value::*;

#[derive(Properties, PartialEq, Clone)]
pub struct InputProps<T>
where
    T: PartialEq + Display,
{
    #[prop_or_default]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub value: InputValue<T>,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub left_icon: Html,
    #[prop_or_default]
    pub right_icon: Html,
    #[prop_or_default]
    pub onchange: Callback<InputValue<T>>,
    #[prop_or_default]
    pub input_type: InputType,
}

#[function_component]
pub fn Input<T>(props: &InputProps<T>) -> Html
where
    T: PartialEq + Clone + Display + FromStr + 'static,
{
    let style = use_safe_input_style()
        .map(|x| x.style())
        .unwrap_or_else(|| InputStyleBuilder::new().build());

    let onchange = {
        let onchange = props.onchange.clone();
        let required = props.required;
        Callback::from(move |e: Event| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            if value.is_empty() {
                if required {
                    onchange.emit(InputValue::empty_error());
                    return;
                } else {
                    onchange.emit(InputValue::empty());
                    return;
                }
            }

            let parse_value = value.parse::<T>();
            let result = match parse_value {
                Ok(v) => InputValue::value(v),
                Err(_) => InputValue::parsing_error(value),
            };
            onchange.emit(result);
        })
    };

    let border = match &*props.value {
        Ok(_) => style.valid_border,
        Err(_) => style.invalid_border,
    };
    let background_color = match &*props.value {
        Ok(_) => style.enabled_background_color,
        Err(_) => style.disabled_background_color,
    };
    let cursor = match &*props.value {
        Ok(_) => style.enabled_cursor,
        Err(_) => style.disabled_cursor,
    };

    html!(
        <div class={match style.override_outside_div_class{
            Some(v)=>v.to_string(),
            None=>format!("w-full h-full {} {}  ",border,background_color)}}
        >
            {props.left_icon.clone()}
            <input
                value={props.value.to_string()}
                class={match style.override_input_class{
                    Some(v)=>v.to_string(),
                    None=>format!("border-none focus:border-transparent focus:outline-none px-2.5 {} ",cursor)
                }}
                name={&props.name}
                {onchange}
                required={props.required}
                type={props.input_type.to_string()}
                placeholder={&props.placeholder}
            />
            {props.right_icon.clone()}
        </div>
    )
}
