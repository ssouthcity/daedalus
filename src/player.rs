use avian2d::prelude::*;
use bevy::prelude::*;

use crate::input::{InteractInput, MovementInput};

const PLAYER_WALK_SPEED: f32 = 96.0;
const PLAYER_RUN_SPEED: f32 = 128.0;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();

    app.add_systems(Update, (player_movement, player_interact));
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Player;

fn player_movement(
    query: Query<&mut LinearVelocity, With<Player>>,
    movement_input: Res<MovementInput>,
) {
    let speed = if movement_input.is_sprinting {
        PLAYER_RUN_SPEED
    } else {
        PLAYER_WALK_SPEED
    };

    for mut velocity in query {
        velocity.0 = movement_input.axis * speed;
    }
}

fn player_interact(mut events: EventReader<InteractInput>) {
    for _ in events.read() {
        println!("interact!");
    }
}
