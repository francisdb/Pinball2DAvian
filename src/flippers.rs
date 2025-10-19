use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const FLIPPER_ENABLED_TORQUE: f32 = 1500000.0;
const FLIPPER_DISABLED_TORQUE: f32 = -500000.0;

// Typical pinball flipper extents involve a maximum upward swing of about 20 degrees for each flipper,
// and a swing of 55-60 degrees from their resting position.
const FLIPPER_MAX_UP_ANGLE: f32 = 20.0_f32.to_radians();
const FLIPPER_MAX_DOWN_ANGLE: f32 = 35.0_f32.to_radians();

pub struct FlippersPlugin;

impl Plugin for FlippersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_flippers)
            .add_systems(Update, left_flipper_movement)
            .add_systems(Update, right_flipper_movement);
    }
}

#[derive(Component)]
struct LeftFlipper;

#[derive(Component)]
struct RightFlipper;

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
        -shape_flipper.extents.x / 2.0 + shape_flipper.extents.y / 2.0,
        0.0,
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
                0.1,
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
            RigidBody::Dynamic,
            Collider::rectangle(shape_flipper.extents.x, shape_flipper.extents.y),
            //SleepingDisabled,
            Mass::from(1.0),
            // flippers have rubbers that make them bouncy
            Restitution::from(0.4),
            Transform::from_xyz(left_flipper_pos.x, left_flipper_pos.y, 0.0),
            LeftFlipper,
        ))
        .id();

    commands.spawn((
        Name::from("Left Flipper Joint"),
        RevoluteJoint::new(left_anchor, left_fliper)
            .with_local_anchor1(Vec2::ZERO)
            .with_local_anchor2(left_pivot)
            .with_angle_limits(-FLIPPER_MAX_DOWN_ANGLE, FLIPPER_MAX_UP_ANGLE), // to avoid jittering we add a small margin
    ));

    //Spawn right flipper
    let right_flipper_pos = Vec2::new(
        crate::PIXELS_PER_METER * 0.1,
        crate::PIXELS_PER_METER * -0.4,
    );
    let right_pivot = Vec2::new(
        shape_flipper.extents.x / 2.0 - shape_flipper.extents.y / 2.0,
        0.0,
    );

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
                0.1,
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
            RigidBody::Dynamic,
            Collider::rectangle(shape_flipper.extents.x, shape_flipper.extents.y),
            //SleepingDisabled,
            Mass::from(1.0),
            // flippers have rubbers that make them bouncy
            Restitution::from(0.5),
            Transform::from_xyz(right_flipper_pos.x, right_flipper_pos.y, 0.0),
            RightFlipper,
        ))
        .id();

    // TODO test this:
    //   You can override the center of mass and set rotation speed directly on a kinematic body

    commands.spawn((
        Name::from("Right Flipper Joint"),
        RevoluteJoint::new(right_anchor, right_flipper)
            .with_local_anchor1(Vec2::ZERO)
            .with_local_anchor2(right_pivot)
            // to avoid jittering we add a small margin
            .with_angle_limits(-FLIPPER_MAX_UP_ANGLE, FLIPPER_MAX_DOWN_ANGLE),
        // JointDamping {
        //     angular: 0.5,
        //     ..default()
        // },
    ));
}

fn left_flipper_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut flippers: Query<Entity, With<LeftFlipper>>,
    mut commands: Commands,
) {
    for flipper in flippers.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::ShiftLeft)
        {
            commands
                .entity(flipper)
                .insert(ConstantTorque(FLIPPER_ENABLED_TORQUE));
        } else {
            // since gravity is not pulling enough we force a torque in the opposite direction
            //commands.entity(flipper).remove::<ConstantTorque>();
            commands
                .entity(flipper)
                .insert(ConstantTorque(FLIPPER_DISABLED_TORQUE));
        }
    }
}

fn right_flipper_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut flippers: Query<Entity, With<RightFlipper>>,
    mut commands: Commands,
) {
    for flipper in flippers.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowRight)
            || keyboard_input.pressed(KeyCode::ShiftRight)
        {
            commands
                .entity(flipper)
                .insert(ConstantTorque(-FLIPPER_ENABLED_TORQUE));
        } else {
            // since gravity is not pulling enough we force a torque in the opposite direction
            //commands.entity(flipper).remove::<ConstantTorque>();
            commands
                .entity(flipper)
                .insert(ConstantTorque(-FLIPPER_DISABLED_TORQUE));
        }
    }
}
