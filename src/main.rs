use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod camera;
mod collectible;
#[cfg(feature = "dev")]
mod debug;
mod entity;
mod field;
mod hud;
mod player;
mod wall;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Daedalus".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
        );

        app.add_plugins((
            // avian2d
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            // bevy_ecs_ldtk
            LdtkPlugin::default(),
        ));

        app.add_plugins((
            camera::CameraPlugin,
            collectible::CollectiblePlugin,
            #[cfg(feature = "dev")]
            debug::DebugPlugin,
            entity::LdtkEntityPlugin,
            hud::HudPlugin,
            player::PlayerPlugin,
            wall::WallPlugin,
        ));

        app.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
            .insert_resource(Gravity(Vec2::ZERO))
            .insert_resource(LevelSelection::index(0))
            .add_systems(Startup, setup);
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        IsDefaultUiCamera,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(1280.0 / 4.0, 720.0 / 4.0, 0.0),
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("stages/maze.ldtk").into(),
        ..default()
    });
}
