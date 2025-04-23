use bevy::prelude::{Resource, Timer};

#[derive(Resource)]
pub struct SimulationConfig {
    pub x_range: (f32, f32),
    pub y_range: (f32, f32),
    pub settlement_count: u32,
    pub npc_count: u32
}

#[derive(Resource)]
pub struct NpcMoveTimer(pub Timer);

