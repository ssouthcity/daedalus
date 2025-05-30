use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::camera::CameraTarget;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerEntity>("Player")
            .add_systems(Update, (move_player, fix_translation).chain());
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Bundle, LdtkEntity)]
pub struct PlayerEntity {
    #[sprite("sprites/player.png")]
    pub sprite: Sprite,
    pub player: Player,
    pub camera_target: CameraTarget,

    pub rigid_body: RigidBody,
    pub locked_axes: LockedAxes,
    pub collider: Collider,
    pub linear_velocity: LinearVelocity,
    pub angular_velocity: AngularVelocity,
    pub friction: Friction,
}

impl Default for PlayerEntity {
    fn default() -> Self {
        Self {
            sprite: Sprite::default(),
            player: Player::default(),
            camera_target: CameraTarget,

            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collider: Collider::rectangle(16.0, 16.0),
            linear_velocity: LinearVelocity::ZERO,
            angular_velocity: AngularVelocity::ZERO,
            friction: Friction::ZERO,
        }
    }
}

pub fn fix_translation(mut query: Query<&mut Transform, With<Player>>) {
    for mut transform in query.iter_mut() {
        transform.translation.z = 3.0;
    }
}

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

    for mut velocity in query.iter_mut() {
        velocity.0 = axis * 128.0;
    }
}
