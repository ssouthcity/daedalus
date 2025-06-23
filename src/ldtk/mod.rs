use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod fields;
mod level;
mod player;
mod potion;
mod wall;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin::default());

    app.insert_resource(LevelSelection::index(0));

    app.insert_resource(LdtkSettings {
        level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
            load_level_neighbors: true,
        },
        ..default()
    });

    app.add_plugins((level::plugin, player::plugin, potion::plugin, wall::plugin));
}

pub fn fix_z_coordinate<C: Component>(transforms: Query<&mut Transform, With<C>>) {
    for mut transform in transforms {
        transform.translation.z = 0.0;
    }
}

pub fn spawn_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Ldtk World"),
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("stages/maze.ldtk").into(),
            ..default()
        },
    ));
}
