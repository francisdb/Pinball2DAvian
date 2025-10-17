use avian2d::math::Vector;
use avian2d::PhysicsPlugins;
use bevy::{
    prelude::*,
    window::{PresentMode},
};
use bevy_prototype_lyon::prelude::*;
use avian2d::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod ball;
use ball::*;

mod flippers;
use flippers::*;

mod walls;
use walls::*;

mod launcher;
use launcher::*;

mod pins;
use pins::*;

pub const PIXELS_PER_METER: f32 = 492.3;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pinball2d".into(),
                resolution: (360, 640).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(WallsPlugin)
        .add_plugins(LauncherPlugin)
        .add_plugins(FlippersPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(PinsPlugin)
        .add_plugins(ShapePlugin)
        .add_systems(Startup,setup)
        .add_plugins(PhysicsPlugins::default().with_length_unit(PIXELS_PER_METER))
        .insert_resource(Gravity(Vector::NEG_Y * 520.0))
        //.insert_resource(Gravity(Vector::NEG_Y * 9.81 * 100.0))
        .insert_resource(SubstepCount(50))
        // .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
        //     PIXELS_PER_METER,
        // ))
        .add_systems(Update, exit_on_escape)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn exit_on_escape(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: MessageWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}