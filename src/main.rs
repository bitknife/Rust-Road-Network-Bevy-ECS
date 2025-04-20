use road_network::{spawn_settlements, build_smart_roads, RoadNetwork, Waypoint, Settlement};
use std::collections::HashMap;

fn main() {
    let (settlements, waypoints) = spawn_settlements(10, (0.0, 100.0), (0.0, 100.0));

    let mut network = RoadNetwork {
        waypoints: waypoints.into_iter().map(|w| (w.id, w)).collect(),
        settlements: settlements.into_iter().map(|s| (s.id, s)).collect(),
        roads: HashMap::new(),
    };

    let roads = build_smart_roads(
        &network.settlements,
        &network.waypoints,
        0.3,    // top 30% = hubs
        20.0,   // connect small towns within 20 units
    );

    for road in &roads {
        println!("{:?}", road);
    }

    network.roads = roads.into_iter().map(|r| (r.id, r)).collect();

}
