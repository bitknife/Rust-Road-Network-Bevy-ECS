pub mod components;
pub mod resources;
pub mod systems;
mod factories;

// Re-export core types for easier access
pub use components::{Road, Settlement, NPC, MovingTo};

pub use resources::{ SimulationConfig, NpcMoveTimer };
