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
    point_of_rotation: Vec3,
    curr_angle: f32,
}

#[derive(Component)]
struct RightFlipper {
    point_of_rotation: Vec3,
    curr_angle: f32,
}

fn spawn_flippers(mut commands: Commands) {
    //Spawn flippers
    let shape_flipper: shapes::Rectangle = shapes::Rectangle {
        extents: Vec2::new(
            crate::PIXELS_PER_METER * 0.25,
            crate::PIXELS_PER_METER * 0.05,
        ),
        origin: shapes::RectangleOrigin::Center,
        radii: None,
    }
    .into();

    //Spawn left flipper
    let left_flipper_pos = Vec2::new(
        crate::PIXELS_PER_METER * -0.2,
        crate::PIXELS_PER_METER * -0.4,
    );

    commands.spawn((
        Name::from("Flipper Left"),
        ShapeBuilder::with(&shape_flipper)
            .fill(Color::BLACK)
            .stroke((bevy::color::palettes::css::TEAL, 2.0))
            .build(),
        RigidBody::Kinematic,
        Collider::rectangle(shape_flipper.extents.x, shape_flipper.extents.y),
        Restitution::from(0.99).with_combine_rule(CoefficientCombine::Max),
        Transform::from_xyz(left_flipper_pos.x, left_flipper_pos.y, 0.0),
        LeftFlipper {
            point_of_rotation: Vec3::new(
                left_flipper_pos.x - (shape_flipper.extents.x / 2.0),
                left_flipper_pos.y + (shape_flipper.extents.y) / 2.0,
                0.0,
            ),
            curr_angle: 0.0,
        },
    ));

    //Spawn right flipper
    let right_flipper_pos = Vec2::new(
        crate::PIXELS_PER_METER * 0.1,
        crate::PIXELS_PER_METER * -0.4,
    );

    commands.spawn((
        Name::from("Flipper Right"),
        ShapeBuilder::with(&shape_flipper)
            .fill(Color::BLACK)
            .stroke((bevy::color::palettes::css::TEAL, 2.0))
            .build(),
        RigidBody::Kinematic,
        Collider::rectangle(shape_flipper.extents.x, shape_flipper.extents.y),
        Restitution::from(0.999).with_combine_rule(CoefficientCombine::Max),
        Transform::from_xyz(right_flipper_pos.x, right_flipper_pos.y, 0.0),
        RightFlipper {
            point_of_rotation: Vec3::new(
                right_flipper_pos.x + (shape_flipper.extents.x / 2.0),
                right_flipper_pos.y + (shape_flipper.extents.y) / 2.0,
                0.0,
            ),
            curr_angle: 0.0,
        },
    ));
}

fn left_flipper_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut left_flippers: Query<(&mut LeftFlipper, &mut Transform), With<LeftFlipper>>,
) {
    for (mut left_flipper, mut left_flipper_transform) in left_flippers.iter_mut() {
        let mut new_angle = left_flipper.curr_angle;
        let change_angle: f32;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::ShiftLeft)
        {
            change_angle = 0.09;
        } else {
            change_angle = -0.07;
        }

        new_angle += change_angle;
        let new_clamped_angle = new_angle.clamp(-0.3, 0.3);
        let pivot_rotation = Quat::from_rotation_z(new_clamped_angle - left_flipper.curr_angle);
        left_flipper_transform.rotate_around(left_flipper.point_of_rotation, pivot_rotation);
        left_flipper.curr_angle = new_clamped_angle;
    }
}

fn right_flipper_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut right_flippers: Query<(&mut RightFlipper, &mut Transform), With<RightFlipper>>,
) {
    for (mut right_flipper, mut right_flipper_transform) in right_flippers.iter_mut() {
        let mut new_angle = right_flipper.curr_angle;
        let change_angle: f32;
        if keyboard_input.pressed(KeyCode::ArrowRight)
            || keyboard_input.pressed(KeyCode::ShiftRight)
        {
            change_angle = -0.09;
        } else {
            change_angle = 0.07;
        }

        new_angle += change_angle;
        let new_clamped_angle = new_angle.clamp(-0.3, 0.3);
        let pivot_rotation = Quat::from_rotation_z(new_clamped_angle - right_flipper.curr_angle);
        right_flipper_transform.rotate_around(right_flipper.point_of_rotation, pivot_rotation);
        right_flipper.curr_angle = new_clamped_angle;
    }
}
