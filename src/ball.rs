use super::BottomWall;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
            .add_systems(Update, handle_ball_intersections_with_bottom_wall);
    }
}

#[derive(Component)]
pub(crate) struct Ball;

fn spawn_ball(mut commands: Commands) {
    let ball_pos = Vec2::new(
        crate::PIXELS_PER_METER * 0.3,
        crate::PIXELS_PER_METER * -0.2,
    );

    let shape_ball = shapes::Circle {
        radius: crate::PIXELS_PER_METER * 0.03,
        center: Vec2::ZERO,
    };
    let bevy_shape = Circle::new(shape_ball.radius);

    commands.spawn((
        Name::from("Ball"),
        ShapeBuilder::with(&shape_ball)
            .fill(Color::BLACK)
            .stroke((bevy::color::palettes::css::TEAL, 2.0))
            .build(),
        Transform::from_xyz(ball_pos.x, ball_pos.y, 0.0),
        Collider::circle(shape_ball.radius),
        CollisionEventsEnabled,
        Restitution::from(0.7),
        // a standard pinball ball mass is about 80 grams
        //MassPropertiesBundle::from_shape(&bevy_shape, 10.0),
        RigidBody::Dynamic,
        Ball,
        SleepingDisabled,
    ));
    //.insert(Ccd::enabled()) ;
}

fn handle_ball_intersections_with_bottom_wall(
    mut collision_reader: MessageReader<CollisionStart>,
    query_ball: Query<&Ball>,
    query_bottom_wall: Query<&BottomWall>,
    mut commands: Commands,
) {
    let mut ball_entity = None;
    for event in collision_reader.read() {
        if let (Some(entity1), Some(entity2)) = (event.body1, event.body2) {
            if query_ball.contains(entity1) && query_bottom_wall.contains(entity2) {
                ball_entity = Some(entity1)
            } else if query_ball.contains(entity2) && query_bottom_wall.contains(entity1) {
                ball_entity = Some(entity2)
            };
        }
    }
    if let Some(ball) = ball_entity {
        commands.entity(ball).despawn();
        spawn_ball(commands);
    }
}
