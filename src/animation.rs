use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(AsepriteUltraPlugin);

    app.register_type::<DespawnOnAnimationFinish>();
    app.add_systems(
        Update,
        despawn_after_animation.run_if(on_event::<AnimationEvents>),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DespawnOnAnimationFinish;

fn despawn_after_animation(
    mut commands: Commands,
    mut events: EventReader<AnimationEvents>,
    despawns: Query<&DespawnOnAnimationFinish>,
) {
    for event in events.read() {
        if let AnimationEvents::Finished(entity) = event {
            if despawns.contains(*entity) {
                commands.entity(*entity).despawn();
            }
        }
    }
}
