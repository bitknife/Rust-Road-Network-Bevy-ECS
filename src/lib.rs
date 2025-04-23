pub mod core;
mod render;

// Re-export key types and systems for easier access
pub use core::components::{Road, Settlement, MovingTo};
pub use core::resources::SimulationConfig;
pub use core::systems::{generate_world, npc_random_movement_system};

pub use render::systems::{
    render_settlements_system, 
    render_roads_system, 
    render_npcs_system,
    sync_npc_visuals_system};