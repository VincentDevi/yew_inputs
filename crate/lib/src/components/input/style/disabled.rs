use std::ops::Deref;

use yew::html::IntoPropValue;

#[derive(PartialEq, Clone, Copy, Default)]
pub struct Disabled(bool);

impl Deref for Disabled {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoPropValue<Disabled> for bool {
    fn into_prop_value(self) -> Disabled {
        Disabled(self)
    }
}

impl From<bool> for Disabled {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl Disabled {
    pub fn background_color(&self) -> &'static str {
        match self.0 {
            true => "",
            false => "bg-gray-500",
        }
    }
    pub fn cursor(&self) -> &'static str {
        match self.0 {
            true => "cursor-not-allowed",
            false => "",
        }
    }
}
