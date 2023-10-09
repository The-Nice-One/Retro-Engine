use std::fmt;
use super::trait_def::Component;

pub struct VerticalLine {
    height: u32,
    design: &'static str
}

impl VerticalLine {
    pub fn new(design: &'static str, height: u32) -> Self {
        Self {
            height: height,
            design: design
        }
    }
}

impl fmt::Display for VerticalLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 1..self.height {
            
            write!(f, "{}\n", self.design)?;
        }
        write!(f, "{}", self.design)
    }
}

impl Component for VerticalLine {
    fn display(&self) -> String {
        format!("{}", self)
    }
}