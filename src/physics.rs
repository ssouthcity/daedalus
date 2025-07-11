use crate::pause::Paused;
use avian2d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default().with_length_unit(32.0));

    app.insert_resource(Gravity::ZERO);

    app.add_systems(OnEnter(Paused(true)), pause_physics);
    app.add_systems(OnEnter(Paused(false)), unpause_physics);
}

fn pause_physics(mut time: ResMut<Time<Physics>>) {
    time.pause();
}

fn unpause_physics(mut time: ResMut<Time<Physics>>) {
    time.unpause();
}
