use std::fmt;
use super::trait_def::Component;



pub struct Grid {
    data: Vec<&'static str>,
    size: (u32, u32)
}

impl Grid {
    pub fn new(size: (u32, u32), default: &'static str) -> Self {
        Self {
            data: vec![default; (size.0 * size.1) as usize],
            size: size
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut writer_x = 0;
        for index in 0..self.size.0 * self.size.1 {
            write!(f, "{}", self.data[index as usize])?;

            writer_x += 1;

            if writer_x > self.size.1 - 1 {
                writer_x = 0;
                write!(f, "\n")?;
            }
        }
        write!(f, "")
    }
}


impl Component for Grid {
    fn display(&self) -> String {
        format!("{}", self)    
    }
}

