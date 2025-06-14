use bevy::prelude::*;

const CAMERA_DECAY_RATE: f32 = 2.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_main_camera);
    app.add_systems(Update, follow_target);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct CameraTarget;

fn setup_main_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn follow_target(
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
