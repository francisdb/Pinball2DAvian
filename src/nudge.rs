//! Table nudge.
//!
//! Modelled as a virtual "table on 4 springs": a tap shoves the table and it
//! rings back to centre (damped). Instead of physically moving all the walls,
//! we work in the table's reference frame:
//!   - physics: the table's acceleration becomes a fictitious force on every
//!     dynamic body, applied through the global gravity vector. One-sided wall
//!     contacts rectify the out-and-back swing into a net kick only for balls
//!     resting against something; a free ball nets to zero.
//!   - visual: the camera is offset by the table's displacement so the playfield
//!     appears to jolt (purely cosmetic, no effect on physics).

use avian2d::math::Vector;
use avian2d::prelude::*;
use bevy::prelude::*;
use core::f32::consts::TAU;

// How fast the "table on springs" rings back to centre.
const NUDGE_FREQUENCY_HZ: f32 = 8.0;
// < 1.0 wobbles then settles; ~0.5 gives a visible shake.
const NUDGE_DAMPING_RATIO: f32 = 0.5;
// Strength of a single shove, in px/s^2.
const NUDGE_PUSH_ACCEL: f32 = 12000.0;
// How long one tap shoves the table, in seconds.
const NUDGE_PUSH_DURATION: f32 = 0.04;

pub struct NudgePlugin;

impl Plugin for NudgePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Nudge>()
            .add_systems(Startup, capture_base_gravity)
            .add_systems(Update, (nudge_input, apply_nudge).chain());
    }
}

#[derive(Resource, Default)]
struct Nudge {
    base_gravity: Vector,
    /// Virtual table displacement and velocity (px, px/s).
    pos: Vec2,
    vel: Vec2,
    /// Current shove acceleration applied to the table and how long it lasts.
    push: Vec2,
    push_timer: f32,
}

fn capture_base_gravity(mut nudge: ResMut<Nudge>, gravity: Res<Gravity>) {
    nudge.base_gravity = gravity.0;
}

fn nudge_input(keyboard: Res<ButtonInput<KeyCode>>, mut nudge: ResMut<Nudge>) {
    // Keys mirror Visual Pinball defaults: Z = left nudge, / = right nudge,
    // Space = center nudge. The value is the direction we want the BALL to lurch;
    // the table is shoved the opposite way (real nudge: shove the table left and
    // the ball drifts right). Flip a sign here if a direction feels backwards.
    let mut ball_dir = Vec2::ZERO;
    if keyboard.just_pressed(KeyCode::KeyZ) {
        ball_dir.x -= 1.0; // left nudge
    }
    if keyboard.just_pressed(KeyCode::Slash) {
        ball_dir.x += 1.0; // right nudge
    }
    if keyboard.just_pressed(KeyCode::Space) {
        ball_dir.y -= 1.0; // center nudge: jolt the table up (ball lurches down)
    }

    if ball_dir != Vec2::ZERO {
        nudge.push = -ball_dir.normalize() * NUDGE_PUSH_ACCEL;
        nudge.push_timer = NUDGE_PUSH_DURATION;
    }
}

fn apply_nudge(
    time: Res<Time>,
    mut nudge: ResMut<Nudge>,
    mut gravity: ResMut<Gravity>,
    mut cameras: Query<&mut Transform, With<Camera2d>>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    let omega = TAU * NUDGE_FREQUENCY_HZ;
    let stiffness = omega * omega;
    let damping = 2.0 * NUDGE_DAMPING_RATIO * omega;

    let push = if nudge.push_timer > 0.0 {
        nudge.push_timer -= dt;
        nudge.push
    } else {
        Vec2::ZERO
    };

    // Damped spring: table_accel = shove - k*x - c*v.
    let table_accel = push - stiffness * nudge.pos - damping * nudge.vel;
    let vel = nudge.vel + table_accel * dt;
    nudge.vel = vel;
    nudge.pos += vel * dt;

    // Fictitious acceleration felt by every body in the table's frame.
    gravity.0 = nudge.base_gravity - table_accel;

    // Fake the table jolting by offsetting the camera (visual only).
    if let Ok(mut camera) = cameras.single_mut() {
        camera.translation.x = -nudge.pos.x;
        camera.translation.y = -nudge.pos.y;
    }
}
