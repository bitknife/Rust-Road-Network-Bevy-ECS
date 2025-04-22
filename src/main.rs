use bevy::prelude::*;
use road_network::{core::resources::SimulationConfig, generate_world, render_npcs_system, render_roads_system, render_settlements_system};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Road Network Simulator".into(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(SimulationConfig {
            x_range: (0.0, 800.0),
            y_range: (0.0, 600.0),
            settlement_count: 100,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (
            render_settlements_system,
            render_roads_system,
            render_npcs_system,
        ))
        .run();
}

fn setup(mut commands: Commands, window_q: Query<&Window>) {
    let window = window_q.single();

    let config = SimulationConfig {
        x_range: (0.0, window.width()),
        y_range: (0.0, window.height()),
        settlement_count: 100,
    };

    generate_world(&mut commands, &config);

    center_camera(&mut commands, window, config.x_range, config.y_range);
}

// Camera centering stays as utility
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
