use bevy::prelude::*;
use crate::core::components::*;
use crate::core::resources::SimulationConfig;
use crate::core::factories::{spawn_settlements, build_smart_roads, spawn_npcs, ensure_connected_network};

pub fn create_world_data(config: &SimulationConfig) -> (Vec<(Settlement, Position)>, Vec<Road>, Vec<(NPC, Position)>) {

    let settlements = spawn_settlements(config.settlement_count, config.x_range, config.y_range);
    let mut roads = build_smart_roads(&settlements, 0.05, 50.0);


    let npcs = spawn_npcs(&settlements, 20);

    (settlements, roads, npcs)
}

pub fn generate_world(commands: &mut Commands, config: &SimulationConfig) {
    let settlements = spawn_settlements(config.settlement_count, config.x_range, config.y_range);
    let mut roads = build_smart_roads(&settlements, 0.05, 50.0);
    let npcs = spawn_npcs(&settlements, 20);

    ensure_connected_network(&settlements, &mut roads);

    for (settlement, pos) in settlements {
        commands.spawn((settlement, pos));
    }

    for road in roads {
        commands.spawn(road);
    }

    for (npc, pos) in npcs {
        commands.spawn((npc, pos));
    }
}

