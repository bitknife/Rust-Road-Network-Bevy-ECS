use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Settlement {
    pub id: usize,
    pub name: String,
    pub population: u32,
}

#[derive(Component, Debug, Clone)]
pub struct Road {
    pub id: usize,
    pub waypoint_a: Vec2,
    pub waypoint_b: Vec2,
}

#[derive(Component, Debug, Clone)]
pub struct NPC {
    pub name: String,
    pub target_settlement: Option<Entity>,  // Dynamic target
    pub speed: f32,                         // Units per tick
}

#[derive(Component, Debug, Clone)]
pub struct Position {
    pub coords: Vec2,
}
