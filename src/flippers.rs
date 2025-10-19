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
            RigidBody::Dynamic,
            Collider::rectangle(shape_flipper.extents.x, shape_flipper.extents.y),
            SleepingDisabled,
            // flippers have rubbers that make them bouncy
            Restitution::from(0.5),
            Transform::from_xyz(left_flipper_pos.x, left_flipper_pos.y, 0.0),
            LeftFlipper,
        ))
        .id();

    commands.spawn((
        Name::from("Left Flipper Joint"),
        RevoluteJoint::new(left_anchor, left_fliper)
            .with_local_anchor1(Vec2::ZERO)
            .with_local_anchor2(left_pivot)
            .with_angle_limits(-0.3, 0.3), // to avoid jittering we add a small margin
    ));

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
            RigidBody::Dynamic,
            Collider::rectangle(shape_flipper.extents.x, shape_flipper.extents.y),
            SleepingDisabled,
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
            .with_angle_limits(-0.3, 0.3),
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
                .insert(ConstantTorque(5000000000.0));
        } else {
            // since gravity is not pulling enough we force a torque in the opposite direction
            //commands.entity(flipper).remove::<ConstantTorque>();
            commands
                .entity(flipper)
                .insert(ConstantTorque(-1000000000.0));
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
                .insert(ConstantTorque(-5000000000.0));
        } else {
            // since gravity is not pulling enough we force a torque in the opposite direction
            //commands.entity(flipper).remove::<ConstantTorque>();
            commands
                .entity(flipper)
                .insert(ConstantTorque(1000000000.0));
        }
    }
}
