use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

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
            crate::PIXELS_PER_METER * 0.02,
            crate::PIXELS_PER_METER * 0.05,
        ),
        origin: shapes::RectangleOrigin::Center,
        radii: None,
    };

    let launcher_pos = Vec2::new(
        crate::PIXELS_PER_METER * 0.3,
        crate::PIXELS_PER_METER * -0.50,
    );

    // Create a fixed anchor for the spring
    let anchor = commands
        .spawn((
            Name::from("Launcher Anchor"),
            // ShapeBuilder::with(&shapes::Circle {
            //     radius: 5.0,
            //     center: Vec2::ZERO,
            // })
            // .fill(bevy::color::palettes::css::YELLOW)
            // .build(),
            RigidBody::Static,
            // z=1.0 to draw above launcher
            Transform::from_xyz(launcher_pos.x, launcher_pos.y, 1.0),
        ))
        .id();

    // Spawn the launcher with spring joint
    let launcher = commands
        .spawn((
            Name::from("Launcher"),
            ShapeBuilder::with(&shape_launcher)
                .fill(Color::BLACK)
                .stroke((bevy::color::palettes::css::TEAL, 2.0))
                .build(),
            RigidBody::Dynamic,
            Collider::rectangle(shape_launcher.extents.x, shape_launcher.extents.y),
            Transform::from_xyz(launcher_pos.x, launcher_pos.y, 0.0),
            ConstantForce::new(0.0, 0.0),
            LockedAxes::ROTATION_LOCKED,
            Mass::from(0.2), // Light mass for responsive spring
            Launcher {
                start_point: launcher_pos,
            },
        ))
        .id();

    // Add prismatic joint (vertical slider) with spring properties
    commands.spawn((
        DistanceJoint::new(anchor, launcher)
            .with_local_anchor1(Vec2::ZERO)
            .with_local_anchor2(Vec2::ZERO)
            .with_compliance(0.002),
        // avoid bouncing
        JointDamping {
            linear: 20.0,
            angular: 0.0,
        },
    ));
}

fn launcher_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut launchers: Query<(&Launcher, &Transform, &mut ConstantForce), With<Launcher>>,
) {
    // Force increase in Newtons to pull launcher down
    // TODO why does this need to be so high? In real life a pinball launcher spring is much weaker yet
    //   it spings back fast enough.
    //   A hand can pull between 50 and 100 N easily, but we have to pull 20000 N here to get a good effect.
    // TODO why is the ball pulled into the launcher while we pull it down?
    const PULL_FORCE: f32 = 200.0;
    // Maximum pull distance in meters
    const MAX_PULL_DISTANCE: f32 = 0.08;

    for (launcher, transform, mut constant_force) in launchers.iter_mut() {
        let current_offset = transform.translation.y - launcher.start_point.y;

        if keyboard_input.pressed(KeyCode::Enter) {
            // Apply downward force if not at max stretch
            if current_offset > -crate::PIXELS_PER_METER * MAX_PULL_DISTANCE {
                constant_force.y -= PULL_FORCE;
                println!("Pulling launcher down: force.y = {}", constant_force.y);
            }
        } else {
            // Release: clear force and let the spring push it back
            constant_force.y = 0.0;
        }
    }
}
