use std::ops::Deref;

#[derive(PartialEq, Clone, Copy)]
pub struct Validity(bool);

impl Deref for Validity {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Validity {
    fn default() -> Self {
        Self(true)
    }
}

impl From<bool> for Validity {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl Validity {
    pub fn border(&self) -> &'static str {
        match self.0 {
            true => "border rounded-lg border-blue-500 focus-within:border-blue-700 active-within:border-blue-700  focus-within:outline-blue-700",
            false => "border rounded-lg border-red-500 focus-within:border-red-700 active-within:border-red-700  focus-within:outline-red-700",
        }
    }
}
