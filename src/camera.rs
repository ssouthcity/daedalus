use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, follow_target);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct CameraTarget;

const CAMERA_DECAY_RATE: f32 = 2.0;

pub fn follow_target(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<CameraTarget>)>,
    target: Single<&Transform, (With<CameraTarget>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Vec3 { x, y, .. } = target.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}
