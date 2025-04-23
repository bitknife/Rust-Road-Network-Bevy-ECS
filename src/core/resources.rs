use bevy::prelude::{Resource, Timer};

#[derive(Resource)]
pub struct SimulationConfig {
    pub x_range: (f32, f32),
    pub y_range: (f32, f32),
    pub settlement_count: usize,
    pub npc_count: usize
}

#[derive(Resource)]
pub struct NpcMoveTimer(pub Timer);

