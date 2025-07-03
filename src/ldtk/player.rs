use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    assets::LoadResource, camera::CameraTarget, collectible::Collector, health::Health,
    player::Player,
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();

    app.register_ldtk_entity::<PlayerEntity>("Player");

    app.add_observer(process_player);

    app.add_systems(Update, update_player_animation);
}

#[derive(Asset, Resource, Reflect, Clone)]
#[reflect(Resource)]
struct PlayerAssets {
    #[dependency]
    aseprite_file: Handle<Aseprite>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            aseprite_file: asset_server.load("player.aseprite"),
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
struct PlayerEntity {
    player: Player,
    camera_target: CameraTarget,
    #[worldly]
    worldly: Worldly,
    #[from_entity_instance]
    health: Health,
}

fn process_player(
    trigger: Trigger<OnAdd, Player>,
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
) {
    commands
        .entity(trigger.target())
        .insert((
            AseAnimation {
                aseprite: player_assets.aseprite_file.clone(),
                animation: Animation::tag("idle"),
            },
            Sprite::default(),
        ))
        .insert((
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            LinearVelocity::ZERO,
        ));

    commands.spawn((
        Name::new("Collider"),
        ChildOf(trigger.target()),
        Transform::from_xyz(0.0, -8.0, 0.0),
        Collider::rectangle(4.0, 4.0),
        Collector,
    ));
}

fn update_player_animation(
    mut player: Single<(&mut AseAnimation, &mut Sprite, &LinearVelocity), With<Player>>,
) {
    if player.2.length() <= 0.0 {
        player.0.animation.play_loop("idle");
    } else {
        player.0.animation.play_loop("walk");
    }

    if player.2.x.abs() > 0.0 {
        player.1.flip_x = player.2.x.is_sign_negative();
    }
}
