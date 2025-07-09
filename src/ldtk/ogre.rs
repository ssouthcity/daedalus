use avian2d::prelude::*;
use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{assets::LoadResource, health::Health, patrol::Patrol};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<OgreAssets>();

    app.register_type::<Ogre>();

    app.register_ldtk_entity::<OgreEntity>("Ogre");

    app.add_systems(Update, update_ogre_animation);
}

#[derive(Asset, Resource, Reflect, Clone)]
#[reflect(Resource)]
struct OgreAssets {
    #[dependency]
    aseprite: Handle<Aseprite>,
}

impl FromWorld for OgreAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            aseprite: asset_server.load("ogre.aseprite"),
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(RigidBody::Dynamic, LinearVelocity::ZERO)]
#[component(on_add = on_ogre_add)]
pub struct Ogre;

#[derive(Bundle, Default, LdtkEntity)]
struct OgreEntity {
    ogre: Ogre,

    #[from_entity_instance]
    health: Health,

    #[from_entity_instance]
    patrol: Patrol,
}

fn on_ogre_add(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let ogre_assets = world.resource::<OgreAssets>();

    let aseprite = ogre_assets.aseprite.clone();

    world.commands().entity(entity).insert((
        Sprite::default(),
        AseAnimation {
            aseprite,
            ..default()
        },
    ));
}

fn update_ogre_animation(
    ogres: Query<(&mut AseAnimation, &mut Sprite, &LinearVelocity), With<Ogre>>,
) {
    for (mut ase_animation, mut sprite, linear_velocity) in ogres {
        if linear_velocity.length() <= 0.0 {
            ase_animation.animation.play_loop("idle");
        } else {
            ase_animation.animation.play_loop("walk");
        }

        if linear_velocity.x.abs() > 0.0 {
            sprite.flip_x = linear_velocity.x.is_sign_negative();
        }
    }
}
