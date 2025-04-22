use bevy::prelude::Resource;

#[derive(Resource)]
pub struct SimulationConfig {
    pub x_range: (f32, f32),
    pub y_range: (f32, f32),
    pub settlement_count: usize
}