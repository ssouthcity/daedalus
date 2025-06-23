use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{audio::sound_effect, collectible::Collector, health::HealEvent};

const HEAL_AMOUNT: i32 = 20;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<PotionAssets>();

    app.register_ldtk_entity::<PotionEntity>("Health_Potion");

    app.add_systems(Startup, load_potion_assets);
    app.add_systems(Update, process_potion);
    app.add_systems(Update, super::fix_z_coordinate::<Potion>);
}

#[derive(Resource, Default)]
struct PotionAssets {
    collect_sound: Handle<AudioSource>,
}

#[derive(Component, Default, Clone, Debug)]
struct Potion;

#[derive(Clone, Default, Debug, Bundle, LdtkEntity)]
struct PotionEntity {
    potion: Potion,
    #[sprite_sheet]
    sprite_sheet: Sprite,
}

fn load_potion_assets(asset_server: Res<AssetServer>, mut potion_assets: ResMut<PotionAssets>) {
    potion_assets.collect_sound = asset_server.load("sound_effects/potion_collect.ogg");
}

fn process_potion(mut commands: Commands, entity_query: Query<Entity, Added<Potion>>) {
    for entity in entity_query {
        commands
            .entity(entity)
            .insert((
                RigidBody::Static,
                Collider::circle(8.0),
                Sensor,
                CollisionEventsEnabled,
            ))
            .observe(collect_potion);
    }
}

fn collect_potion(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    mut heal_events: EventWriter<HealEvent>,
    collectors: Query<&ColliderOf, With<Collector>>,
    potion_assets: Res<PotionAssets>,
) {
    if let Ok(collider_of) = collectors.get(trigger.collider) {
        commands.entity(trigger.target()).despawn();

        commands.spawn(sound_effect(potion_assets.collect_sound.clone()));

        heal_events.write(HealEvent {
            entity: collider_of.body,
            amount: HEAL_AMOUNT,
        });
    }
}
