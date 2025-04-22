pub mod core;

// Re-export key types and systems for easier access
pub use core::components::{Road, Settlement, Waypoint, WaypointKind};
pub use core::systems::{build_smart_roads, ensure_connected_network};
pub use core::factories::spawn_settlements;
