use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub struct FlippersPlugin;

impl Plugin for FlippersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_flippers)
            .add_systems(Update, left_flipper_movement)
            .add_systems(Update, right_flipper_movement);
    }
}

#[derive(Component)]
struct LeftFlipper {
    max_angle: f32,
    min_angle: f32,
}

#[derive(Component)]
struct RightFlipper {
    max_angle: f32,
    min_angle: f32,
}

fn spawn_flippers(mut commands: Commands) {
    //Spawn flippers
    let shape_flipper = shapes::Rectangle {
        extents: Vec2::new(
            crate::PIXELS_PER_METER * 0.25,
            crate::PIXELS_PER_METER * 0.05,
        ),
        origin: shapes::RectangleOrigin::Center,
        radii: None,
    };

    //Spawn left flipper
    let left_flipper_pos = Vec2::new(
        crate::PIXELS_PER_METER * -0.2,
        crate::PIXELS_PER_METER * -0.4,
    );
    let left_pivot = Vec2::new(
        -shape_flipper.extents.x / 2.0,
        shape_flipper.extents.y / 2.0,
    );

    let left_anchor = commands
        .spawn((
            Name::from("Left Flipper Anchor"),
            // ShapeBuilder::with(&shapes::Circle {
            //     radius: 5.0,
            //     center: Vec2::ZERO,
            // })
            // .fill(bevy::color::palettes::css::YELLOW)
            // .build(),
            RigidBody::Static,
            Transform::from_xyz(
                left_flipper_pos.x + left_pivot.x,
                left_flipper_pos.y + left_pivot.y,
                0.0,
            ),
        ))
        .id();

    let left_fliper = commands
        .spawn((
            Name::from("Flipper Left"),
            ShapeBuilder::with(&shape_flipper)
                .fill(Color::BLACK)
                .stroke((bevy::color::palettes::css::TEAL, 2.0))
                .build(),
            RigidBody::Kinematic,
            Collider::rectangle(shape_flipper.extents.x, shape_flipper.extents.y),
            Transform::from_xyz(left_flipper_pos.x, left_flipper_pos.y, 0.0),
            LeftFlipper {
                max_angle: 0.3,
                min_angle: -0.3,
            },
        ))
        .id();

    commands.spawn(
        RevoluteJoint::new(left_anchor, left_fliper)
            .with_local_anchor1(Vec2::ZERO)
            .with_local_anchor2(left_pivot)
            .with_angle_limits(-0.3 - 0.05, 0.3 + 0.05), // to avoid jittering we add a small margin
    );

    //Spawn right flipper
    let right_flipper_pos = Vec2::new(
        crate::PIXELS_PER_METER * 0.1,
        crate::PIXELS_PER_METER * -0.4,
    );
    let right_pivot = Vec2::new(shape_flipper.extents.x / 2.0, shape_flipper.extents.y / 2.0);

    let right_anchor = commands
        .spawn((
            Name::from("Right Flipper Anchor"),
            // ShapeBuilder::with(&shapes::Circle {
            //     radius: 5.0,
            //     center: Vec2::ZERO,
            // })
            // .fill(bevy::color::palettes::css::YELLOW)
            // .build(),
            RigidBody::Static,
            Transform::from_xyz(
                right_flipper_pos.x + right_pivot.x,
                right_flipper_pos.y + right_pivot.y,
                0.0,
            ),
        ))
        .id();

    let right_flipper = commands
        .spawn((
            Name::from("Flipper Right"),
            ShapeBuilder::with(&shape_flipper)
                .fill(Color::BLACK)
                .stroke((bevy::color::palettes::css::TEAL, 2.0))
                .build(),
            RigidBody::Kinematic,
            Collider::rectangle(shape_flipper.extents.x, shape_flipper.extents.y),
            //Restitution::from(0.999).with_combine_rule(CoefficientCombine::Max),
            Transform::from_xyz(right_flipper_pos.x, right_flipper_pos.y, 0.0),
            RightFlipper {
                max_angle: 0.3,
                min_angle: -0.3,
            },
        ))
        .id();

    // TODO test this:
    //   You can override the center of mass and set rotation speed directly on a kinematic body

    commands.spawn(
        RevoluteJoint::new(right_anchor, right_flipper)
            .with_local_anchor1(Vec2::ZERO)
            .with_local_anchor2(right_pivot)
            .with_angle_limits(-0.3 - 0.05, 0.3 + 0.05), // to avoid jittering we add a small margin
    );
}

fn left_flipper_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut flippers: Query<(&LeftFlipper, &Transform, &mut AngularVelocity), With<LeftFlipper>>,
) {
    for (flipper, transform, mut angular_velocity) in flippers.iter_mut() {
        let current_angle = transform.rotation.to_euler(EulerRot::XYZ).2;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::ShiftLeft)
        {
            if current_angle < flipper.max_angle {
                angular_velocity.0 = 15.0;
            } else {
                angular_velocity.0 = 0.0;
            }
        } else {
            if current_angle > flipper.min_angle {
                angular_velocity.0 = -10.0;
            } else {
                angular_velocity.0 = 0.0;
            }
        }
    }
}

fn right_flipper_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut flippers: Query<(&RightFlipper, &Transform, &mut AngularVelocity), With<RightFlipper>>,
) {
    for (flipper, transform, mut angular_velocity) in flippers.iter_mut() {
        let current_angle = transform.rotation.to_euler(EulerRot::XYZ).2;

        if keyboard_input.pressed(KeyCode::ArrowRight)
            || keyboard_input.pressed(KeyCode::ShiftRight)
        {
            // TODO why do we have jitter here when reaching the max angle?
            if current_angle > flipper.min_angle {
                angular_velocity.0 = -15.0;
            } else {
                angular_velocity.0 = 0.0;
            }
        } else {
            if current_angle < flipper.max_angle {
                angular_velocity.0 = 10.0;
            } else {
                angular_velocity.0 = 0.0;
            }
        }
    }
}
