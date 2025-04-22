use bevy::prelude::*;
use road_network::{build_smart_roads, ensure_connected_network, Road, Waypoint};
use road_network::core::factories::spawn_settlements;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Road Network".into(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.single();

    let x_range = (0.0, 800.0);
    let y_range = (0.0, 600.0);

    center_camera(&mut commands, window, x_range, y_range);

    let (settlements, waypoints) = spawn_settlements(100, x_range, y_range);

    let mut roads = build_smart_roads(
        &settlements.iter().map(|s| (s.id, s.clone())).collect(),
        &waypoints.iter().map(|w| (w.id, w.clone())).collect(),
        0.05,
        50.0,
    );

    // Ensure connectivity
    ensure_connected_network(
        &settlements.iter().map(|s| (s.id, s.clone())).collect(),
        &waypoints.iter().map(|w| (w.id, w.clone())).collect(),
        &mut roads,
    );

    for road in &roads {
        draw_road(&mut commands, road, &waypoints);
    }

    for wp in &waypoints {
        if let Some(settlement) = settlements.iter().find(|s| s.waypoint_id == wp.id) {
            let color = if settlement.population > 9000 {
                Color::RED
            } else {
                Color::ORANGE
            };

            // Calculate radius based on population
            let radius = map_population_to_radius(settlement.population);

            draw_circle(&mut commands, wp.position, color, radius);
        } else {
            // Default for non-settlement waypoints
            draw_circle(&mut commands, wp.position, Color::WHITE, 2.0);
        }
    }
}

fn map_population_to_radius(population: u32) -> f32 {
    let min_pop = 100;     // Minimum expected population
    let max_pop = 10_000;  // Maximum expected population

    let min_radius = 2.0;
    let max_radius = 10.0;

    let clamped_pop = population.clamp(min_pop, max_pop);

    let scale = (clamped_pop - min_pop) as f32 / (max_pop - min_pop) as f32;

    min_radius + scale * (max_radius - min_radius)
}

fn center_camera(commands: &mut Commands, window: &Window, x_range: (f32, f32), y_range: (f32, f32)) {
    let center_x = (x_range.0 + x_range.1) / 2.0;
    let center_y = (y_range.0 + y_range.1) / 2.0;

    let area_width = x_range.1 - x_range.0;
    let area_height = y_range.1 - y_range.0;

    let scale_x = area_width / window.width();
    let scale_y = area_height / window.height();

    let scale = scale_x.max(scale_y);

    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(center_x, center_y, 1000.0),
            scale: Vec3::splat(scale),
            ..default()
        },
        ..default()
    });
}

fn draw_road(commands: &mut Commands, road: &Road, waypoints: &[Waypoint]) {
    if road.waypoints.len() != 2 {
        return;
    }

    let a = waypoints.iter().find(|w| w.id == road.waypoints[0]).unwrap().position;
    let b = waypoints.iter().find(|w| w.id == road.waypoints[1]).unwrap().position;

    draw_line(commands, a, b, Color::GRAY);
}

fn draw_line(commands: &mut Commands, a: (f32, f32), b: (f32, f32), color: Color) {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    let length = (dx * dx + dy * dy).sqrt();
    let angle = dy.atan2(dx).to_degrees();

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::new(length, 1.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0, 0.0),
            rotation: Quat::from_rotation_z(angle.to_radians()),
            ..default()
        },
        ..default()
    });
}

fn draw_circle(commands: &mut Commands, pos: (f32, f32), color: Color, radius: f32) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::splat(radius * 2.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(pos.0, pos.1, 1.0)),
        ..default()
    });
}
