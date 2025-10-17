use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use avian2d::prelude::*;
use crate::ball::Ball;

pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_launcher)
            .add_systems(Update, launcher_movement);
    }
}

#[derive(Component)]
struct Launcher {
    start_point: Vec2,
}

fn spawn_launcher(mut commands: Commands) {
    //Spawn launcher
    let shape_launcher = shapes::Rectangle {
        extents: Vec2::new(
            crate::PIXELS_PER_METER * 0.05,
            crate::PIXELS_PER_METER * 0.05,
        ),
        origin: shapes::RectangleOrigin::Center,
        radii: None
    };

    let launcher_pos = Vec2::new(
        crate::PIXELS_PER_METER * 0.3,
        crate::PIXELS_PER_METER * -0.58,
    );

    commands
        .spawn((
            Name::from("Launcher"),
            ShapeBuilder::with(&shape_launcher)
                .fill(Color::BLACK)
                .stroke((bevy::color::palettes::css::TEAL, 2.0))
                .build(),
            RigidBody::Kinematic,
            Collider::rectangle(
                shape_launcher.extents.x ,
                shape_launcher.extents.y,
            ), 
            //MassPropertiesBundle::from_shape(&launcher_shape, 2.0),
            Restitution::from(0.999),
            Transform::from_xyz(launcher_pos.x, launcher_pos.y, 0.0),
            Launcher {
                start_point: launcher_pos,
            }
        ));
}

fn launcher_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut launchers: Query<(&mut Launcher, &mut Transform), With<Launcher>>,
    mut ball: Query<&mut LinearVelocity, (With<Ball>, Without<Launcher>)>,
) {
    // TODO for now this does not seem to be working as expected as the ball is not launched

    // create a local variable that keeps track of how long the space key has been pressed
    // when released create a pulse on the ball

    // TODO the launcher should be a spring that compresses when space is held down


    for (launcher, mut launcher_transform) in launchers.iter_mut() {
        let mut next_ypos = launcher_transform.translation.y;

        if keyboard_input.pressed(KeyCode::Space) {
            next_ypos = next_ypos + crate::PIXELS_PER_METER * 0.04;
        } else {
            next_ypos = next_ypos - crate::PIXELS_PER_METER * 0.04;
        }
        let clamped_ypos = next_ypos.clamp(
            launcher.start_point.y,
            launcher.start_point.y + crate::PIXELS_PER_METER * 0.05,
        );
        launcher_transform.translation.y = clamped_ypos;
    }

    // hack for now, if space is released, increase the ball velocity upwards
    if keyboard_input.just_released(KeyCode::Space) {
        for mut ball_velocity in ball.iter_mut() {
            ball_velocity.y = 900.0;
        }
    }

}
