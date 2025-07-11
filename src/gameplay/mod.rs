use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::assets::LoadResource;

mod actors;
mod behavior;
mod hud;
mod level;
mod movement;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin::default());

    app.insert_resource(LevelSelection::index(0));

    app.insert_resource(LdtkSettings {
        level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
            load_level_neighbors: true,
        },
        ..default()
    });

    app.register_type::<LdtkProjectAssets>();
    app.load_resource::<LdtkProjectAssets>();

    app.add_plugins((
        actors::plugin,
        behavior::plugin,
        hud::plugin,
        level::plugin,
        movement::plugin,
    ));
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LdtkProjectAssets {
    #[dependency]
    project: Handle<LdtkProject>,
}

impl FromWorld for LdtkProjectAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            project: asset_server.load("stages/maze.ldtk"),
        }
    }
}

pub fn spawn_world(mut commands: Commands, project_asset: Res<LdtkProjectAssets>) {
    commands.spawn((
        Name::new("Ldtk World"),
        LdtkWorldBundle {
            ldtk_handle: project_asset.project.clone().into(),
            ..default()
        },
    ));
}
