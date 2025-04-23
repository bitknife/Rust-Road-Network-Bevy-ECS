use bevy::prelude::*;
use road_network::{core::resources::SimulationConfig, generate_world, npc_random_movement_system, render_npcs_system, render_roads_system, render_settlements_system, sync_npc_visuals_system};
use road_network::core::NpcMoveTimer;
use bevy_tweening::TweeningPlugin;

use clap::Parser;

/// Road Network Simulator CLI
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct CliArgs {
    /// Number of settlements
    #[arg(long, default_value_t = 100)]
    settlements: u32,

    /// Number of NPCs
    #[arg(long, default_value_t = 50)]
    npcs: u32,

    /// NPC move interval in seconds
    #[arg(long, default_value_t = 2.0)]
    npc_move_interval: f32,

    /// Window width
    #[arg(long, default_value_t = 800.0)]
    width: f32,

    /// Window height
    #[arg(long, default_value_t = 600.0)]
    height: f32,
}

fn main() {
    let args = CliArgs::parse();

    let config = SimulationConfig {
        x_range: (0.0, args.width),
        y_range: (0.0, args.height),
        settlement_count: args.settlements,
        npc_count: args.npcs,
    };

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Road Network Simulator".into(),
                resolution: (args.width, args.height).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TweeningPlugin)
        .insert_resource(config)
        .insert_resource(NpcMoveTimer(Timer::from_seconds(args.npc_move_interval, TimerMode::Repeating)))
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
    config: Res<SimulationConfig>,
) {
    let window = window_q.single();

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
