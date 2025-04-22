use std::collections::{HashMap, HashSet};
use crate::{Road, Settlement, Waypoint};

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

pub fn ensure_connected_network(
    settlements: &HashMap<usize, Settlement>,
    waypoints: &HashMap<usize, Waypoint>,
    existing_roads: &mut Vec<Road>,
) {
    let mut adjacency: HashMap<usize, HashSet<usize>> = HashMap::new();

    // Build adjacency list
    for road in existing_roads.iter() {
        if road.waypoints.len() == 2 {
            let a = road.waypoints[0];
            let b = road.waypoints[1];
            adjacency.entry(a).or_default().insert(b);
            adjacency.entry(b).or_default().insert(a);
        }
    }

    // Track visited settlements
    let mut visited = HashSet::new();
    let mut clusters = Vec::new();

    // DFS to find clusters
    for settlement in settlements.values() {
        if !visited.contains(&settlement.waypoint_id) {
            let mut cluster = HashSet::new();
            dfs(settlement.waypoint_id, &adjacency, &mut visited, &mut cluster);
            clusters.push(cluster);
        }
    }

    let mut road_id = existing_roads.iter().map(|r| r.id).max().unwrap_or(0) + 1;

    // Connect clusters until one remains
    while clusters.len() > 1 {
        let cluster_a = clusters.pop().unwrap();
        let cluster_b = clusters.pop().unwrap();

        // Find closest pair between clusters
        let mut min_dist = f32::MAX;
        let mut best_pair = (0, 0);

        for &a in &cluster_a {
            for &b in &cluster_b {
                let pos_a = waypoints.get(&a).unwrap().position;
                let pos_b = waypoints.get(&b).unwrap().position;
                let dist = distance(pos_a, pos_b);
                if dist < min_dist {
                    min_dist = dist;
                    best_pair = (a, b);
                }
            }
        }

        // Add road
        existing_roads.push(Road {
            id: road_id,
            name: format!("Connector {}-{}", best_pair.0, best_pair.1),
            waypoints: vec![best_pair.0, best_pair.1],
        });
        road_id += 1;

        // Merge clusters and continue
        let merged: HashSet<_> = cluster_a.union(&cluster_b).cloned().collect();
        clusters.push(merged);

        // Update adjacency list
        adjacency.entry(best_pair.0).or_default().insert(best_pair.1);
        adjacency.entry(best_pair.1).or_default().insert(best_pair.0);
    }
}

fn dfs(
    start: usize,
    adjacency: &HashMap<usize, HashSet<usize>>,
    visited: &mut HashSet<usize>,
    cluster: &mut HashSet<usize>,
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

fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}