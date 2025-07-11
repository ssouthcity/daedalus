use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    animation::DespawnOnAnimationFinish,
    assets::LoadResource,
    audio::sound_effect,
    gameplay::behavior::{
        collectible::Collector,
        health::Heal,
        inventory::{InventoryIcon, ItemOf, OnUseItem},
    },
};

const HEAL_AMOUNT: i32 = 20;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Potion>();

    app.load_resource::<PotionAssets>();

    app.register_ldtk_entity::<PotionEntity>("Health_Potion");

    app.add_observer(process_potion);

    app.add_systems(Update, super::fix_z_coordinate::<Potion>);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct PotionAssets {
    #[dependency]
    collect_sound: Handle<AudioSource>,
    #[dependency]
    inventory_icon: Handle<Image>,
    #[dependency]
    use_animation: Handle<Aseprite>,
}

impl FromWorld for PotionAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            collect_sound: asset_server.load("sound_effects/potion_collect.ogg"),
            inventory_icon: asset_server.load("potion-icon.png"),
            use_animation: asset_server.load("potion-effect.aseprite"),
        }
    }
}

#[derive(Component, Default, Clone, Debug, Reflect)]
#[reflect(Component)]
struct Potion;

#[derive(Clone, Default, Debug, Bundle, LdtkEntity)]
struct PotionEntity {
    potion: Potion,

    #[sprite_sheet]
    sprite_sheet: Sprite,
}

fn process_potion(trigger: Trigger<OnAdd, Potion>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert((
            RigidBody::Static,
            Collider::circle(8.0),
            Sensor,
            CollisionEventsEnabled,
        ))
        .observe(collect_potion);
}

fn collect_potion(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    collectors: Query<&ColliderOf, With<Collector>>,
    potion_assets: Res<PotionAssets>,
) {
    if let Ok(collider_of) = collectors.get(trigger.collider) {
        commands.entity(trigger.target()).despawn();

        commands
            .spawn((
                Name::new("Health Potion"),
                ItemOf(collider_of.body),
                InventoryIcon(potion_assets.inventory_icon.clone()),
            ))
            .observe(drink_potion);
    }
}

fn drink_potion(
    trigger: Trigger<OnUseItem>,
    mut commands: Commands,
    potion_assets: Res<PotionAssets>,
    items: Query<&ItemOf>,
) {
    commands.entity(trigger.target()).despawn();

    commands.spawn(sound_effect(potion_assets.collect_sound.clone()));

    if let Ok(item_of) = items.get(trigger.target()) {
        commands.spawn((
            Name::new("Potion VFX"),
            ChildOf(item_of.0),
            Sprite::default(),
            AseAnimation {
                aseprite: potion_assets.use_animation.clone(),
                animation: Animation::default().with_repeat(AnimationRepeat::Count(0)),
            },
            DespawnOnAnimationFinish,
        ));

        commands.trigger_targets(Heal(HEAL_AMOUNT), item_of.0);
    }
}
