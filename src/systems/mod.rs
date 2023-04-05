pub mod blocks;

pub mod health_component;
pub mod human;

use crate::systems::blocks::{air::AirBlock, dirt::DirtBlock};
use crate::systems::human::Human;

pub enum Thing {
    Human(Human),
    Dirt(DirtBlock),
    Air(AirBlock),
    Newline
}
