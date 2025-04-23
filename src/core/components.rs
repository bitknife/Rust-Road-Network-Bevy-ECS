use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Settlement {
    pub id: u32,
    pub name: String,
    pub population: u32,
}

#[derive(Component, Debug, Clone)]
pub struct Road {
    pub id: u32,
    pub waypoint_a: Vec2,
    pub waypoint_b: Vec2,
}

#[derive(Component, Debug, Clone)]
pub struct NPC {
    pub name: String,
    pub current_settlement: (i32, i32),   // Using key format
    pub last_settlement: Option<(i32, i32)>,
    pub speed: f32,                          // Units per tick
}

#[derive(Component, Debug, Clone)]
pub struct Position {
    pub coords: Vec2,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct MovingTo(pub Vec2);