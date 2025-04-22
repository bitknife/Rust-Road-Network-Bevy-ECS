pub mod core;
mod render;

// Re-export key types and systems for easier access
pub use core::components::{Road, Settlement};
pub use core::resources::SimulationConfig;
pub use core::systems::{generate_world};

pub use render::systems::{render_settlements_system, render_roads_system};