use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{camera::CameraTarget, health::Health, player::Player};

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerEntity {
    pub player: Player,
    pub camera_target: CameraTarget,
    #[sprite_sheet(no_grid)]
    pub sprite_sheet: Sprite,
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    pub health: Health,
}

pub fn process_player(mut commands: Commands, entity_query: Query<Entity, Added<Player>>) {
    for entity in entity_query.iter() {
        commands.entity(entity).insert((
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            LinearVelocity::ZERO,
        ));

        commands.spawn((
            ChildOf(entity),
            Transform::from_xyz(0.0, -8.0, 0.0),
            Collider::rectangle(4.0, 4.0),
        ));
    }
}
