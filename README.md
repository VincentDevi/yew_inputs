# Yew Input Component

## Introduction 
This Rust library for Yew provides a highly configurable and robust Input component that can handle various input types (text, number, email, etc.) and integrates error handling directly within the component. The Input component is designed to be easy to use while offering full customization of styles via a builder (InputStyleBuilder) or context (InputStyleContext).

## Features
- Automatic Parsing
- Integreted Error Handling
- Style Customization
- Support Various Input Types

## Usage
### Basic Example

Here’s how to use the Input component with a u32 type:

```
use yew::prelude::*;
use yew_input::*;

#[function_component(App)]
fn app() -> Html {
    let value = use_state(|| InputValue::<u32>::default());

    let onchange = {
        let value = value.clone();
        Callback::from(move |new_value: InputValue<u32>| {
            value.set(new_value);
        })
    };

    html! {
        <Input<u32>
            placeholder="Enter a number"
            input_type={"number"}
            value={(*value).clone()}
            {onchange}
            required=true
        />
    }
}
```
### Error Handling
The Input component automatically handles input and parsing errors. You can check if the value is valid and display a custom error message:

```
if let Err(err) = &*value {
    html! { <p>{ format!("Error: {}", err) }</p> }
}
```
### Style Customization
You can customize the styles using the InputStyleBuilder:

```
let custom_style = InputStyleBuilder::new()
    .enabled_background_color("bg-white")
    .disabled_background_color("bg-gray-500")
    .enabled_cursor("cursor-pointer")
    .disabled_cursor("cursor-not-allowed")
    .valid_border("border-green-500")
    .invalid_border("border-red-500")
    .build();

```

#### Using Style Context
To apply a custom style globally to multiple Input components, you can use the InputStyleContext:

```
html! {
    <InputStyleContext input_style={Some(custom_style)}>
        <Input<u32> placeholder="Enter a number" />
        <Input<String> placeholder="Enter a text" />
    </InputStyleContext>
}
```
