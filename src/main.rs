use bevy::prelude::*;
use road_network::{core::resources::SimulationConfig, generate_world, npc_random_movement_system, render_npcs_system, render_roads_system, render_settlements_system, sync_npc_visuals_system};
use road_network::core::NpcMoveTimer;
use bevy_tweening::TweeningPlugin;

fn main() {
    let config = SimulationConfig {
        x_range: (0.0, 800.0),
        y_range: (0.0, 600.0),
        settlement_count: 100,
        npc_count: 50
    };

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Road Network Simulator".into(),
                resolution: (
                    config.x_range.1 - config.x_range.0,
                    config.y_range.1 - config.y_range.0
                ).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TweeningPlugin)
        .insert_resource(config)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            render_settlements_system,
            render_roads_system,
            render_npcs_system,
            npc_random_movement_system,
            sync_npc_visuals_system,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    window_q: Query<&Window>,
    config: Res<SimulationConfig>,   // 💡 Access the inserted resource
) {
    commands.insert_resource(NpcMoveTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));

    let window = window_q.single();

    // No need to redefine config — use the resource directly
    generate_world(&mut commands, &config);

    // If you want to adjust camera dynamically based on window/config:
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
