use bevy::prelude::*;
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};

use crate::core::components::{NPC, Position, Road};
use crate::core::resources::NpcMoveTimer;

/// Helper to convert Vec2 to a hashable key
fn vec2_to_key(pos: Vec2) -> (i32, i32) {
    (pos.x.round() as i32, pos.y.round() as i32)
}

/// Build adjacency graph from roads
fn build_road_graph(roads: &Query<&Road>) -> HashMap<(i32, i32), HashSet<(i32, i32)>> {
    let mut graph: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();

    for road in roads.iter() {
        graph.entry(vec2_to_key(road.waypoint_a)).or_default().insert(vec2_to_key(road.waypoint_b));
        graph.entry(vec2_to_key(road.waypoint_b)).or_default().insert(vec2_to_key(road.waypoint_a));
    }

    graph
}

/// System: Move NPCs randomly every few seconds
pub fn npc_random_movement_system(
    mut timer: ResMut<NpcMoveTimer>,
    time: Res<Time>,
    mut query: Query<(&mut NPC, &mut Position)>,
    roads: Query<&Road>,
) {
    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;  // Wait until timer triggers
    }

    let road_graph = build_road_graph(&roads);
    let mut rng = rand::thread_rng();

    for (mut npc, mut pos) in query.iter_mut() {
        let current = npc.current_settlement;

        let Some(neighbors) = road_graph.get(&current) else { continue };

        let mut options: Vec<_> = neighbors
            .iter()
            .copied()
            .filter(|&neighbor| Some(neighbor) != npc.last_settlement)
            .collect();

        if options.is_empty() {
            options = neighbors.iter().copied().collect();
        }

        if let Some(&next) = options.choose(&mut rng) {
            npc.last_settlement = Some(current);
            npc.current_settlement = next;
            pos.coords = Vec2::new(next.0 as f32, next.1 as f32);
            // info!("{} moved to {:?}", npc.name, next);
        }
    }
}