use crate::core::components::*;
use rand::{Rng, seq::SliceRandom};
use std::collections::{HashMap, HashSet};
use bevy::math::Vec2;


/// Spawns settlements with random positions and populations.
pub fn spawn_settlements(
    count: u32,
    x_range: (f32, f32),
    y_range: (f32, f32),
) -> Vec<(Settlement, Position)> {
    let mut rng = rand::thread_rng();
    let mut settlements = Vec::new();

    let sample_names = vec![
        "Ashvale", "Brimstead", "Cairnhold", "Dunford", "Eldham",
        "Fairreach", "Glenbrook", "Highmere", "Ironhill", "Jorwick",
    ];

    for id in 1..=count {
        let name = sample_names
            .choose(&mut rng)
            .map(|s| (*s).to_string()) // convert &str → String
            .unwrap_or_else(|| format!("Settlement {}", id));
        let population = rng.gen_range(100..10_000);

        let pos = Position {
            coords: Vec2::new(
                rng.gen_range(x_range.0..x_range.1),
                rng.gen_range(y_range.0..y_range.1),
            ),
        };

        settlements.push((
            Settlement { id, name, population },
            pos,
        ));
    }

    settlements
}

pub fn build_smart_roads(
    settlements: &[(Settlement, Position)],
    hub_percent: f32,
    mesh_distance_threshold: f32,
) -> Vec<Road> {
    let mut roads = Vec::new();

    let mut settlement_list: Vec<_> = settlements.iter().collect();
    settlement_list.sort_by_key(|(s, _)| std::cmp::Reverse(s.population));

    let hub_count = ((hub_percent * settlement_list.len() as f32).ceil() as usize).max(1);
    let (hubs, non_hubs) = settlement_list.split_at(hub_count);

    let mut road_id = 1;

    // Connect non-hubs to nearest hubs
    for &(_, pos) in non_hubs {
        let mut nearest = None;
        let mut nearest_dist = f32::MAX;

        for &(_, hub_pos) in hubs {
            let dist = pos.coords.distance(hub_pos.coords);
            if dist < nearest_dist {
                nearest = Some(hub_pos.coords);
                nearest_dist = dist;
            }
        }

        if let Some(hub_coords) = nearest {
            roads.push(Road {
                id: road_id,
                waypoint_a: pos.coords,
                waypoint_b: hub_coords,
            });
            road_id += 1;
        }
    }

    // Local mesh between nearby non-hubs
    for (i, &(_, a_pos)) in non_hubs.iter().enumerate() {
        for &(_, b_pos) in non_hubs.iter().skip(i + 1) {
            if a_pos.coords.distance(b_pos.coords) <= mesh_distance_threshold {
                roads.push(Road {
                    id: road_id,
                    waypoint_a: a_pos.coords,
                    waypoint_b: b_pos.coords,
                });
                road_id += 1;
            }
        }
    }

    roads
}

pub fn spawn_npcs(
    settlements: &[(Settlement, Position)],
    npc_count: u32,
) -> Vec<(NPC, Position)> {
    let mut rng = rand::thread_rng();
    let mut npcs = Vec::new();

    for i in 0..npc_count {
        let (_, home_pos) = settlements.choose(&mut rng).unwrap();

        let npc = NPC {
            name: format!("NPC_{}", i),
            current_settlement: (home_pos.coords.x.round() as i32, home_pos.coords.y.round() as i32),
            last_settlement: None,
            speed: rng.gen_range(0.5..2.0),
        };

        let pos = Position {
            coords: home_pos.coords + Vec2::new(rng.gen_range(-2.0..2.0), rng.gen_range(-2.0..2.0)),
        };

        npcs.push((npc, pos));
    }

    npcs
}

/// Ensure all settlements are connected by adding minimal connector roads.
pub fn ensure_connected_network(
    settlements: &[(Settlement, Position)],
    existing_roads: &mut Vec<Road>,
) {
    // Use (i32, i32) as hashable position keys
    let mut adjacency: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();

    for road in existing_roads.iter() {
        let a_key = vec2_to_key(road.waypoint_a);
        let b_key = vec2_to_key(road.waypoint_b);

        adjacency.entry(a_key).or_default().insert(b_key);
        adjacency.entry(b_key).or_default().insert(a_key);
    }

    let mut visited = HashSet::new();
    let mut clusters = Vec::new();

    for (_settlement, pos) in settlements {
        let key = vec2_to_key(pos.coords);
        if !visited.contains(&key) {
            let mut cluster = HashSet::new();
            dfs(key, &adjacency, &mut visited, &mut cluster);
            clusters.push(cluster);
        }
    }

    let mut road_id = existing_roads.iter().map(|r| r.id).max().unwrap_or(0) + 1;

    while clusters.len() > 1 {
        let cluster_a = clusters.pop().unwrap();
        let cluster_b = clusters.pop().unwrap();

        let mut min_dist = f32::MAX;
        let mut best_pair = ((0, 0), (0, 0));

        for &a in &cluster_a {
            for &b in &cluster_b {
                let a_vec = Vec2::new(a.0 as f32, a.1 as f32);
                let b_vec = Vec2::new(b.0 as f32, b.1 as f32);
                let dist = a_vec.distance(b_vec);
                if dist < min_dist {
                    min_dist = dist;
                    best_pair = (a, b);
                }
            }
        }

        existing_roads.push(Road {
            id: road_id,
            waypoint_a: Vec2::new(best_pair.0.0 as f32, best_pair.0.1 as f32),
            waypoint_b: Vec2::new(best_pair.1.0 as f32, best_pair.1.1 as f32),
        });
        road_id += 1;

        let merged: HashSet<_> = cluster_a.union(&cluster_b).cloned().collect();
        clusters.push(merged);

        adjacency.entry(best_pair.0).or_default().insert(best_pair.1);
        adjacency.entry(best_pair.1).or_default().insert(best_pair.0);
    }
}

fn dfs(
    start: (i32, i32),
    adjacency: &HashMap<(i32, i32), HashSet<(i32, i32)>>,
    visited: &mut HashSet<(i32, i32)>,
    cluster: &mut HashSet<(i32, i32)>,
) {
    let mut stack = vec![start];

    while let Some(node) = stack.pop() {
        if visited.insert(node) {
            cluster.insert(node);
            if let Some(neighbors) = adjacency.get(&node) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }
    }
}

fn vec2_to_key(pos: Vec2) -> (i32, i32) {
    (pos.x.round() as i32, pos.y.round() as i32)
}
