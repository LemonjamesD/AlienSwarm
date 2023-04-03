use std::ops::{Add, Sub};
use std::cmp::{Ord, min, max};


pub struct HealthComponent<H> {
    pub current_health: H,
    pub max_health: H
}

pub trait HealthComponentTrait<H> = Add<Output = H> + Sub<Output = H> + Ord + Default + Copy;

impl<H: HealthComponentTrait<H>> HealthComponent<H> {
    pub fn new(current_health: H, max_health: H) -> Self {
        Self {
            current_health,
            max_health
        }
    }

    pub fn add(&mut self, rhs: H) -> &mut Self {
        self.current_health = max(H::default(), min(self.max_health, self.current_health + rhs));
        self
    }

    pub fn sub(&mut self, rhs: H) -> &mut Self {
        self.current_health = max(H::default(), min(self.max_health, self.current_health - rhs));
        self
    }
}
