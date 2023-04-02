use super::health_component::HealthComponent;

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
