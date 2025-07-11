use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::assets::LoadResource;
use crate::audio::TransitionBackgroundMusic;
use crate::camera::CameraTarget;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<BackgroundMusicAssets>();
    app.load_resource::<BackgroundMusicAssets>();

    app.add_systems(Update, level_selection_follow_camera);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct BackgroundMusicAssets {
    #[dependency]
    overworld: Handle<AudioSource>,
    #[dependency]
    inside: Handle<AudioSource>,
}

impl FromWorld for BackgroundMusicAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            overworld: asset_server.load("music/overworld.ogg"),
            inside: asset_server.load("music/inside.ogg"),
        }
    }
}

fn level_selection_follow_camera(
    camera_target_transform: Single<&GlobalTransform, With<CameraTarget>>,
    levels: Query<(&LevelIid, &GlobalTransform)>,
    ldtk_project: Single<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut level_selection: ResMut<LevelSelection>,
    mut transition_music: EventWriter<TransitionBackgroundMusic>,
    background_music_assets: Res<BackgroundMusicAssets>,
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

        if !level_bounds.contains(camera_target_transform.translation().truncate()) {
            continue;
        }

        if let LevelSelection::Iid(current_id) = level_selection.clone() {
            if current_id.as_str() == level.iid {
                continue;
            }
        }

        *level_selection = LevelSelection::Iid(level_iid.clone());

        if let Ok(value) = level.get_enum_field("BackgroundMusicTrack") {
            let track = match value.as_str() {
                "Overworld" => background_music_assets.overworld.clone_weak(),
                "Inside" => background_music_assets.inside.clone_weak(),
                _ => unreachable!("unrecognized background music track"),
            };

            transition_music.write(TransitionBackgroundMusic(track));
        }
    }
}
