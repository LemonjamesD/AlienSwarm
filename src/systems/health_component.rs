use std::ops::{Add, Sub};
use std::cmp::{Ord, min, max};

/// HealthComponent struct to make working with health easier.
pub struct HealthComponent<H> {
    pub current_health: H,
    pub max_health: H
}

/// A Trait used for the Generics in HealthComponent.
pub trait HealthComponentTrait<H> = Add<Output = H> + Sub<Output = H> + Ord + Default + Copy;

impl<H: HealthComponentTrait<H>> HealthComponent<H> {
    pub fn new(current_health: H, max_health: H) -> Self {
        Self {
            current_health,
            max_health
        }
    }

    /// Adds Health.
    pub fn add(&mut self, rhs: H) -> &mut Self {
        self.current_health = max(H::default(), min(self.max_health, self.current_health + rhs));
        self
    }

    /// Remove Health.
    pub fn sub(&mut self, rhs: H) -> &mut Self {
        self.current_health = max(H::default(), min(self.max_health, self.current_health - rhs));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::systems::health_component::HealthComponent;
    
    #[test]
    fn health_add_test() {
        let mut health = HealthComponent::new(50, 100);
        health.add(50);
        assert_eq!(100, health.current_health);
    }

    #[test]
    fn health_sub_test() {
        let mut health = HealthComponent::new(100, 100);
        health.sub(50);
        assert_eq!(50, health.current_health);
    }
}