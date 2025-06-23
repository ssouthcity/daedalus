use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{camera::CameraTarget, collectible::Collector, health::Health, player::Player};

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<PlayerEntity>("Player");

    app.add_systems(Update, process_player);
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
struct PlayerEntity {
    player: Player,
    camera_target: CameraTarget,
    #[sprite_sheet(no_grid)]
    sprite_sheet: Sprite,
    #[worldly]
    worldly: Worldly,
    #[from_entity_instance]
    health: Health,
}

fn process_player(mut commands: Commands, entity_query: Query<Entity, Added<Player>>) {
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
            Collector,
        ));
    }
}
