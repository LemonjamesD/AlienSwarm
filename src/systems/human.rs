use super::health_component::HealthComponent;

/// Human Character for the player to control.
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
