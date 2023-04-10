use super::health_component::HealthComponent;
use std::fmt::{self, Display};
use std::io::Write;

/// Human Character for the player to control.
#[derive(Clone)]
pub struct Human {
    health: HealthComponent<u16>
}

impl Human {
    pub fn new() -> Self {
        Self {
           health: HealthComponent::<u16>::new(100, 100)
        }
    }
}

impl Display for Human {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "☺")
    }
}