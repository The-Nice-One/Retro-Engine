use std::fmt;

use unicode_segmentation::UnicodeSegmentation;

use super::components::trait_def::Component;





pub struct Scene {

}

impl Scene {
    pub fn new() -> Self {
        Self {}
    }

    pub fn add_component(&mut self, component: Box<impl Component>) {
        
    }
}


pub fn allign_horizontally(base: String, join: String) -> String {
    let mut base_vec: Vec<&str> = base.split("\n").collect();
    let mut join_vec: Vec<&str> = join.split("\n").collect();

    let mut cloned_vec = base_vec.clone();
    cloned_vec.sort_by(|a, b| b.graphemes(true).count().cmp(&a.graphemes(true).count()));
    let longest_line = cloned_vec[0].graphemes(true).count() as u32;
    let mut returned = String::from("");


    let adder = String::from(" ").repeat(longest_line as usize);
    let mut index = 0;

    if base_vec.len() > join_vec.len() {
        for _ in 0..base_vec.len()-join_vec.len() { join_vec.push(""); }
    } else if join_vec.len() > base_vec.len() {
        for _ in 0..join_vec.len()-base_vec.len() { base_vec.push(&adder); }
    }
    

    for line in base_vec {
        let lenght: u32 = line.graphemes(true).count() as u32;
        returned.push_str(line);
        
        if lenght < longest_line {
            for _ in 0..longest_line-lenght {
                returned.push_str(" ");
            }
        }

        returned.push_str(join_vec[index]);
        returned.push_str("\n");
        index += 1;
        
    }
    returned
}
