use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub struct PinsPlugin;

impl Plugin for PinsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pins)
            .add_systems(Update, handle_pin_events)
            .add_systems(Update, respawn_pin_to_toggle_color);
    }
}

// TODO rename to Bumper
#[derive(Component)]
struct Pin {
    timestamp_last_hit: f64,
    position: Vec2,
}

fn spawn_pins(mut commands: Commands) {
    let pins_pos: [Vec2; 3] = [
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

    for (i, pin_pos) in pins_pos.iter().enumerate() {
        spawn_single_pin(format!("Pin{}", i), &mut commands, *pin_pos, None);
    }
}

fn spawn_single_pin(
    name: String,
    commands: &mut Commands,
    position: Vec2,
    timestamp_last_hit: Option<f64>,
) {
    let shape_pin = shapes::Circle {
        radius: crate::PIXELS_PER_METER * 0.05,
        center: Vec2::ZERO,
    };
    //let bevy_shape = Circle::new(shape_pin.radius);

    let temp_timestamp_last_hit = timestamp_last_hit.unwrap_or(0.0);

    let mut color = bevy::color::palettes::css::GREEN;
    if temp_timestamp_last_hit == 0.0 {
        color = bevy::color::palettes::css::TEAL;
    }

    commands.spawn((
        Name::from(name),
        ShapeBuilder::with(&shape_pin)
            .fill(Color::BLACK)
            .stroke((color, 2.0))
            .build(),
        Transform::from_xyz(position.x, position.y, 0.0),
        RigidBody::Static,
        //MassPropertiesBundle::from_shape(&bevy_shape, 1.0),
        Restitution::new(0.7),
        Collider::circle(shape_pin.radius),
        Pin {
            timestamp_last_hit: temp_timestamp_last_hit,
            position,
        },
    ));
}

fn respawn_pin_to_toggle_color(
    mut query: Query<(Entity, &Name, &Pin), With<Pin>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, name, pin) in query.iter_mut() {
        let diff = time.elapsed_secs_f64() - pin.timestamp_last_hit;
        if pin.timestamp_last_hit > 0.0 && diff > 1.0 {
            //Color have been toggled for more than a second so respawn
            let pos = pin.position;
            commands.entity(entity).despawn();
            spawn_single_pin(name.into(), &mut commands, pos, None);
        }
    }
}

fn handle_pin_events(
    query: Query<(Entity, &Name, &Pin), With<Pin>>,
    time: Res<Time>,
    mut contact_events: MessageReader<CollisionStart>,
    mut commands: Commands,
) {
    for contact_event in contact_events.read() {
        for (entity, name, pin) in query.iter() {
            if let (Some(h1), Some(h2)) = (contact_event.body1, contact_event.body2)
                && (h1 == entity || h2 == entity)
            {
                // TODO we should add some sound effect here
                // TODO we should have the bumper expand a bit and then go back to normal size

                // TODO we respawn to change color, there should be a better way?
                //   an other option would be using bevy_light_2d to light up the pin

                let pos = pin.position;
                let timestamp_last_hit = time.elapsed_secs_f64();
                commands.entity(entity).despawn();
                spawn_single_pin(name.into(), &mut commands, pos, Some(timestamp_last_hit));
            }
        }
    }
}
