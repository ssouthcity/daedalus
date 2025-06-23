use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::audio::TransitionBackgroundMusic;
use crate::camera::CameraTarget;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, level_selection_follow_camera);
}

fn level_selection_follow_camera(
    camera_target_transform: Single<&GlobalTransform, With<CameraTarget>>,
    levels: Query<(&LevelIid, &GlobalTransform)>,
    ldtk_project: Single<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut level_selection: ResMut<LevelSelection>,
    mut transition_music: EventWriter<TransitionBackgroundMusic>,
    asset_server: Res<AssetServer>,
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

        if let Some(FieldValue::FilePath(Some(path))) = level
            .field_instances
            .iter()
            .find(|f| f.identifier == "Background_Music")
            .map(|f| f.value.clone())
        {
            let music = asset_server.load(format!("stages/{}", path));
            transition_music.write(TransitionBackgroundMusic(music));
        }
    }
}
