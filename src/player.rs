use avian2d::prelude::*;
use bevy::prelude::*;

const PLAYER_WALK_SPEED: f32 = 96.0;
const PLAYER_RUN_SPEED: f32 = 128.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

pub fn move_player(
    mut query: Query<&mut LinearVelocity, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut axis = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        axis += Vec2::Y;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        axis -= Vec2::X;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        axis -= Vec2::Y;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        axis += Vec2::X;
    }

    axis = axis.normalize();

    let speed = if keyboard_input.pressed(KeyCode::ShiftLeft) {
        PLAYER_RUN_SPEED
    } else {
        PLAYER_WALK_SPEED
    };

    for mut velocity in query.iter_mut() {
        velocity.0 = axis * speed;
    }
}
