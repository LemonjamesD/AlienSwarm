pub struct HealthComponent<H> {
    pub current_health: H,
    pub max_health: H
}

impl<H> HealthComponent<H> {
    pub fn new(current_health: H, max_health: H) -> Self {
        Self {
            current_health,
            max_health
        }
    }
}
