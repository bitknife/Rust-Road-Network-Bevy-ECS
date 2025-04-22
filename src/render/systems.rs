use bevy::prelude::*;
use crate::core::components::{Settlement, Position, Road};

#[derive(Component)]
pub struct Rendered;  // Marker for already-rendered entities

pub fn render_settlements_system(
    mut commands: Commands,
    query: Query<(Entity, &Settlement, &Position), Without<Rendered>>,
) {
    for (entity, settlement, pos) in query.iter() {
        let color = if settlement.population > 9000 {
            Color::RED
        } else {
            Color::ORANGE
        };

        let radius = map_population_to_radius(settlement.population);

        // Spawn the visual as a separate entity
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(radius * 2.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(pos.coords.x, pos.coords.y, 1.0)),
                ..default()
            },
            SettlementVisual { parent: entity },  // Optional: track which settlement this belongs to
        ));

        // Mark the logical entity as rendered
        commands.entity(entity).insert(Rendered);
    }
}

#[derive(Component)]
pub struct SettlementVisual {
    pub parent: Entity,  // Link back to the logical Settlement entity
}

fn map_population_to_radius(population: u32) -> f32 {
    let min_pop = 100;
    let max_pop = 10_000;
    let min_radius = 2.0;
    let max_radius = 10.0;

    let clamped_pop = population.clamp(min_pop, max_pop);

    let scale = (clamped_pop - min_pop) as f32 / (max_pop - min_pop) as f32;
    min_radius + scale * (max_radius - min_radius)
}

#[derive(Component)]
pub struct RoadVisual {
    pub parent: Entity,  // Link to logical Road entity
}

pub fn render_roads_system(
    mut commands: Commands,
    query: Query<(Entity, &Road), Without<Rendered>>,
) {
    for (entity, road) in query.iter() {
        let a = road.waypoint_a;
        let b = road.waypoint_b;

        let dx = b.x - a.x;
        let dy = b.y - a.y;
        let length = (dx * dx + dy * dy).sqrt();
        let angle = dy.atan2(dx).to_degrees();

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    custom_size: Some(Vec2::new(length, 1.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0, 0.0),
                    rotation: Quat::from_rotation_z(angle.to_radians()),
                    ..default()
                },
                ..default()
            },
            RoadVisual { parent: entity },
        ));

        commands.entity(entity).insert(Rendered);
    }
}
