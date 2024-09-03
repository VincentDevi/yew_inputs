use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputStyleContextProps {
    #[prop_or_default]
    pub input_style: Option<InputStyle>,
    pub children: Html,
}

#[function_component]
pub fn InputStyleContext(props: &InputStyleContextProps) -> Html {
    let context = Context {
        input_style: props
            .input_style
            .unwrap_or_else(|| InputStyleBuilder::new().build()),
    };
    html!(
        <ContextProvider<Context> {context} >
        { props.children.clone() }
        </ContextProvider<Context>>
    )
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Context {
    input_style: InputStyle,
}
impl Context {
    pub fn style(&self) -> InputStyle {
        self.input_style
    }
}

#[hook]
pub fn use_safe_input_style() -> Option<Context> {
    use_context::<Context>()
}

#[hook]
pub fn use_unsafe_input_style() -> Context {
    use_context::<Context>().expect("cannot call this hook outside of InputStyleContext")
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct InputStyle {
    pub enabled_background_color: &'static str,
    pub disabled_background_color: &'static str,
    pub enabled_cursor: &'static str,
    pub disabled_cursor: &'static str,
    pub valid_border: &'static str,
    pub invalid_border: &'static str,
    pub override_outside_div_class: Option<&'static str>,
    pub override_input_class: Option<&'static str>,
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct InputStyleBuilder {
    pub enabled_background_color: Option<&'static str>,
    pub disabled_background_color: Option<&'static str>,
    pub enabled_cursor: Option<&'static str>,
    pub disabled_cursor: Option<&'static str>,
    pub valid_border: Option<&'static str>,
    pub invalid_border: Option<&'static str>,
    pub override_outside_div_class: Option<&'static str>,
    pub override_input_class: Option<&'static str>,
}

impl InputStyleBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn enabled_background_color(&mut self, value: &'static str) -> &mut Self {
        self.enabled_background_color = Some(value);
        self
    }
    pub fn disabled_background_color(&mut self, value: &'static str) -> &mut Self {
        self.disabled_background_color = Some(value);
        self
    }
    pub fn enabled_cursor(&mut self, value: &'static str) -> &mut Self {
        self.enabled_cursor = Some(value);
        self
    }
    pub fn disabled_cursor(&mut self, value: &'static str) -> &mut Self {
        self.disabled_cursor = Some(value);
        self
    }
    pub fn valid_border(&mut self, value: &'static str) -> &mut Self {
        self.valid_border = Some(value);
        self
    }
    pub fn invalid_border(&mut self, value: &'static str) -> &mut Self {
        self.invalid_border = Some(value);
        self
    }
    pub fn override_input_class(&mut self, value: &'static str) -> &mut Self {
        self.override_input_class = Some(value);
        self
    }
    pub fn override_outside_div_class(&mut self, value: &'static str) -> &mut Self {
        self.override_outside_div_class = Some(value);
        self
    }
    pub fn build(&self) -> InputStyle {
        InputStyle {
            enabled_background_color: self.enabled_background_color.unwrap_or(""),
            disabled_background_color: self.disabled_background_color.unwrap_or("bg-gray-500"),
            enabled_cursor: self.enabled_cursor.unwrap_or(""),
            disabled_cursor: self.disabled_cursor.unwrap_or("cursor-not-allowed"),
            valid_border: self.valid_border.unwrap_or("border rounded-lg border-lime-500 focus-within:border-blue-700 active-within:border-blue-700  focus-within:outline-blue-700"),
            invalid_border: self.invalid_border.unwrap_or("border rounded-lg border-red-500 focus-within:border-red-700 active-within:border-red-700  focus-within:outline-red-700"),
            override_input_class: self.override_input_class,
            override_outside_div_class: self.override_outside_div_class,
        }
    }
}
