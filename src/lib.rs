pub mod model;
pub mod utils;

pub use model::{Waypoint, Road, RoadNetwork, Settlement, WaypointKind};
pub use utils::spawn_settlements;
pub use utils::build_smart_roads;
