use crate::player;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

const COLLECTIBLE_COLLECT_THRESHOLD: f32 = 12.0;
const COLLECTIBLE_FLOAT_THRESHOLD: f32 = 32.0;
const COLLECTIBLE_FLOAT_SPEED: f32 = 2.0;

pub struct CollectiblePlugin;

impl Plugin for CollectiblePlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PotionBundle>("Health_Potion")
            .add_systems(
                Update,
                (
                    float_towards_player_system,
                    float_towards_target_system,
                    collection_system,
                )
                    .chain(),
            );
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
struct Collectible;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Component)]
struct FloatTowards(Entity);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
enum Potion {
    #[default]
    Health,
}

#[derive(Clone, Default, Debug, Bundle, LdtkEntity)]
struct PotionBundle {
    collectible: Collectible,
    potion: Potion,
    #[sprite_sheet]
    sprite_sheet: Sprite,
}

fn float_towards_player_system(
    mut commands: Commands,
    player: Single<(Entity, &Transform), With<player::Player>>,
    collectibles: Query<(Entity, &Transform), (With<Collectible>, Without<player::Player>)>,
) {
    let (player_entity, player_transform) = (player.0, player.1);

    for (collectible_entity, collectible_transform) in collectibles.iter() {
        let distance = player_transform
            .translation
            .distance(collectible_transform.translation);

        if distance <= COLLECTIBLE_FLOAT_THRESHOLD {
            commands
                .entity(collectible_entity)
                .insert(FloatTowards(player_entity));
        }
    }
}

fn float_towards_target_system(
    mut collectibles: Query<(&mut Transform, &FloatTowards), With<Collectible>>,
    transforms: Query<&Transform, Without<Collectible>>,
    time: Res<Time>,
) {
    for (mut collectible_transform, collectible_float_towards) in collectibles.iter_mut() {
        let target_transform = transforms.get(collectible_float_towards.0).expect("");
        let Vec3 { x, y, .. } = target_transform.translation;
        let direction = Vec3::new(x, y, collectible_transform.translation.z);

        collectible_transform.translation.smooth_nudge(
            &direction,
            COLLECTIBLE_FLOAT_SPEED,
            time.delta_secs(),
        );
    }
}

fn collection_system(
    mut commands: Commands,
    player_transform: Single<&Transform, With<player::Player>>,
    collectibles: Query<(Entity, &Transform), (With<Collectible>, Without<player::Player>)>,
) {
    for (collectible_entity, collectible_transform) in collectibles.iter() {
        let distance = player_transform
            .translation
            .distance(collectible_transform.translation);

        if distance <= COLLECTIBLE_COLLECT_THRESHOLD {
            // handle collect
            println!("collected!");

            commands.entity(collectible_entity).despawn();
        }
    }
}
