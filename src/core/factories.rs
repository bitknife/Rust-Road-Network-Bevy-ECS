use rand::{Rng, seq::SliceRandom};
use crate::{Settlement, Waypoint, WaypointKind};

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
