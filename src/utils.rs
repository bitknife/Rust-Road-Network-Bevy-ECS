use std::collections::HashMap;
use rand::{Rng, seq::SliceRandom};
use crate::model::{Settlement, Waypoint, WaypointKind, Road};

pub fn spawn_settlements(
    count: usize,
    x_range: (f32, f32),
    y_range: (f32, f32),
) -> (Vec<Settlement>, Vec<Waypoint>) {
    let mut rng = rand::thread_rng();
    let mut settlements = Vec::new();
    let mut waypoints = Vec::new();

    let sample_names = vec![
        "Ashvale", "Brimstead", "Cairnhold", "Dunford", "Eldham",
        "Fairreach", "Glenbrook", "Highmere", "Ironhill", "Jorwick",
    ];

    for id in 1..=count {
        let name = sample_names
            .choose(&mut rng)
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("Settlement {}", id));

        let population = rng.gen_range(100..10_000);
        let x = rng.gen_range(x_range.0..x_range.1);
        let y = rng.gen_range(y_range.0..y_range.1);

        let waypoint_id = id;

        settlements.push(Settlement {
            id,
            name: name.clone(),
            population,
            waypoint_id,
        });

        waypoints.push(Waypoint {
            id: waypoint_id,
            position: (x, y),
            kind: WaypointKind::Settlement,
        });
    }

    (settlements, waypoints)
}

pub fn build_smart_roads(
    settlements: &HashMap<usize, Settlement>,
    waypoints: &HashMap<usize, Waypoint>,
    hub_percent: f32,          // e.g., 0.2 = top 20% as hubs
    mesh_distance_threshold: f32,
) -> Vec<Road> {
    let mut roads = Vec::new();

    // Step 1: Sort by population, find hubs
    let mut settlement_list: Vec<&Settlement> = settlements.values().collect();
    settlement_list.sort_by_key(|s| std::cmp::Reverse(s.population));

    let hub_count = ((hub_percent * settlement_list.len() as f32).ceil() as usize).max(1);
    let (hubs, non_hubs) = settlement_list.split_at(hub_count);

    let mut road_id = 1;

    for &settlement in non_hubs {
        let my_wp = waypoints.get(&settlement.waypoint_id).unwrap();
        let mut nearest = None;
        let mut nearest_dist = f32::MAX;

        for &hub in hubs {
            let hub_wp = waypoints.get(&hub.waypoint_id).unwrap();
            let dist = distance(my_wp.position, hub_wp.position);
            if dist < nearest_dist {
                nearest = Some(hub);
                nearest_dist = dist;
            }
        }

        if let Some(hub) = nearest {
            roads.push( Road{
                id: road_id,
                name: format!("{} to {}", settlement.name, hub.name),
                waypoints: vec![settlement.waypoint_id, hub.waypoint_id],
            });
            road_id += 1;
        }
    }

    for (i, &a) in non_hubs.iter().enumerate() {
        for &b in non_hubs.iter().skip(i + 1) {
            let a_wp = waypoints.get(&a.waypoint_id).unwrap();
            let b_wp = waypoints.get(&b.waypoint_id).unwrap();
            let dist = distance(a_wp.position, b_wp.position);
            if dist < mesh_distance_threshold {
                roads.push(Road {
                    id: road_id,
                    name: format!("{} <-> {}", &a.name, &b.name),
                    waypoints: vec![a.waypoint_id, b.waypoint_id],
                });
                road_id += 1;
            }
        }
    }

    roads
}

fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}