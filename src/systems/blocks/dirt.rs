use std::fmt::{self, Display};
use std::io::Write;

#[derive(Clone, Copy)]
pub struct DirtBlock {
    
}

impl DirtBlock {
    pub fn new() -> Self {
        Self {
            
        }
    }
}

impl Display for DirtBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "D")
    }
}