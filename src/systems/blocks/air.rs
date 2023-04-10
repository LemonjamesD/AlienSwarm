use std::fmt::{self, Display};
use std::io::Write;

#[derive(Clone, Copy)]
pub struct AirBlock {
    
}

impl AirBlock {
    pub fn new() -> Self {
        Self {
            
        }
    }
}

impl Display for AirBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " ")
    }
}