use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod fields;
mod player;
mod potion;
mod wall;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin::default());

    app.insert_resource(LevelSelection::index(0));

    app.add_systems(Startup, spawn_level);

    app.register_ldtk_entity::<player::PlayerEntity>("Player")
        .add_systems(Update, player::process_player);

    app.register_ldtk_entity::<potion::PotionEntity>("Health_Potion");

    app.register_ldtk_int_cell::<wall::WallBundle>(1);
}

fn spawn_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Ldtk World"),
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("stages/maze.ldtk").into(),
            ..default()
        },
    ));
}
