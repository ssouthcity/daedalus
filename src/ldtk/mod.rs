use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::player::Player;

mod fields;
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

    app.add_systems(Startup, spawn_world)
        .add_systems(Update, level_selection_follow_player);

    app.register_ldtk_entity::<player::PlayerEntity>("Player")
        .add_systems(Update, player::process_player);

    app.register_ldtk_entity::<potion::PotionEntity>("Health_Potion");

    app.register_ldtk_int_cell::<wall::WallBundle>(1);
}

fn spawn_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Ldtk World"),
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("stages/maze.ldtk").into(),
            ..default()
        },
    ));
}

fn level_selection_follow_player(
    player_transform: Single<&GlobalTransform, With<Player>>,
    levels: Query<(&LevelIid, &GlobalTransform)>,
    ldtk_project: Single<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut level_selection: ResMut<LevelSelection>,
) {
    let ldtk_project = ldtk_project_assets
        .get(ldtk_project.into_inner())
        .expect("ldtk project should be loaded before player is spawned");

    for (level_iid, level_transform) in levels.iter() {
        let level = ldtk_project
            .get_raw_level_by_iid(level_iid.get())
            .expect("level should exist in only project");

        let Vec3 { x, y, .. } = level_transform.translation();

        let level_bounds = Rect::new(x, y, x + level.px_wid as f32, y + level.px_hei as f32);

        if level_bounds.contains(player_transform.translation().truncate()) {
            *level_selection = LevelSelection::Iid(level_iid.clone());
        }
    }
}
