use crate::ball::Ball;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const BUMPER_PULSE_FORCE: f32 = 35.0;

pub struct BumpersPlugin;

impl Plugin for BumpersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bumpers)
            .add_systems(Update, handle_bumper_collisions)
            .add_systems(Update, hide_expired_bumper_indicators);
    }
}

#[derive(Component)]
struct Bumper;

#[derive(Component)]
struct BumperIndicator {
    timestamp_last_hit: f64,
}

fn spawn_bumpers(mut commands: Commands) {
    let bumper_positions: [Vec2; 3] = [
        Vec2::new(
            crate::PIXELS_PER_METER * -0.17,
            crate::PIXELS_PER_METER * 0.35,
        ),
        Vec2::new(
            crate::PIXELS_PER_METER * 0.17,
            crate::PIXELS_PER_METER * 0.35,
        ),
        Vec2::new(0.0, crate::PIXELS_PER_METER * 0.2),
    ];

    for (i, position) in bumper_positions.iter().enumerate() {
        spawn_single_bumper(&mut commands, format!("Bumper{}", i), *position);
    }
}

fn spawn_single_bumper(commands: &mut Commands, name: String, position: Vec2) {
    let outer_radius = crate::PIXELS_PER_METER * 0.05;
    let inner_radius = crate::PIXELS_PER_METER * 0.04;

    let shape_bumper = shapes::Circle {
        radius: outer_radius,
        center: Vec2::ZERO,
    };

    let shape_indicator = shapes::Circle {
        radius: inner_radius,
        center: Vec2::ZERO,
    };

    // Create the main bumper
    commands.spawn((
        Name::from(name),
        ShapeBuilder::with(&shape_bumper)
            .fill(Color::BLACK)
            .stroke((bevy::color::palettes::css::TEAL, 2.0))
            .build(),
        Transform::from_xyz(position.x, position.y, 0.0),
        RigidBody::Static,
        Restitution::new(0.7),
        Collider::circle(outer_radius),
        Bumper,
        children![(
            Name::from("BumperIndicator"),
            ShapeBuilder::with(&shape_indicator)
                .fill(bevy::color::palettes::css::GREEN)
                .build(),
            Visibility::Hidden,
            Transform::from_xyz(0.0, 0.0, 0.1), // Slightly above bumper
            BumperIndicator {
                timestamp_last_hit: 0.0,
            },
        )],
    ));
}

fn handle_bumper_collisions(
    bumper_query: Query<(&Children, Entity, &Transform), With<Bumper>>,
    mut indicator_query: Query<(Entity, &mut BumperIndicator), With<BumperIndicator>>,
    mut ball_query: Query<(&Transform, Forces), With<Ball>>,
    time: Res<Time>,
    mut contact_events: MessageReader<CollisionStart>,
    mut commands: Commands,
) {
    for contact_event in contact_events.read() {
        for (children, bumper_entity, bumper_transform) in bumper_query.iter() {
            if let (Some(h1), Some(h2)) = (contact_event.body1, contact_event.body2)
                && (h1 == bumper_entity || h2 == bumper_entity)
            {
                // Activate the indicator
                for child in children.iter() {
                    if let Ok((entity, mut indicator)) = indicator_query.get_mut(child) {
                        indicator.timestamp_last_hit = time.elapsed_secs_f64();
                        commands.entity(entity).insert(Visibility::Visible);
                    }
                }

                // Apply outward pulse to the ball
                let ball_entity = if h1 == bumper_entity { h2 } else { h1 };
                if let Ok((ball_transform, mut forces)) = ball_query.get_mut(ball_entity) {
                    // Calculate direction from bumper center to ball
                    let bumper_pos = bumper_transform.translation.truncate();
                    let ball_pos = ball_transform.translation.truncate();
                    let direction = (ball_pos - bumper_pos).normalize();

                    forces.apply_linear_impulse(direction * BUMPER_PULSE_FORCE);
                }
            }
        }
    }
}

const BUMPER_LIGHT_TIME: f64 = 0.2;

fn hide_expired_bumper_indicators(
    mut query: Query<(Entity, &BumperIndicator), With<BumperIndicator>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, indicator) in query.iter_mut() {
        if indicator.timestamp_last_hit > 0.0 {
            let elapsed = time.elapsed_secs_f64() - indicator.timestamp_last_hit;
            if elapsed > BUMPER_LIGHT_TIME {
                commands.entity(entity).insert(Visibility::Hidden);
            }
        }
    }
}
