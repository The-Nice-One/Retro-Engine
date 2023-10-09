use std::fmt;
use super::trait_def::Component;

pub struct Text {
    pub string: &'static str,
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl Component for Text {
    fn display(&self) -> String {
        format!("{}", self)
    }
}

impl Text {
    pub fn new(string: &'static str) -> Self {
        Self {
            string: string
        }
    }
}
