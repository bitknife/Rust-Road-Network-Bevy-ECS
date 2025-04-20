use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Settlement {
    pub id: usize,
    pub name: String,
    pub population: u32,
    pub waypoint_id: usize,
}

#[derive(Debug, Clone)]
pub enum WaypointKind {
    Normal,
    Junction,
    Settlement,
}

#[derive(Debug, Clone)]
pub struct Waypoint {
    pub id: usize,
    pub position: (f32, f32),
    pub kind: WaypointKind,
}

#[derive(Debug, Clone)]
pub struct Road {
    pub id: usize,
    pub name: String,
    pub waypoints: Vec<usize>, // usually two: start and end
}

#[derive(Debug)]
pub struct RoadNetwork {
    pub waypoints: HashMap<usize, Waypoint>,
    pub roads: HashMap<usize, Road>,
    pub settlements: HashMap<usize, Settlement>,
}
